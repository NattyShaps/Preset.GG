mod config;
mod errors;
mod middleware;
mod preset;
mod rate_limit;
mod routes;
mod services;
mod solana;

use std::net::SocketAddr;
use std::sync::Arc;

use tracing_subscriber::EnvFilter;

use crate::config::AppConfig;
use crate::rate_limit::RateLimiter;

/// Shared application state passed to all route handlers.
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub rate_limiter: Arc<RateLimiter>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize tracing/logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Load configuration
    let config = config::AppConfig::from_env()?;
    let addr = format!("0.0.0.0:{}", config.api_port);

    tracing::info!("Starting Preset.gg API server on {}", addr);

    // Build shared state
    let state = AppState {
        config: config.clone(),
        rate_limiter: Arc::new(RateLimiter::new()),
    };

    // Build router with CORS
    let app = routes::create_router(state)
        .layer(middleware::cors::cors_layer(&config.cors_origin));

    // Start server (with ConnectInfo for IP extraction)
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("🎛️  Preset.gg API listening on http://{}", addr);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
