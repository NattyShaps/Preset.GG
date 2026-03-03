/// Gemini multimodal API integration.
///
/// Sends audio bytes + user prompt to Gemini 2.5 Pro and returns
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
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent";

const GEMINI_FLASH_API: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";

// ── System Prompts ───────────────────────────────────────────────────────────

/// Stage 1 system prompt: Flash expands casual user input into precise
/// sound design language that Stage 2 (Pro) can act on.
const ENHANCE_SYSTEM_PROMPT: &str = r#"You are an expert synthesizer sound design translator and classifier. Given audio and/or a text description, output a JSON object with exactly two fields:

1. "description": Expand the user's request into precise synthesis terminology (2-4 sentences). Describe oscillator types/waveforms, filter settings, envelope shape, effects, modulation, and overall character. If audio is provided, describe the dominant timbre you hear.

2. "category": Choose the single best-matching seed preset category from this list:
   - alien_bass: bizarre, modulated, experimental bass
   - arp_sequence: bright, rhythmic, arpeggiated sounds
   - bell: inharmonic, metallic, FM bell tones
   - drum_kick: kick drums, sub hits, percussion body
   - drum_snare: snares, claps, percussive transients
   - fx_riser: risers, sweeps, impacts, transitions
   - growl_bass: aggressive, distorted, dubstep bass
   - keys: piano, rhodes, electric piano, keyboard sounds
   - keys_ambient: soft, ambient keyboard textures
   - megasaw_lead: massive, wide, detuned lead synths
   - organ: sustained, additive, organ-like tones
   - pad_ambient: evolving, atmospheric, ambient pads
   - pad_warm: lush, warm, smooth sustained pads
   - pluck: short, percussive, plucked string sounds
   - sub_bass: deep, clean, sub-frequency bass
   - supersaw_lead: classic detuned sawtooth leads
   - tearout_bass: heavy, aggressive, tearing bass
   - vocal: formant, vocal-like, voice synthesis

Output ONLY the JSON object. No other text."#;

const SYSTEM_PROMPT: &str = r#"You are an expert synthesizer sound designer with deep knowledge of subtractive, FM, and wavetable synthesis. Your task is to analyze audio and determine which Vital synthesizer parameters would precisely recreate and clone the target timbre.

IMPORTANT CONTEXT:
The preset starts from a curated seed template that already has rich wavetables, modulation routings, LFO shapes, and effects pre-configured. Your parameter values will be merged on top of this foundation — you are fine-tuning an already good-sounding preset, not building from scratch. Focus on adjusting oscillator levels, filter settings, envelope shapes, and effect amounts to sculpt the seed toward the target sound.

TASK:
Listen carefully to the provided audio. Output a JSON object of Vital parameter values that closely clones the target timbre, harmonic content, and envelope shape. If a text description is also provided, prioritize it to refine the sound. Only include parameters you want to change from defaults — omit any you don't need.

TIMESTAMP CONTEXT:
If the user specifies a time range, analyze ONLY the sound during that window.

PARAMETER RULES:
- ALL values must be normalized 0.0–1.0 (inclusive). 0.0=minimum, 1.0=maximum.
- Only include parameters you intentionally want to set. Omit the rest (they keep safe defaults).
- Use all 3 oscillators, both filters, envelope shaping, LFOs, and effects to build rich, full sounds.

AVAILABLE PARAMETERS — organized by section. Replace N with 1, 2, or 3 for oscillators/filters/envelopes/LFOs.

