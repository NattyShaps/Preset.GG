/// POST /api/generate — Main preset generation endpoint.
///
/// Pipeline:
///   1. Validate request
///   2. Rate limit check (IP-based for unauthenticated users)
///   3. Fetch audio from Audius (if track_id provided)
///   4. Enforce audio size budget
///   5. Send audio + prompt to Gemini
///   6. Merge Gemini output into selected seed template
///   7. Return preset file inline as base64 (direct download mode)

use axum::extract::{ConnectInfo, State};
use axum::Json;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;

use crate::config::{AUDIUS_FETCH_TIMEOUT_SECS, GEMINI_FLASH_TIMEOUT_SECS, GEMINI_REQUEST_TIMEOUT_SECS, MAX_AUDIO_SIZE_BYTES};
use crate::errors::AppError;
use crate::preset::vital::merge_gemini_into_template;
use crate::preset::schema::get_seed_template;
use crate::services::{audius, gemini};
use crate::AppState;

// ── Request / Response ────────────────────────────────────────────────────────

/// Request body for POST /api/generate
#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    /// User's text description of the desired sound
    pub prompt: Option<String>,
    /// Audius track ID to analyze
    pub track_id: Option<String>,
    /// Focus window start (seconds)
    pub start_timestamp: Option<f64>,
    /// Focus window end (seconds)
    pub end_timestamp: Option<f64>,
    /// Output format: "vital" (default) or "fxp"
    pub format: Option<String>,
    /// Wallet public key for tier checking (post-MVP)
    pub wallet_pubkey: Option<String>,
}

/// Response body for POST /api/generate
#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub preset_id: String,
    pub download_url: String,
    pub file_name: String,
    pub format: String,
    /// Base64-encoded .vital file bytes for direct client-side download.
    pub preset_data: String,
    /// How many generations the user has used in the current 24h window.
    pub generations_used: u32,
    /// Maximum generations allowed in the 24h window.
    pub generations_limit: u32,
}

// ── Handler ───────────────────────────────────────────────────────────────────

pub async fn generate_preset(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(payload): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, AppError> {
    let config = &state.config;

    // ── Step 1: Validate request ──────────────────────────────────────────────

    let prompt = payload.prompt.as_deref().unwrap_or("").trim().to_string();
    let track_id = payload.track_id.as_deref().unwrap_or("").trim().to_string();

    if prompt.is_empty() && track_id.is_empty() {
        return Err(AppError::BadRequest(
            "At least one of 'prompt' or 'track_id' is required".to_string(),
        ));
    }

    let format = payload.format.as_deref().unwrap_or("vital").to_lowercase();
    if format != "vital" && format != "fxp" {
        return Err(AppError::BadRequest(
            "format must be 'vital' or 'fxp'".to_string(),
        ));
    }

    tracing::info!(
        "Generate request: prompt={:?}, track_id={:?}, format={}, ip={}",
        prompt,
        track_id,
        format,
        addr.ip()
    );

    // ── Step 2: Rate limit (IP-based for MVP) ───────────────────────────────

    let ip = addr.ip();
    let (generations_used, generations_limit) = state.rate_limiter.check_and_record(ip)?;

    tracing::info!(
        "Rate limit check passed: ip={}, used={}/{}",
        ip, generations_used, generations_limit
    );

    // ── Step 3: Fetch audio from Audius ───────────────────────────────────────

    let (audio_bytes, track_title): (Vec<u8>, Option<String>) = if !track_id.is_empty() {
        tracing::info!("Fetching audio for track: {}", track_id);

        let bytes = audius::fetch_audio(
            &track_id,
            &config.audius_api_key,
            AUDIUS_FETCH_TIMEOUT_SECS,
            MAX_AUDIO_SIZE_BYTES,
        )
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("too large") {
                AppError::BadRequest(
                    "Audio too large. Please select a shorter region using the focus window.".to_string(),
                )
            } else {
                AppError::Internal(format!("Failed to fetch audio from Audius: {}", msg))
            }
        })?;

        // [M3 TODO] Fetch track metadata for the preset name. For now, use track_id.
        (bytes, Some(format!("Track {}", &track_id[..8.min(track_id.len())])))
    } else {
        // No track — text-only prompt mode. Gemini will work from the prompt alone.
        tracing::info!("No track_id provided — text-only prompt mode");
        (generate_silence_bytes(), None)
    };

    // ── Step 3b: Audio size check (belt + suspenders) ─────────────────────────

    if audio_bytes.len() > MAX_AUDIO_SIZE_BYTES {
        return Err(AppError::BadRequest(
            "Audio too large. Please select a shorter region using the focus window.".to_string(),
        ));
    }

    // ── Step 4: Enhance prompt via Flash ──────────────────────────────────────

    let (enhanced_prompt, seed_category) = if !prompt.is_empty() {
        match gemini::enhance_prompt(
            &config.gemini_api_key,
            if !track_id.is_empty() { Some(&audio_bytes) } else { None },
            &prompt,
            payload.start_timestamp,
            payload.end_timestamp,
            GEMINI_FLASH_TIMEOUT_SECS,
        )
        .await
        {
            Ok((description, category)) => {
                tracing::info!("Enhanced prompt: {:?}, seed category: {:?}", description, category);
                (description, category)
            }
            Err(e) => {
                tracing::warn!("Prompt enhancement failed, using original: {}", e);
                (prompt.clone(), "init".to_string())
            }
        }
    } else {
        (prompt.clone(), "init".to_string())
    };

    // ── Step 5: Send to Gemini ────────────────────────────────────────────────

    tracing::info!(
        "Sending {} bytes of audio to Gemini (prompt: {:?})",
        audio_bytes.len(),
        enhanced_prompt
    );

    let gemini_output = gemini::generate_preset_json(
        &config.gemini_api_key,
        &audio_bytes,
        &enhanced_prompt,
        payload.start_timestamp,
        payload.end_timestamp,
        GEMINI_REQUEST_TIMEOUT_SECS,
    )
    .await
    .map_err(|e| {
        AppError::Internal(format!("AI generation failed: {}. Try a different prompt or track.", e))
    })?;

    // ── Step 6: Select seed template and merge ───────────────────────────────

    let seed_template = get_seed_template(&seed_category);
    tracing::info!("Using seed template: {}", seed_category);

    let vital_bytes = merge_gemini_into_template(
        &gemini_output,
        seed_template,
        &enhanced_prompt,
        track_title.as_deref(),
    )
    .map_err(|e| AppError::Internal(format!("Preset generation failed: {}", e)))?;

    tracing::info!("Generated .vital file: {} bytes", vital_bytes.len());

    // ── Step 7: Encode file for direct download ──────────────────────────────

    let preset_data = BASE64.encode(&vital_bytes);

    let preset_id = Uuid::new_v4().to_string();
    let file_name = format!("preset_{}.vital", &preset_id[..8]);

    let download_url = format!("/api/presets/{}/download", preset_id);

    tracing::info!(
        "Generation complete. preset_id={}, file={}, size={}B, gen={}/{}",
        preset_id,
        file_name,
        vital_bytes.len(),
        generations_used,
        generations_limit
    );

    // ── Step 8: Return ────────────────────────────────────────────────────────

    Ok(Json(GenerateResponse {
        preset_id,
        download_url,
        file_name,
        format,
        preset_data,
        generations_used,
        generations_limit,
    }))
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Generate a minimal silent audio buffer for text-only prompt mode.
fn generate_silence_bytes() -> Vec<u8> {
    vec![
        0xFF, 0xFB, 0x90, 0x00, // MPEG1, Layer3, 128kbps, 44100Hz, Stereo
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ]
}
