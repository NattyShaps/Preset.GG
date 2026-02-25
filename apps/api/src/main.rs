mod config;
mod errors;
mod middleware;
mod preset;
mod routes;
mod services;
mod solana;

use tracing_subscriber::EnvFilter;

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

    // Build router with CORS
    let app = routes::create_router()
        .layer(middleware::cors::cors_layer(&config.cors_origin));

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("🎛️  Preset.gg API listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
