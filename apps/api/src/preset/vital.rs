/// Vital preset file generation and Gemini response merging.
///
/// Pipeline:
///   Gemini JSON output (normalized 0–1 values)
///   → look up each param in the mapping table
///   → convert from 0–1 to real Vital range
///   → merge into the real Init.vital template under ["settings"]
///   → auto-enable effects whose wet mix is nonzero
///   → set metadata at root level
///   → serialize → Vec<u8> (raw JSON — the .vital format is uncompressed JSON)

use anyhow::{anyhow, Result};
use serde_json::{json, Value};

use super::schema::find_mapping;

// ── Public API ────────────────────────────────────────────────────────────────

/// Merge Gemini's parameter output into a seed template,
/// then produce a raw JSON .vital file as bytes.
pub fn merge_gemini_into_template(
    gemini_output: &Value,
    template_json: &str,
    prompt: &str,
    track_title: Option<&str>,
) -> Result<Vec<u8>> {
    // 1. Parse the seed template.
    let mut template: Value = serde_json::from_str(template_json)
        .map_err(|e| anyhow!("Failed to parse seed template: {}", e))?;

    // 2. Get mutable reference to the "settings" sub-object.
    let settings = template
        .get_mut("settings")
        .and_then(|s| s.as_object_mut())
        .ok_or_else(|| anyhow!("Init.vital template missing 'settings' object"))?;

    // 3. Get the Gemini output as an object.
    let gemini_params = gemini_output
        .as_object()
        .ok_or_else(|| anyhow!("Gemini output is not a JSON object"))?;

    let mut merged_count = 0usize;
    let mut dropped_count = 0usize;

    // 4. For each Gemini param, look up the mapping and convert.
    for (key, raw_value) in gemini_params {
        match find_mapping(key) {
            None => {
                dropped_count += 1;
                tracing::debug!("Dropping unknown Gemini param: {}", key);
            }
            Some(mapping) => {
                if let Some(v) = raw_value.as_f64() {
                    // Clamp Gemini's output to 0–1 first.
                    let clamped = v.clamp(0.0, 1.0);
                    // Convert to real Vital range, then clamp to valid bounds.
                    let vital_value = (mapping.convert)(clamped);
                    let safe_value = vital_value.clamp(mapping.min, mapping.max);
                    settings.insert(mapping.vital_key.to_string(), json!(safe_value));
                    merged_count += 1;
                } else {
                    tracing::warn!(
                        "Gemini param {} has non-numeric value: {:?}",
                        key,
                        raw_value
                    );
                    dropped_count += 1;
                }
            }
        }
    }

    // 5. Auto-enable effects only if Gemini explicitly set a param for them.
    //    We check Gemini's raw output keys (not template defaults) to avoid
    //    spuriously enabling effects whose template dry_wet is nonzero.
    const EFFECT_PREFIXES: &[(&str, &str)] = &[
        ("reverb_",      "reverb_on"),
        ("delay_",       "delay_on"),
        ("chorus_",      "chorus_on"),
        ("distortion_",  "distortion_on"),
        ("flanger_",     "flanger_on"),
        ("phaser_",      "phaser_on"),
        ("compressor_",  "compressor_on"),
        ("eq_",          "eq_on"),
    ];
    for (prefix, on_key) in EFFECT_PREFIXES {
        if gemini_params.keys().any(|k| k.starts_with(prefix)) {
            settings.insert(on_key.to_string(), json!(1.0));
        }
    }

    // 6. Auto-enable oscillators only if Gemini explicitly set any param for them.
    if gemini_params.keys().any(|k| k.starts_with("osc_2_")) {
        settings.insert("osc_2_on".to_string(), json!(1.0));
    }
    if gemini_params.keys().any(|k| k.starts_with("osc_3_")) {
        settings.insert("osc_3_on".to_string(), json!(1.0));
    }

    // 7. If Gemini set filter params, enable filter 1.
    if gemini_params.contains_key("filter_1_cutoff")
        || gemini_params.contains_key("filter_1_resonance")
        || gemini_params.contains_key("filter_1_drive")
        || gemini_params.contains_key("filter_1_blend")
    {
        settings.insert("filter_1_on".to_string(), json!(1.0));
    }

    // 7b. If Gemini set filter 2 params, enable filter 2.
    if gemini_params.contains_key("filter_2_cutoff")
        || gemini_params.contains_key("filter_2_resonance")
        || gemini_params.contains_key("filter_2_drive")
        || gemini_params.contains_key("filter_2_blend")
    {
        settings.insert("filter_2_on".to_string(), json!(1.0));
    }

    // 7c. If Gemini set sample params, enable sample.
    if gemini_params.contains_key("sample_level")
        || gemini_params.contains_key("sample_transpose")
    {
        settings.insert("sample_on".to_string(), json!(1.0));
    }

    // 7d. If Gemini set EQ params, enable EQ.
    if gemini_params.contains_key("eq_band_gain")
        || gemini_params.contains_key("eq_low_gain")
        || gemini_params.contains_key("eq_high_gain")
    {
        settings.insert("eq_on".to_string(), json!(1.0));
    }

    tracing::info!(
        "Merged {} Gemini params into template ({} dropped as unknown/invalid)",
        merged_count,
        dropped_count
    );

    // 8. Set preset metadata at the ROOT level (not in settings).
    //    Vital's C++ uses fixed-size buffers for string fields.
    //    Use ASCII only (no multi-byte chars) and truncate conservatively
    //    to stay well within buffer limits.
    let preset_name = match track_title {
        Some(title) => format!("{} - {}", title, prompt),
        None => format!("AI Preset - {}", prompt),
    };
    let preset_name = truncate_ascii_safe(&preset_name, 50);

    template["preset_name"] = json!(preset_name);
    template["author"] = json!("Preset.gg");
    let comments = format!("Generated by Preset.gg AI. Prompt: {}", prompt);
    let comments = truncate_ascii_safe(&comments, 100);
    template["comments"] = json!(comments);

    // 9. Serialize to raw JSON (Vital expects uncompressed JSON in .vital files).
    serialize_json(&template)
}

// ── Private helpers ───────────────────────────────────────────────────────────

/// Truncate a string to at most `max_bytes` bytes, ensuring we never
/// split a multi-byte UTF-8 character. Appends "..." if truncated.
fn truncate_ascii_safe(s: &str, max_bytes: usize) -> String {
    if s.len() <= max_bytes {
        return s.to_string();
    }
    // Walk back from max_bytes-3 (room for "...") to find a valid char boundary.
    let mut end = max_bytes.saturating_sub(3);
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}...", &s[..end])
}

/// Serialize a JSON value to raw bytes (no compression).
fn serialize_json(value: &Value) -> Result<Vec<u8>> {
    let json_str = serde_json::to_string(value)
        .map_err(|e| anyhow!("JSON serialization failed: {}", e))?;

    tracing::info!("Vital file: {} bytes", json_str.len());

    Ok(json_str.into_bytes())
}
