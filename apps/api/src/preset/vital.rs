/// Vital preset file generation.
///
/// Converts a Gemini JSON response into a valid .vital file
/// (gzipped JSON matching the Vital synthesizer schema).

use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

use super::schema::VitalPreset;

/// Convert a Gemini JSON response into a .vital file (gzipped JSON).
///
/// # Arguments
/// * `preset` - The preset data to serialize
///
/// # Returns
/// Gzipped bytes ready to be saved as a .vital file.
pub fn create_vital_file(preset: &VitalPreset) -> Result<Vec<u8>> {
    let json = serde_json::to_string_pretty(preset)?;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(json.as_bytes())?;
    let compressed = encoder.finish()?;

    Ok(compressed)
}

/// Parse a Gemini JSON response into a VitalPreset struct.
pub fn parse_gemini_response(json: &serde_json::Value) -> Result<VitalPreset> {
    let preset: VitalPreset = serde_json::from_value(json.clone())?;
    Ok(preset)
}
