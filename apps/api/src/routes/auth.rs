use axum::Json;
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

/// Request body for POST /api/auth/verify
#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub wallet_pubkey: String,
    pub signature: String,
    pub message: String,
}

/// Response body for POST /api/auth/verify
#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    pub tier: String,
    pub audio_balance: f64,
    pub daily_generations_used: u32,
    pub daily_generations_limit: u32,
}

/// POST /api/auth/verify — Verify wallet signature and return user tier.
pub async fn verify_wallet(
    Json(payload): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, AppError> {
    // TODO: Implementation steps:
    // 1. Verify the wallet signature against the message
    // 2. Query Solana RPC for $AUDIO balance (via solana::token_gate)
    // 3. Determine tier (via solana::tiers)
    // 4. Check daily generation count from Supabase
    // 5. Return tier info

    tracing::info!("Verify wallet: {}", payload.wallet_pubkey);

    Err(AppError::Internal("Not yet implemented".to_string()))
}
