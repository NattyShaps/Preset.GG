pub mod auth;
pub mod generate;
pub mod health;
pub mod presets;
pub mod stream;

use axum::{routing::{get, post}, Router};

use crate::AppState;

/// Build the complete API router with all route groups.
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health
        .route("/health", get(health::health_check))
        // Audio stream proxy
        .route("/api/stream/{track_id}", get(stream::stream_track))
        // Generation
        .route("/api/generate", post(generate::generate_preset))
        // Presets
        .route("/api/presets", get(presets::list_presets))
        .route("/api/presets/{id}/download", get(presets::download_preset))
        // Auth
        .route("/api/auth/verify", post(auth::verify_wallet))
        .with_state(state)
}