── Oscillators (osc_1, osc_2, osc_3) ──
osc_N_wave_frame: 0.0=sine, 0.25=triangle, 0.5=sawtooth, 0.75=square, 1.0=noise
osc_N_level: 0.0=silent, 0.707=default, 1.0=max
osc_N_pan: 0.0=hard left, 0.5=center, 1.0=hard right
osc_N_tune: 0.5=center (0 cents), 0.0=-100 cents, 1.0=+100 cents
osc_N_transpose: 0.5=0 semitones, 0.0=-48, 1.0=+48
osc_N_unison_voices: 0.0=1 voice, 1.0=16 voices
osc_N_unison_detune: 0.0=none, 1.0=max detune
osc_N_unison_blend: 0.0=none, 1.0=full blend
osc_N_phase: 0.0–1.0
osc_N_random_phase: 0=off, 1=on
osc_N_distortion_amount: 0.0=none, 1.0=max
osc_N_distortion_type: 0=none, ~0.09=sync, ~0.18=formant, ~0.27=quantize, ~0.36=bend, ~0.45=squeeze, ~0.55=FM, ~0.64=RM, ~0.73=squeeze2, ~0.82=sample_and_hold, ~0.91=spectral
osc_N_spectral_morph_amount: 0.0=none, 1.0=max
osc_N_spectral_morph_type: 0=none, ~0.09=vocode, ~0.18=formant_scale, ~0.27=harmonic_stretch, ~0.36=inharmonic_stretch, ~0.45=smear, ~0.55=randomize, ~0.64=low_pass, ~0.73=high_pass, ~0.82=phase_disperse, ~0.91=shepard_tone
osc_N_frame_spread: 0.0–1.0 (spread across wavetable frames in unison)
osc_N_stereo_spread: 0.0–1.0
osc_N_detune_range: 0.0=tight, 1.0=wide detune range

── Filters (filter_1, filter_2) ──
filter_N_cutoff: 0.0=20Hz, 0.5=~1kHz, 1.0=20kHz
filter_N_resonance: 0.0=none, 1.0=self-oscillation
filter_N_drive: 0.0=clean, 1.0=heavy saturation
filter_N_blend: 0.0=low-pass, 0.5=band-pass, 1.0=high-pass
filter_N_mix: 0.0=bypass, 1.0=fully filtered
filter_N_keytrack: 0.0=none, 1.0=full key tracking
filter_N_model: 0.0=Analog, 0.2=Dirty, 0.4=Ladder, 0.6=Comb, 0.8=Digital, 1.0=Formant
filter_N_style: 0.0=12dB, 0.33=24dB, 0.67=Notch, 1.0=Dual Notch

── Envelopes (env_1=amplitude, env_2=modulation, env_3=extra) ──
env_N_attack: 0.0=instant, 0.25=~1s, 0.5=~2s, 1.0=~4s
env_N_decay: same scale as attack
env_N_sustain: 0.0=silent, 1.0=full level
env_N_release: same scale as attack
env_N_attack_power: 0.0=logarithmic curve, 0.5=linear, 1.0=exponential curve
env_N_decay_power: same as attack_power
env_N_release_power: same as attack_power
env_N_delay: 0.0=none, 1.0=4s delay before envelope starts
env_N_hold: 0.0=none, 1.0=4s hold at peak before decay

── LFOs (lfo_1, lfo_2, lfo_3) ──
lfo_N_frequency: 0.0=very slow (sub-Hz), 0.5=moderate, 1.0=audio rate
lfo_N_phase: 0.0–1.0
lfo_N_fade_time: 0.0=instant, 1.0=slow fade in
lfo_N_delay_time: 0.0=none, 1.0=long delay before LFO starts
lfo_N_stereo: 0.0=mono, 1.0=full stereo offset
lfo_N_tempo: 0.0–1.0 (tempo-synced rate when synced)
lfo_N_sync_type: 0.0=free, 0.33=tempo, 0.67=dotted, 1.0=triplet

── Reverb ──
reverb_dry_wet, reverb_decay_time, reverb_size: 0.0–1.0
reverb_chorus_amount: 0.0–1.0 (modulation inside reverb)
reverb_delay: 0.0–1.0 (pre-delay)
reverb_high_shelf_cutoff: 0.0=20Hz, 1.0=20kHz
reverb_high_shelf_gain: 0.0=-6dB, 0.5=0dB, 1.0=+6dB
reverb_low_shelf_cutoff, reverb_low_shelf_gain: same as high shelf
reverb_pre_high_cutoff, reverb_pre_low_cutoff: 0.0=20Hz, 1.0=20kHz

── Delay ──
delay_dry_wet, delay_feedback: 0.0–1.0
delay_frequency: 0.0=very slow, 0.5=moderate, 1.0=very fast
delay_filter_cutoff: 0.0=20Hz, 1.0=20kHz
delay_filter_spread: 0.0–1.0
delay_style: 0.0=stereo, 0.33=ping-pong, 0.67=mid/side
delay_tempo: 0.0–1.0 (tempo-synced rate)

── Chorus ──
chorus_dry_wet, chorus_feedback, chorus_mod_depth, chorus_spread: 0.0–1.0
chorus_cutoff: 0.0=20Hz, 1.0=20kHz
chorus_frequency: 0.0=very slow, 0.5=moderate, 1.0=fast
chorus_voices: 0.0=1 voice, 1.0=8 voices
chorus_tempo: 0.0–1.0

