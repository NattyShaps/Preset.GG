use axum::Json;
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

/// Request body for POST /api/generate
#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub prompt: String,
    pub track_id: Option<String>,
    pub track_url: Option<String>,
    pub start_timestamp: Option<f64>,
    pub end_timestamp: Option<f64>,
    pub format: Option<String>, // "vital" or "fxp"
    pub wallet_pubkey: Option<String>,
}

/// Response body for POST /api/generate
#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub preset_id: String,
    pub download_url: String,
    pub file_name: String,
    pub format: String,
}

/// POST /api/generate — Main preset generation endpoint.
pub async fn generate_preset(
    Json(payload): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, AppError> {
    // TODO: Implementation steps:
    // 1. Validate request
    // 2. Check wallet tier & daily limits (via solana module + supabase)
    // 3. Fetch audio from Audius (via services::audius)
    // 4. Send audio + prompt to Gemini (via services::gemini)
    // 5. Parse Gemini JSON response into preset format (via preset module)
    // 6. Upload preset file to Supabase Storage (via services::supabase)
    // 7. Record generation in Supabase DB
    // 8. Return download URL

    tracing::info!("Generate request: prompt={}", payload.prompt);

    Err(AppError::Internal("Not yet implemented".to_string()))
}
