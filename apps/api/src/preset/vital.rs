/// Vital preset file generation and Gemini response merging.
///
/// Pipeline:
///   Gemini JSON output → validate & clamp → merge into Init.vital template
///   → serialize → gzip → Vec<u8> (a valid .vital file)
///
/// ⚠️  PLACEHOLDER TEMPLATE:
///   `PLACEHOLDER_INIT_TEMPLATE` is a minimal approximation of a Vital preset.
///   It will produce a basic working sound but will NOT perfectly match the
///   real Vital Init preset.
///
///   TODO (Nathan): Extract the real Init.vital JSON:
///   1. Open Vital synth
///   2. Load any preset, then File → Init Preset to reset to defaults
///   3. File → Save Preset As → save as "init_preset.vital"
///   4. gunzip init_preset.vital && cat init_preset.vital.gz (or just open as text)
///   5. Paste the JSON here, replacing PLACEHOLDER_INIT_TEMPLATE

use anyhow::{anyhow, Result};
use flate2::write::GzEncoder;
use flate2::Compression;
use serde_json::{json, Value};
use std::io::Write;

use super::schema::{find_param_range, VitalPreset};

// ── Placeholder Template ──────────────────────────────────────────────────────
//
// Minimal valid Vital-compatible JSON. All synthesis params are at neutral
// defaults. Replace this with the real Init.vital JSON once extracted.

const PLACEHOLDER_INIT_TEMPLATE: &str = r#"{
  "preset_name": "Preset.gg Generated",
  "author": "Preset.gg",
  "comments": "AI-generated preset from Preset.gg",
  "preset_style": "",
  "osc_1_wave_frame": 0.0,
  "osc_1_level": 1.0,
  "osc_1_pan": 0.5,
  "osc_1_tune": 0.5,
  "osc_1_transpose": 0.5,
  "osc_1_unison_voices": 0.0,
  "osc_1_unison_detune": 0.2,
  "osc_1_phase": 0.0,
  "osc_2_wave_frame": 0.0,
  "osc_2_level": 0.0,
  "osc_2_pan": 0.5,
  "osc_2_tune": 0.5,
  "osc_2_transpose": 0.5,
  "osc_2_unison_voices": 0.0,
  "osc_2_unison_detune": 0.2,
  "osc_2_phase": 0.0,
  "filter_1_cutoff": 1.0,
  "filter_1_resonance": 0.0,
  "filter_1_drive": 0.0,
  "filter_1_blend": 0.0,
  "filter_2_cutoff": 1.0,
  "filter_2_resonance": 0.0,
  "filter_2_drive": 0.0,
  "filter_2_blend": 0.0,
  "env_1_attack": 0.0,
  "env_1_decay": 0.3,
  "env_1_sustain": 1.0,
  "env_1_release": 0.3,
  "env_1_attack_power": 0.0,
  "env_1_decay_power": 0.0,
  "env_1_release_power": 0.0,
  "env_2_attack": 0.0,
  "env_2_decay": 0.3,
  "env_2_sustain": 0.0,
  "env_2_release": 0.3,
  "lfo_1_frequency": 0.3,
  "lfo_1_amount": 0.0,
  "lfo_1_phase": 0.0,
  "lfo_1_fade_time": 0.0,
  "lfo_1_delay_time": 0.0,
  "lfo_2_frequency": 0.3,
  "lfo_2_amount": 0.0,
  "lfo_2_phase": 0.0,
  "reverb_mix": 0.0,
  "reverb_decay_time": 0.5,
  "reverb_size": 0.5,
  "reverb_pre_low_cutoff": 0.0,
  "reverb_pre_high_cutoff": 1.0,
  "delay_mix": 0.0,
  "delay_frequency": 0.5,
  "delay_feedback": 0.3,
  "delay_filter_cutoff": 0.7,
  "chorus_mix": 0.0,
  "chorus_amount": 0.3,
  "chorus_frequency": 0.3,
  "chorus_feedback": 0.0,
  "distortion_amount": 0.0,
  "distortion_mix": 1.0,
  "distortion_drive": 0.5,
  "phaser_mix": 0.0,
  "phaser_amount": 0.5,
  "phaser_frequency": 0.3,
  "phaser_feedback": 0.3,
  "compressor_mix": 0.0,
  "compressor_attack": 0.3,
  "compressor_release": 0.3,
  "compressor_ratio": 0.3,
  "compressor_threshold": 0.5,
  "volume": 0.7,
  "velocity_track": 0.5,
  "pitch_range": 0.1,
  "stereo_routing": 0.5
}"#;

