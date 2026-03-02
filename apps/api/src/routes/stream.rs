/// GET /api/stream/:track_id
///
/// Proxies an Audius track audio stream to the client.
/// Follows the Audius 302 redirect server-side and pipes the audio bytes
/// back without buffering — suitable for both browser playback and the
/// Gemini pipeline in M2.

use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use std::time::Duration;

use crate::config::AppConfig;

pub async fn stream_track(
    State(config): State<AppConfig>,
    Path(track_id): Path<String>,
) -> Result<Response<Body>, StatusCode> {
    let upstream_url = format!(
        "https://api.audius.co/v1/tracks/{}/stream",
        track_id
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| {
            tracing::error!("Failed to build reqwest client: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let mut request = client.get(&upstream_url);
    if !config.audius_api_key.is_empty() {
        request = request.header("x-api-key", &config.audius_api_key);
    }

    let upstream = request.send().await.map_err(|e| {
        tracing::error!("Audius stream request failed for track {}: {}", track_id, e);
        StatusCode::BAD_GATEWAY
    })?;

    let status = upstream.status();
    if !status.is_success() {
        tracing::warn!(
            "Audius returned non-success status {} for track {}",
            status,
            track_id
        );
        return Err(
            StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY)
        );
    }

    // Stream bytes back without buffering into memory
    let stream = upstream.bytes_stream();
    let body = Body::from_stream(stream);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "audio/mpeg")
        .header("Access-Control-Allow-Origin", "*")
        .header("Accept-Ranges", "bytes")
        .body(body)
        .map_err(|e| {
            tracing::error!("Failed to build response: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(response)
}