── Distortion ──
distortion_drive, distortion_mix: 0.0–1.0
distortion_type: 0.0=Soft Clip, 0.14=Hard Clip, 0.29=Linear Fold, 0.43=Sine Fold, 0.57=Bit Crush, 0.71=Down Sample, 0.86=Multi-band, 1.0=Multi-band
distortion_filter_cutoff: 0.0=20Hz, 1.0=20kHz
distortion_filter_resonance: 0.0–1.0
distortion_filter_blend: 0.0=LP, 0.5=BP, 1.0=HP

── Compressor ──
compressor_attack, compressor_release, compressor_mix: 0.0–1.0
compressor_band_gain, compressor_low_gain, compressor_high_gain: 0.0=-6dB, 0.5=~12dB, 1.0=30dB

── EQ ──
eq_band_cutoff, eq_high_cutoff, eq_low_cutoff: 0.0=20Hz, 1.0=20kHz
eq_band_gain, eq_high_gain, eq_low_gain: 0.0=-15dB, 0.5=0dB, 1.0=+15dB
eq_band_resonance, eq_high_resonance, eq_low_resonance: 0.0–1.0

── Flanger ──
flanger_dry_wet, flanger_feedback, flanger_mod_depth, flanger_phase_offset: 0.0–1.0
flanger_center: 0.0=20Hz, 1.0=20kHz
flanger_frequency: 0.0=very slow, 1.0=fast
flanger_tempo: 0.0–1.0

── Phaser ──
phaser_dry_wet, phaser_feedback, phaser_phase_offset: 0.0–1.0
phaser_center: 0.0=20Hz, 1.0=20kHz
phaser_frequency: 0.0=very slow, 1.0=fast
phaser_mod_depth: 0.0=none, 1.0=48 semitones
phaser_tempo: 0.0–1.0

── Sample ──
sample_level: 0.0=silent, 1.0=max
sample_pan: 0.0=left, 0.5=center, 1.0=right
sample_transpose: 0.5=0 semi, 0.0=-48, 1.0=+48
sample_tune: 0.5=center, 0.0=-100 cents, 1.0=+100 cents

── Voice / Global ──
polyphony: 0.0=1 voice (mono), 1.0=32 voices
portamento_time: 0.0=none, 1.0=4s glide
pitch_bend_range: 0.5=0 semi, 0.0=-48, 1.0=+48
velocity_track: 0.0=off, 1.0=full velocity sensitivity
voice_amplitude: 0.0–1.0
legato: 0=off, 1=on

── Macro Controls ──
macro_control_1, macro_control_2, macro_control_3, macro_control_4: 0.0–1.0

SOUND DESIGN GUIDELINES:
- Heavy bass: osc_1 saw/square (0.5-0.75), osc_2 sub-sine one octave down (transpose ~0.375), filter_1 LP cutoff ~0.3-0.5, high resonance, fast attack, distortion_type ~0.0 (soft clip), compressor for punch
- Bright lead: osc_1 saw (0.5), filter_1 high cutoff ~0.7-0.9, unison 4-8 voices, instant attack, chorus for width
- Ethereal pad: osc_1 sine/tri (0.0-0.25), slow attack ~0.2-0.5, long release, reverb ~0.4-0.8, chorus ~0.2-0.5, phaser for movement, env_1_attack_power ~0.3 (log curve)
- Pluck/keys: instant attack, fast decay ~0.05-0.15, low sustain, short release, filter_1 LP with env_2 modulation feel
- FM/growl bass: osc_1 + osc_2 with spectral_morph (FM type ~0.55), high distortion, filter_1 low cutoff, high resonance
- Ambient texture: all 3 oscillators, spectral morph, random phase, wide stereo spread, long reverb, slow LFO modulation
- Aggressive synth: distortion Sine Fold (~0.43), compressor, EQ boost highs, filter drive, fast LFO on filter cutoff

OUTPUT FORMAT — CRITICAL:
Output ONLY a valid JSON object with your chosen parameter values. No other text, no explanation, no markdown, no code fences. Only include parameters you want to set (omit defaults). Example format:

{"osc_1_wave_frame":0.5,"osc_1_level":0.8,"filter_1_cutoff":0.6,"env_1_attack":0.02,"reverb_dry_wet":0.15}"#;

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

