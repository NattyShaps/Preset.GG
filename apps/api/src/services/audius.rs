/// Audius API integration — server-side audio fetching for the Gemini pipeline.
///
/// Note: The browser-facing stream proxy lives in routes/stream.rs.
/// This module fetches full audio bytes server-side for Gemini analysis.

use anyhow::{anyhow, Result};
use std::time::Duration;

/// Fetch the full audio bytes for a track from Audius.
///
/// Calls `GET https://api.audius.co/v1/tracks/{trackId}/stream`, follows the
/// 302 redirect server-side, and reads the response body into memory.
///
/// Returns an error if the response body exceeds `max_bytes` (audio too large)
/// or if the request fails.
pub async fn fetch_audio(
    track_id: &str,
    api_key: &str,
    timeout_secs: u64,
    max_bytes: usize,
) -> Result<Vec<u8>> {
    let url = format!(
        "https://api.audius.co/v1/tracks/{}/stream",
        track_id
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()?;

    let mut req = client.get(&url);
    if !api_key.is_empty() {
        req = req.header("x-api-key", api_key);
    }

    let resp = req.send().await?;

    if !resp.status().is_success() {
        return Err(anyhow!(
            "Audius returned status {} for track {}",
            resp.status(),
            track_id
        ));
    }

    // Read body in chunks, enforcing the size cap.
    let mut bytes: Vec<u8> = Vec::new();
    let mut stream = resp.bytes_stream();

    use futures_util::StreamExt;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        bytes.extend_from_slice(&chunk);
        if bytes.len() > max_bytes {
            return Err(anyhow!(
                "Audio too large: exceeded {} bytes. Select a shorter region.",
                max_bytes
            ));
        }
    }

    if bytes.is_empty() {
        return Err(anyhow!("Audius returned empty audio for track {}", track_id));
    }

    tracing::info!(
        "Fetched audio for track {}: {} bytes ({:.1} MB)",
        track_id,
        bytes.len(),
        bytes.len() as f64 / 1_048_576.0
    );

    Ok(bytes)
}

/// Resolve a track ID to its final signed CDN stream URL.
///
/// Follows the Audius 302 redirect and returns the final URL.
/// Useful when you need the URL itself rather than the bytes.
pub async fn resolve_track_stream_url(
    track_id: &str,
    api_key: &str,
    timeout_secs: u64,
) -> Result<String> {
    let url = format!(
        "https://api.audius.co/v1/tracks/{}/stream",
        track_id
    );

    // Build a client that does NOT follow redirects so we can capture the Location header.
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(Duration::from_secs(timeout_secs))
        .build()?;

    let mut req = client.get(&url);
    if !api_key.is_empty() {
        req = req.header("x-api-key", api_key);
    }

    let resp = req.send().await?;

    // Expect a 302 redirect.
    if resp.status().is_redirection() {
        let location = resp
            .headers()
            .get("location")
            .ok_or_else(|| anyhow!("Audius 302 response missing Location header"))?
            .to_str()?
            .to_string();
        return Ok(location);
    }

    // Some responses may be 200 directly from a content node.
    if resp.status().is_success() {
        return Ok(url);
    }

    Err(anyhow!(
        "Unexpected Audius response status: {}",
        resp.status()
    ))
}
