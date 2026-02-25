use axum::Json;
use serde_json::{json, Value};

/// GET /health — Basic health check endpoint.
pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "preset-gg-api",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}