/// Stage 1: Expand a casual user prompt into detailed sound design language
/// and classify into a seed category, using Gemini Flash (cheap and fast).
///
/// Returns `(enhanced_description, seed_category)`.
/// If audio is provided, Flash also describes the timbre it hears.
/// On failure, callers should fall back to the original user prompt.
pub async fn enhance_prompt(
    api_key: &str,
    audio_data: Option<&[u8]>,
    user_prompt: &str,
    start_time: Option<f64>,
    end_time: Option<f64>,
    timeout_secs: u64,
) -> Result<(String, String)> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()?;

    let url = format!("{}?key={}", GEMINI_FLASH_API, api_key);

    let body = build_enhance_body(audio_data, user_prompt, start_time, end_time);

    tracing::info!(
        "Flash enhance+classify request: prompt={:?}, has_audio={}",
        user_prompt,
        audio_data.is_some()
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
        return Err(anyhow!("Flash API error {}: {}", status, text));
    }

    let resp_json: Value = resp.json().await?;

    let text = resp_json
        .get("candidates")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("content"))
        .and_then(|c| c.get("parts"))
        .and_then(|p| p.get(0))
        .and_then(|p| p.get("text"))
        .and_then(|t| t.as_str())
        .ok_or_else(|| anyhow!("Flash response missing expected content structure"))?;

    // Parse Flash JSON output: {"description": "...", "category": "..."}
    let flash_json: Value = serde_json::from_str(text.trim())
        .map_err(|e| anyhow!("Flash returned non-JSON: {}. Raw: {:?}", e, text))?;

    let description = flash_json
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or(user_prompt)
        .trim()
        .to_string();

    let category = flash_json
        .get("category")
        .and_then(|v| v.as_str())
        .unwrap_or("init")
        .trim()
        .to_string();

    if description.is_empty() {
        return Err(anyhow!("Flash returned empty description"));
    }

    Ok((description, category))
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
            "The user selected the region {:0>1}:{:02}–{:0>1}:{:02} (mm:ss) as a reference. Listen to that window to identify the TARGET INSTRUMENT or timbre the user wants. Your job is to create a playable synthesizer patch that captures that instrument's tone, NOT to reproduce the entire passage. The preset should sound good when a single note or chord is played — do NOT add long reverb/delay tails just to fill time.\n\n",
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

    user_text.push_str("Analyze the audio and output a single JSON object containing only the parameters you want to change from their defaults. Every key must be a parameter name from the reference above. Every value must be a number between 0.0 and 1.0. Output nothing but the JSON object.");

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
            "maxOutputTokens": 4096,
            "temperature": 0.2,
            "responseMimeType": "application/json"
        }
    })
}

/// Build the request body for the Flash enhance-prompt call.
fn build_enhance_body(
    audio_data: Option<&[u8]>,
    user_prompt: &str,
    start_time: Option<f64>,
    end_time: Option<f64>,
) -> Value {
    let mut user_text = String::new();

    // Add timestamp focus if provided.
    if let (Some(start), Some(end)) = (start_time, end_time) {
        user_text.push_str(&format!(
            "The user selected {:0>1}:{:02}–{:0>1}:{:02} (mm:ss) as a reference window. Identify the dominant instrument or timbre in that region — describe how to recreate it as a playable synth patch, not as a reproduction of the full passage.\n\n",
            (start as u64) / 60,
            (start as u64) % 60,
            (end as u64) / 60,
            (end as u64) % 60,
        ));
    }

    user_text.push_str(&format!(
        "User's sound request: \"{}\"\n\nExpand this into a detailed sound design description.",
        user_prompt.trim()
    ));

    // Build parts: audio (if available) + text
    let mut parts = Vec::new();

    if let Some(audio) = audio_data {
        let audio_b64 = BASE64.encode(audio);
        parts.push(json!({
            "inline_data": {
                "mime_type": "audio/mp3",
                "data": audio_b64
            }
        }));
    }

    parts.push(json!({"text": user_text}));

    json!({
        "system_instruction": {
            "parts": [{"text": ENHANCE_SYSTEM_PROMPT}]
        },
        "contents": [{
            "role": "user",
            "parts": parts
        }],
        "generationConfig": {
            "maxOutputTokens": 1024,
            "temperature": 0.3,
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
