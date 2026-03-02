/// POST /api/auth/verify — Verify wallet and return user tier info.
///
/// HACKATHON MVP TRUST MODEL:
/// No cryptographic signature verification (SIWS). The frontend-provided
/// publicKey is accepted at face value. This means tier spoofing is
/// theoretically possible — acceptable for a demo. SIWS is planned as the
/// first post-hackathon security hardening task.

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;
use crate::errors::AppError;
use crate::solana::{tiers, token_gate};

// ── Request / Response ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub wallet_pubkey: String,
    // HACKATHON MVP: signature and message fields removed.
    // Post-hackathon: add signature: String, message: String for SIWS.
}

#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    pub tier: String,
    pub audio_balance: f64,
    pub daily_generations_used: u32,
    pub daily_generations_limit: u32,
}

// ── Handler ───────────────────────────────────────────────────────────────────

pub async fn verify_wallet(
    State(config): State<AppConfig>,
    Json(payload): Json<VerifyRequest>,
) -> Result<Json<VerifyResponse>, AppError> {
    let pubkey = payload.wallet_pubkey.trim();

    if pubkey.is_empty() {
        return Err(AppError::BadRequest("wallet_pubkey is required".to_string()));
    }

    tracing::info!("Verifying wallet: {}", pubkey);

    // ── Step 1: Query $AUDIO balance from Solana ──────────────────────────────

    let balance = token_gate::get_audio_balance(
        &config.solana_rpc_url,
        pubkey,
        &config.audio_token_mint,
    )
    .map_err(|e| {
        tracing::error!("Failed to query $AUDIO balance for {}: {}", pubkey, e);
        AppError::Internal(format!("Could not verify wallet balance: {}", e))
    })?;

    // ── Step 2: Determine tier ────────────────────────────────────────────────

    let tier = tiers::tier_from_balance(balance);
    let limit = tiers::daily_limit(&tier);

    tracing::info!(
        "Wallet {} — balance: {:.2} $AUDIO, tier: {:?}, limit: {}",
        pubkey,
        balance,
        tier,
        limit
    );

    // ── Step 3: Daily generation count ────────────────────────────────────────
    // [STUBBED] Until Supabase is wired up, return 0 used.
    let daily_used: u32 = 0;

    // ── Step 4: Return ────────────────────────────────────────────────────────

    Ok(Json(VerifyResponse {
        tier: serde_json::to_value(&tier)
            .ok()
            .and_then(|v| v.as_str().map(String::from))
            .unwrap_or_else(|| "free".to_string()),
        audio_balance: balance,
        daily_generations_used: daily_used,
        daily_generations_limit: limit,
    }))
}
