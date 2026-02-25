pub mod auth;
pub mod generate;
pub mod health;
pub mod presets;

use axum::{routing::{get, post}, Router};

/// Build the complete API router with all route groups.
pub fn create_router() -> Router {
    Router::new()
        // Health
        .route("/health", get(health::health_check))
        // Generation
        .route("/api/generate", post(generate::generate_preset))
        // Presets
        .route("/api/presets", get(presets::list_presets))
        .route("/api/presets/{id}/download", get(presets::download_preset))
        // Auth
        .route("/api/auth/verify", post(auth::verify_wallet))
}
