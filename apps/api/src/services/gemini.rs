/// Gemini multimodal API integration.
///
/// Sends audio bytes + user prompt to Gemini 2.5 Flash and returns
/// a structured JSON object of Vital synthesizer parameter values.
///
/// Key technique (from spike): provide a template JSON and ask Gemini to
/// fill in values. This prevents verbose output and guarantees parseable JSON.
///
/// All parameter names match real Vital keys. Values are normalized 0.0–1.0;
/// the Rust merge layer converts to actual Vital ranges.

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde_json::{json, Value};
use std::time::Duration;

use crate::config::GEMINI_MAX_RETRIES;

const GEMINI_API_BASE: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";

// ── System Prompt ────────────────────────────────────────────────────────────

const SYSTEM_PROMPT: &str = r#"You are an expert synthesizer sound designer with deep knowledge of subtractive, FM, and wavetable synthesis. Your task is to analyze audio and determine which synthesizer parameters would best recreate or complement that sound in the Vital synthesizer.

TASK:
Listen carefully to the provided audio. Then fill in the JSON template below with parameter values that would produce a synthesizer sound matching or complementing what you hear. If a text description is also provided, prioritize it to refine the sound type (e.g., "heavy bass", "bright lead", "ethereal pad").

TIMESTAMP CONTEXT:
If the user specifies a time range (e.g., "focus on 0:30 to 0:45"), analyze ONLY the sound during that window. Ignore the rest of the audio.

PARAMETER RULES:
- ALL values must be normalized between 0.0 and 1.0 (inclusive).
- 0.0 = minimum, 1.0 = maximum for each parameter.
- Oscillator wave_frame: 0.0=sine, 0.25=triangle, 0.5=sawtooth, 0.75=square, 1.0=noise.
- Oscillator level: 0.0=silent, 0.707=default (-3dB), 1.0=maximum.
- Oscillator pan: 0.0=hard left, 0.5=center, 1.0=hard right.
- Oscillator tune: 0.5=center (0 cents), 0.0=−100 cents, 1.0=+100 cents.
- Oscillator transpose: 0.5=0 semitones, 0.0=−48 semi, 1.0=+48 semi.
- Oscillator unison_voices: 0.0=1 voice, 1.0=16 voices.
- Envelope attack/decay/release: 0.0=instant, 0.25=fast (~1s), 0.5=medium (~2s), 1.0=very slow (~4s).
- Envelope sustain: 0.0=silent, 1.0=full volume held.
- Filter cutoff: 0.0=fully closed (20Hz), 0.5=middle (~1kHz), 1.0=fully open (20kHz).
- Filter resonance: 0.0=none, 1.0=maximum resonance/self-oscillation.
- Filter blend: 0.0=low-pass, 0.5=band-pass, 1.0=high-pass.
- LFO frequency: 0.0=very slow (sub-Hz), 0.5=moderate, 1.0=very fast (audio rate).
- Effect dry_wet/mix: 0.0=fully dry (off), 1.0=fully wet.
- Distortion drive: 0.0=clean, 1.0=maximum distortion.

SOUND DESIGN GUIDELINES BY TYPE:
- Heavy bass: osc_1_wave_frame ~0.5-0.75 (saw/square), filter_1_cutoff ~0.3-0.5, filter_1_resonance ~0.3-0.6, env_1_attack ~0.0, env_1_decay ~0.1-0.2, env_1_sustain ~0.5-0.8, reverb_dry_wet ~0.0-0.1
- Bright lead: osc_1_wave_frame ~0.5 (saw), filter_1_cutoff ~0.7-0.9, env_1_attack ~0.0-0.05, env_1_sustain ~0.7-1.0, reverb_dry_wet ~0.1-0.3
- Ethereal pad: osc_1_wave_frame ~0.0-0.25 (sine/tri), filter_1_cutoff ~0.4-0.7, env_1_attack ~0.2-0.5, env_1_release ~0.3-0.6, reverb_dry_wet ~0.4-0.8, chorus_dry_wet ~0.2-0.5
- Pluck: env_1_attack ~0.0, env_1_decay ~0.05-0.15, env_1_sustain ~0.0-0.2, env_1_release ~0.05-0.15
- FM bass (Skrillex-style): osc_1_wave_frame ~0.5-0.75, osc_2_level ~0.3-0.6, filter_1_cutoff ~0.2-0.4, filter_1_resonance ~0.5-0.8, distortion_drive ~0.3-0.6

OUTPUT FORMAT — CRITICAL:
Output ONLY this exact JSON structure with your values filled in. No other text, no explanation, no markdown, no code fences. Just the raw JSON:

{"osc_1_wave_frame":0.5,"osc_1_level":0.8,"osc_1_pan":0.5,"osc_1_tune":0.5,"osc_1_transpose":0.5,"osc_1_unison_voices":0.0,"osc_1_unison_detune":0.2,"osc_1_phase":0.5,"osc_2_wave_frame":0.25,"osc_2_level":0.0,"osc_2_pan":0.5,"osc_2_tune":0.5,"osc_2_transpose":0.5,"osc_2_unison_voices":0.0,"osc_2_unison_detune":0.2,"osc_2_phase":0.5,"filter_1_cutoff":0.6,"filter_1_resonance":0.3,"filter_1_drive":0.0,"filter_1_blend":0.0,"env_1_attack":0.02,"env_1_decay":0.3,"env_1_sustain":0.7,"env_1_release":0.2,"env_2_attack":0.0,"env_2_decay":0.2,"env_2_sustain":0.0,"env_2_release":0.2,"lfo_1_frequency":0.3,"lfo_1_phase":0.0,"lfo_1_fade_time":0.0,"lfo_1_delay_time":0.0,"reverb_dry_wet":0.15,"reverb_decay_time":0.4,"reverb_size":0.5,"delay_dry_wet":0.0,"delay_feedback":0.3,"delay_frequency":0.5,"chorus_dry_wet":0.0,"chorus_feedback":0.3,"distortion_drive":0.0,"distortion_mix":0.5}"#;

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
                tracing::info!(
                    "Gemini returned {} parameters",
                    params.as_object().map(|m| m.len()).unwrap_or(0)
                );
                return Ok(params);
            }
            Err(e) => {
                if attempt == 0 {
                    tracing::warn!("Gemini JSON parse failed on attempt 1, retrying: {}", e);
                    continue;
                }
                return Err(anyhow!(
                    "Gemini response could not be parsed as JSON after {} attempts: {}",
                    attempt + 1,
                    e
                ));
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
            "maxOutputTokens": 2048,
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