// ── Public API ────────────────────────────────────────────────────────────────

/// Merge Gemini's parameter output into the Init.vital template, then produce
/// a gzipped .vital file as bytes.
///
/// Steps:
/// 1. Load the Init.vital placeholder as a base JSON object
/// 2. For each key-value in Gemini's output:
///    - Drop keys not in ALLOWED_PARAMS (silently — keeps template defaults)
///    - Clamp values to the allowed range
///    - Overwrite the template's default with the clamped value
/// 3. Set preset metadata (name, author, comments)
/// 4. Serialize to JSON, gzip compress, return bytes
pub fn merge_gemini_into_template(
    gemini_output: &Value,
    prompt: &str,
    track_title: Option<&str>,
) -> Result<Vec<u8>> {
    let mut template: Value = serde_json::from_str(PLACEHOLDER_INIT_TEMPLATE)
        .map_err(|e| anyhow!("Failed to parse Init.vital template: {}", e))?;

    // Get the Gemini output as an object.
    let gemini_params = gemini_output
        .as_object()
        .ok_or_else(|| anyhow!("Gemini output is not a JSON object"))?;

    let mut merged_count = 0usize;
    let mut dropped_count = 0usize;

    for (key, raw_value) in gemini_params {
        match find_param_range(key) {
            None => {
                // Unknown key — drop silently, keep template default.
                dropped_count += 1;
                tracing::debug!("Dropping unknown Gemini param: {}", key);
            }
            Some((min, max)) => {
                if let Some(v) = raw_value.as_f64() {
                    let clamped = v.clamp(min, max);
                    template[key] = json!(clamped);
                    merged_count += 1;
                } else {
                    tracing::warn!("Gemini param {} has non-numeric value: {:?}", key, raw_value);
                    dropped_count += 1;
                }
            }
        }
    }

    tracing::info!(
        "Merged {} Gemini params into template ({} dropped as unknown/invalid)",
        merged_count,
        dropped_count
    );

    // Update preset metadata.
    let preset_name = match track_title {
        Some(title) => format!("{} — {}", title, prompt),
        None => format!("AI Preset — {}", prompt),
    };
    // Truncate to 64 chars to avoid overly long names.
    let preset_name = if preset_name.len() > 64 {
        format!("{}...", &preset_name[..61])
    } else {
        preset_name
    };

    template["preset_name"] = json!(preset_name);
    template["author"] = json!("Preset.gg");
    template["comments"] = json!(format!("Generated by Preset.gg AI. Prompt: {}", prompt));

    // Serialize and gzip.
    gzip_json(&template)
}

/// Create a .vital file from a VitalPreset struct.
/// Used internally and for testing.
pub fn create_vital_file(preset: &VitalPreset) -> Result<Vec<u8>> {
    let value = serde_json::to_value(preset)?;
    gzip_json(&value)
}

// ── Private helpers ───────────────────────────────────────────────────────────

/// Serialize a JSON value to pretty-printed JSON and gzip compress it.
fn gzip_json(value: &Value) -> Result<Vec<u8>> {
    let json_str = serde_json::to_string_pretty(value)
        .map_err(|e| anyhow!("JSON serialization failed: {}", e))?;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(json_str.as_bytes())
        .map_err(|e| anyhow!("Gzip write failed: {}", e))?;
    let compressed = encoder
        .finish()
        .map_err(|e| anyhow!("Gzip finish failed: {}", e))?;

    tracing::info!(
        "Vital file: {} bytes JSON → {} bytes gzipped",
        json_str.len(),
        compressed.len()
    );

    Ok(compressed)
}
