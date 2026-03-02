/// POST /api/generate — Main preset generation endpoint.
///
/// Pipeline:
///   1. Validate request
///   2. Fetch audio from Audius (if track_id provided)
///   3. Enforce audio size budget
///   4. Send audio + prompt to Gemini
///   5. Merge Gemini output into Init.vital template
///   6. Return preset file inline as base64 (direct download mode)
///   7. [M3 TODO] Upload to Supabase Storage + record in DB

use axum::{extract::State, Json};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::{AppConfig, AUDIUS_FETCH_TIMEOUT_SECS, GEMINI_REQUEST_TIMEOUT_SECS, MAX_AUDIO_SIZE_BYTES};
use crate::errors::AppError;
use crate::preset::vital::merge_gemini_into_template;
use crate::services::{audius, gemini};
use crate::solana::{tiers, token_gate};

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
    /// Wallet public key for tier checking (M4)
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
}

// ── Handler ───────────────────────────────────────────────────────────────────

pub async fn generate_preset(
    State(config): State<AppConfig>,
    Json(payload): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, AppError> {

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
        "Generate request: prompt={:?}, track_id={:?}, format={}, wallet={:?}",
        prompt,
        track_id,
        format,
        payload.wallet_pubkey
    );

    // ── Tier enforcement ──────────────────────────────────────────────────────

    if let Some(ref pubkey) = payload.wallet_pubkey {
        let pubkey = pubkey.trim();
        if !pubkey.is_empty() {
            // Authenticated user: check $AUDIO balance and tier.
            let balance = token_gate::get_audio_balance(
                &config.solana_rpc_url,
                pubkey,
                &config.audio_token_mint,
            )
            .unwrap_or(0.0);

            let tier = tiers::tier_from_balance(balance);
            let limit = tiers::daily_limit(&tier);

            // [STUBBED] Daily count from Supabase — returns 0 until M3.
            let daily_used: u32 = 0;

            if daily_used >= limit {
                return Err(AppError::RateLimited(format!(
                    "Daily generation limit reached ({}/{}). Hold more $AUDIO to unlock higher tiers.",
                    daily_used, limit
                )));
            }

            // Serum format requires Pro tier or above.
            if format == "fxp" && tier != tiers::UserTier::Pro && tier != tiers::UserTier::Studio {
                return Err(AppError::Forbidden(
                    "Serum .fxp export requires Pro tier (hold 100+ $AUDIO).".to_string(),
                ));
            }

            tracing::info!(
                "Tier check passed: wallet={}, tier={:?}, used={}/{}",
                pubkey, tier, daily_used, limit
            );
        }
    } else {
        // Unauthenticated user: IP-based rate limiting.
        // [STUBBED] Until Supabase is wired up, allow all requests.
        // M3 TODO: extract IP, query Supabase, enforce LIMIT_UNAUTHENTICATED.
        tracing::info!("No wallet provided — unauthenticated request (rate limiting stubbed)");
    }

    // ── Step 2: Fetch audio from Audius ───────────────────────────────────────

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
        // Send a short silent audio clip to satisfy the multimodal requirement.
        // [M3 TODO] Decide if text-only mode skips audio entirely or uses a stub.
        tracing::info!("No track_id provided — text-only prompt mode");
        (generate_silence_bytes(), None)
    };

    // ── Step 3: Audio size check (belt + suspenders) ──────────────────────────

    if audio_bytes.len() > MAX_AUDIO_SIZE_BYTES {
        return Err(AppError::BadRequest(
            "Audio too large. Please select a shorter region using the focus window.".to_string(),
        ));
    }

    // ── Step 4: Send to Gemini ────────────────────────────────────────────────

    tracing::info!(
        "Sending {} bytes of audio to Gemini (prompt: {:?})",
        audio_bytes.len(),
        prompt
    );

    let gemini_output = gemini::generate_preset_json(
        &config.gemini_api_key,
        &audio_bytes,
        &prompt,
        payload.start_timestamp,
        payload.end_timestamp,
        GEMINI_REQUEST_TIMEOUT_SECS,
    )
    .await
    .map_err(|e| {
        AppError::Internal(format!("AI generation failed: {}. Try a different prompt or track.", e))
    })?;

    // ── Step 5: Merge into Init.vital template ────────────────────────────────

    let vital_bytes = merge_gemini_into_template(
        &gemini_output,
        &prompt,
        track_title.as_deref(),
    )
    .map_err(|e| AppError::Internal(format!("Preset generation failed: {}", e)))?;

    tracing::info!("Generated .vital file: {} bytes", vital_bytes.len());

    // ── Step 6: Encode file for direct download ──────────────────────────────

    let preset_data = BASE64.encode(&vital_bytes);

    let preset_id = Uuid::new_v4().to_string();
    let file_name = format!("preset_{}.vital", &preset_id[..8]);

    // [M3 TODO] Upload vital_bytes to Supabase Storage, record in DB.
    let download_url = format!("/api/presets/{}/download", preset_id);

    tracing::info!(
        "Generation complete. preset_id={}, file={}, size={}B",
        preset_id,
        file_name,
        vital_bytes.len()
    );

    // ── Step 7: Return ────────────────────────────────────────────────────────

    Ok(Json(GenerateResponse {
        preset_id,
        download_url,
        file_name,
        format,
        preset_data,
    }))
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Generate a minimal silent audio buffer for text-only prompt mode.
/// This is a valid but silent MP3 frame that Gemini will accept.
///
/// [M3 TODO] Decide whether text-only mode should be supported long-term.
/// For now this lets us test the pipeline without a track.
fn generate_silence_bytes() -> Vec<u8> {
    // Minimal valid MP3 frame: ID3 header + one silent frame.
    // This is enough for Gemini to accept the request without audio context.
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
