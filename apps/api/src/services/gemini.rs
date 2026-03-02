/// Gemini multimodal API integration.
///
/// Sends audio bytes + user prompt to Gemini 2.5 Flash and returns
/// a structured JSON object of Vital synthesizer parameter values.
///
/// Key technique (from spike): provide a template JSON and ask Gemini to
/// fill in values. This prevents verbose output and guarantees parseable JSON.

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde_json::{json, Value};
use std::time::Duration;

use crate::config::GEMINI_MAX_RETRIES;

const GEMINI_API_BASE: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";

// ── System Prompt ────────────────────────────────────────────────────────────
//
// This prompt uses the template approach confirmed in Spike C:
// Providing a filled-in JSON template and asking Gemini to modify the values
// produces compact, parseable output without markdown fences or truncation.

const SYSTEM_PROMPT: &str = r#"You are an expert synthesizer sound designer with deep knowledge of subtractive, FM, and wavetable synthesis. Your task is to analyze audio and determine which synthesizer parameters would best recreate or complement that sound in the Vital synthesizer.

TASK:
Listen carefully to the provided audio. Then fill in the JSON template below with parameter values that would produce a synthesizer sound matching or complementing what you hear. If a text description is also provided, prioritize it to refine the sound type (e.g., "heavy bass", "bright lead", "ethereal pad").

TIMESTAMP CONTEXT:
If the user specifies a time range (e.g., "focus on 0:30 to 0:45"), analyze ONLY the sound during that window. Ignore the rest of the audio.

PARAMETER RULES:
- ALL values must be normalized between 0.0 and 1.0 (inclusive).
- 0.0 = minimum, 1.0 = maximum for each parameter.
- Oscillator wave_frame: 0.0=sine, 0.25=triangle, 0.5=sawtooth, 0.75=square, 1.0=noise.
- Envelope attack/decay/release: 0.0=instant, 1.0=very slow (4 seconds).
- Envelope sustain: 0.0=silent, 1.0=full volume held.
- Filter cutoff: 0.0=fully closed (dark/muffled), 1.0=fully open (bright).
- LFO frequency: 0.0=very slow (sub-Hz), 1.0=very fast (audio rate).
- Mix parameters (reverb_mix, delay_mix, etc.): 0.0=dry, 1.0=fully wet.
- Level parameters: 0.0=silent, 1.0=maximum.

SOUND DESIGN GUIDELINES BY TYPE:
- Heavy bass: osc_1_wave_frame ~0.5-0.75 (saw/square), filter_1_cutoff ~0.3-0.5, filter_1_resonance ~0.3-0.6, env_1_attack ~0.0, env_1_decay ~0.3-0.5, env_1_sustain ~0.5-0.8, reverb_mix ~0.0-0.1
- Bright lead: osc_1_wave_frame ~0.5 (saw), filter_1_cutoff ~0.7-0.9, env_1_attack ~0.0-0.1, env_1_sustain ~0.7-1.0, reverb_mix ~0.1-0.3
- Ethereal pad: osc_1_wave_frame ~0.0-0.25 (sine/triangle), filter_1_cutoff ~0.4-0.7, env_1_attack ~0.4-0.8, env_1_release ~0.5-0.9, reverb_mix ~0.4-0.8, chorus_mix ~0.2-0.5
- Pluck: env_1_attack ~0.0, env_1_decay ~0.1-0.3, env_1_sustain ~0.0-0.2, env_1_release ~0.1-0.3
- FM bass (Skrillex-style): osc_1_wave_frame ~0.5-0.75, filter_1_cutoff ~0.2-0.4, filter_1_resonance ~0.5-0.8, distortion_amount ~0.3-0.6, lfo_1_amount ~0.3-0.6

OUTPUT FORMAT — CRITICAL:
Output ONLY this exact JSON structure with your values filled in. No other text, no explanation, no markdown, no code fences. Just the raw JSON:

{"osc_1_wave_frame":0.5,"osc_1_level":0.8,"osc_1_pan":0.5,"osc_1_tune":0.5,"osc_1_transpose":0.5,"osc_2_wave_frame":0.25,"osc_2_level":0.0,"osc_2_pan":0.5,"osc_2_tune":0.5,"osc_2_transpose":0.5,"filter_1_cutoff":0.6,"filter_1_resonance":0.3,"filter_1_drive":0.2,"filter_1_blend":0.5,"env_1_attack":0.02,"env_1_decay":0.3,"env_1_sustain":0.7,"env_1_release":0.4,"env_2_attack":0.0,"env_2_decay":0.2,"env_2_sustain":0.0,"env_2_release":0.2,"lfo_1_frequency":0.3,"lfo_1_amount":0.0,"lfo_2_frequency":0.2,"lfo_2_amount":0.0,"reverb_mix":0.15,"reverb_decay_time":0.4,"reverb_size":0.5,"delay_mix":0.0,"delay_frequency":0.5,"distortion_amount":0.0,"distortion_mix":0.5,"chorus_mix":0.0,"chorus_amount":0.3,"phaser_mix":0.0,"phaser_amount":0.3}"#;

// ── Public API ───────────────────────────────────────────────────────────────

