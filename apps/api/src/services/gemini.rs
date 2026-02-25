/// Gemini 3.1 Pro API integration.
///
/// Sends audio data + prompt to Gemini's multimodal API and receives
/// a structured JSON response matching the Vital preset schema.

use anyhow::Result;
use serde_json::Value;

/// Send audio + text prompt to Gemini and receive a preset JSON response.
///
/// # Arguments
/// * `api_key` - Gemini API key
/// * `audio_data` - Raw audio bytes
/// * `prompt` - User's text description of the desired sound
///
/// # Returns
/// A JSON value matching the Vital preset schema.
pub async fn generate_preset_json(
    _api_key: &str,
    _audio_data: &[u8],
    _prompt: &str,
) -> Result<Value> {
    // TODO: Implementation steps:
    // 1. Construct the multimodal request with audio + system prompt
    // 2. System prompt instructs Gemini to output ONLY a valid Vital preset JSON
    // 3. Send request to Gemini 3.1 Pro API
    // 4. Parse and validate the JSON response
    // 5. Return the preset JSON

    // System prompt template:
    // "You are a professional sound designer. Analyze the provided audio sample
    //  and generate a Vital synthesizer preset that recreates the sound.
    //  Output ONLY a valid JSON object matching the Vital preset schema.
    //  Do not include any explanation or markdown."

    anyhow::bail!("Gemini integration not yet implemented")
}
