/// CORS middleware configuration.

use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

/// Create CORS layer allowing the frontend origin.
pub fn cors_layer(origin: &str) -> CorsLayer {
    if origin == "*" {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers(Any)
    } else {
        CorsLayer::new()
            .allow_origin(origin.parse::<axum::http::HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers(Any)
    }
}