/// Send audio bytes + prompt to Gemini and receive Vital parameter JSON.
///
/// # Arguments
/// * `api_key` - Gemini API key
/// * `audio_data` - Raw MP3 audio bytes
/// * `prompt` - User's text description of the desired sound
/// * `start_time` - Optional start timestamp in seconds (for focus window)
/// * `end_time` - Optional end timestamp in seconds (for focus window)
/// * `timeout_secs` - Request timeout in seconds
///
/// # Returns
/// A `serde_json::Value` object with Vital parameter names → values (0.0–1.0).
pub async fn generate_preset_json(
    api_key: &str,
    audio_data: &[u8],
    prompt: &str,
    start_time: Option<f64>,
    end_time: Option<f64>,
    timeout_secs: u64,
) -> Result<Value> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()?;

    let url = format!("{}?key={}", GEMINI_API_BASE, api_key);

    // Attempt once, retry up to GEMINI_MAX_RETRIES times on JSON parse failure.
    for attempt in 0..=GEMINI_MAX_RETRIES {
        let body = build_request_body(audio_data, prompt, start_time, end_time, attempt);

        tracing::info!(
            "Gemini request attempt {}: prompt={:?}, audio_bytes={}",
            attempt + 1,
            prompt,
            audio_data.len()
        );

        let resp = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            if attempt == 0 {
                tracing::warn!("Gemini returned {} on attempt 1, retrying: {}", status, text);
                continue;
            }
            return Err(anyhow!("Gemini API error {}: {}", status, text));
        }

        let resp_json: Value = resp.json().await?;
        match extract_and_parse_content(&resp_json) {
            Ok(params) => {
                tracing::info!("Gemini returned {} parameters", params.as_object().map(|m| m.len()).unwrap_or(0));
                return Ok(params);
            }
            Err(e) => {
                if attempt == 0 {
                    tracing::warn!("Gemini JSON parse failed on attempt 1, retrying: {}", e);
                    continue;
                }
                return Err(anyhow!("Gemini response could not be parsed as JSON after {} attempts: {}", attempt + 1, e));
            }
        }
    }

    Err(anyhow!("Gemini generation failed after retries"))
}

// ── Private helpers ──────────────────────────────────────────────────────────

fn build_request_body(
    audio_data: &[u8],
    prompt: &str,
    start_time: Option<f64>,
    end_time: Option<f64>,
    attempt: u32,
) -> Value {
    let audio_b64 = BASE64.encode(audio_data);

    // Build the user text part.
    let mut user_text = String::new();

    // Add timestamp focus if provided.
    if let (Some(start), Some(end)) = (start_time, end_time) {
        user_text.push_str(&format!(
            "FOCUS ONLY on the sound between {:0>1}:{:02} and {:0>1}:{:02} (mm:ss). Ignore audio outside this window.\n\n",
            (start as u64) / 60,
            (start as u64) % 60,
            (end as u64) / 60,
            (end as u64) % 60,
        ));
    }

    // Add user prompt if present.
    if !prompt.trim().is_empty() {
        user_text.push_str(&format!("Sound description: {}\n\n", prompt.trim()));
    }

    // On retry, add an explicit nudge.
    if attempt > 0 {
        user_text.push_str("IMPORTANT: Your previous response was not valid JSON. You MUST output ONLY the raw JSON object — no text before it, no text after it, no markdown code fences. Start your response with { and end with }.\n\n");
    }

    user_text.push_str("Now fill in the JSON template from the system prompt with your analysis values.");

    json!({
        "system_instruction": {
            "parts": [{"text": SYSTEM_PROMPT}]
        },
        "contents": [{
            "role": "user",
            "parts": [
                {
                    "inline_data": {
                        "mime_type": "audio/mp3",
                        "data": audio_b64
                    }
                },
                {
                    "text": user_text
                }
            ]
        }],
        "generationConfig": {
            "maxOutputTokens": 512,
            "temperature": 0.2,
            "responseMimeType": "application/json"
        }
    })
}

/// Extract the text content from a Gemini response and parse it as JSON.
fn extract_and_parse_content(resp: &Value) -> Result<Value> {
    let text = resp
        .get("candidates")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("content"))
        .and_then(|c| c.get("parts"))
        .and_then(|p| p.get(0))
        .and_then(|p| p.get("text"))
        .and_then(|t| t.as_str())
        .ok_or_else(|| anyhow!("Gemini response missing expected content structure"))?;

    // Strip markdown code fences if Gemini wrapped the output anyway.
    let cleaned = strip_markdown_fences(text);

    let parsed: Value = serde_json::from_str(cleaned.trim())
        .map_err(|e| anyhow!("Failed to parse Gemini output as JSON: {}. Raw: {:?}", e, cleaned))?;

    // Verify it's an object with at least some keys.
    if parsed.as_object().map(|m| m.is_empty()).unwrap_or(true) {
        return Err(anyhow!("Gemini returned an empty JSON object"));
    }

    Ok(parsed)
}

/// Remove ```json ... ``` or ``` ... ``` code fences from a string.
fn strip_markdown_fences(s: &str) -> &str {
    let s = s.trim();
    if let Some(inner) = s.strip_prefix("```json") {
        inner.trim_end_matches("```").trim()
    } else if let Some(inner) = s.strip_prefix("```") {
        inner.trim_end_matches("```").trim()
    } else {
        s
    }
}
