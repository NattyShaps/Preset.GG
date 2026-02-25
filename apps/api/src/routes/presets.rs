use axum::Json;
use serde::Serialize;

use crate::errors::AppError;

/// Response for a single preset in the list.
#[derive(Debug, Serialize)]
pub struct PresetListItem {
    pub id: String,
    pub name: String,
    pub format: String,
    pub prompt: String,
    pub download_url: String,
    pub created_at: String,
}

/// GET /api/presets — List user's preset generation history.
pub async fn list_presets() -> Result<Json<Vec<PresetListItem>>, AppError> {
    // TODO: Query Supabase for user's presets based on wallet pubkey from headers
    tracing::info!("List presets requested");
    Ok(Json(vec![]))
}

/// GET /api/presets/:id/download — Download a specific preset file.
pub async fn download_preset(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    // TODO: Fetch preset file from Supabase Storage and return as download
    tracing::info!("Download preset: {}", id);
    Err(AppError::NotFound(format!("Preset {} not found", id)))
}
