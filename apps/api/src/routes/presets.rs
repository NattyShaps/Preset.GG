/// Preset listing and download endpoints.
///
/// GET /api/presets — List user's generation history
/// GET /api/presets/:id/download — Download a preset file
///
/// [STUBBED] Both endpoints return placeholder data until Supabase is wired up.

use axum::Json;
use axum::http::HeaderMap;
use serde::Serialize;

use crate::errors::AppError;

// ── Response types ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct PresetListItem {
    pub id: String,
    pub name: String,
    pub format: String,
    pub prompt: String,
    pub download_url: String,
    pub created_at: String,
}

// ── Handlers ──────────────────────────────────────────────────────────────────

/// GET /api/presets — List user's preset generation history.
///
/// Reads wallet pubkey from `X-Wallet-Pubkey` header.
/// [STUBBED] Returns empty array until Supabase is wired up.
pub async fn list_presets(
    headers: HeaderMap,
) -> Result<Json<Vec<PresetListItem>>, AppError> {
    let wallet = headers
        .get("X-Wallet-Pubkey")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if wallet.is_empty() {
        return Err(AppError::Unauthorized(
            "Connect your wallet to view preset history.".to_string(),
        ));
    }

    tracing::info!("List presets for wallet: {}", wallet);

    // [STUBBED] Query Supabase `generations` table filtered by wallet.
    // M3 TODO: Implement real query via services::supabase::get_user_presets()
    Ok(Json(vec![]))
}

/// GET /api/presets/:id/download — Download a specific preset file.
///
/// [STUBBED] Returns 404 until Supabase Storage is wired up.
/// M3 TODO: Look up generation record in Supabase, return redirect to Storage URL.
pub async fn download_preset(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    tracing::info!("Download preset: {}", id);

    // [STUBBED] In M3 this will:
    // 1. Look up the generation record in Supabase by preset_id
    // 2. Return a redirect to the Supabase Storage public URL
    // 3. Or stream the file directly with Content-Disposition header
    Err(AppError::NotFound(format!(
        "Preset {} not found. Supabase storage not yet connected.",
        id
    )))
}
