use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Mutex;
use std::time::Instant;

use crate::errors::AppError;

/// Maximum free generations per IP per 24-hour window.
const FREE_DAILY_LIMIT: u32 = 5;

/// Rolling window size in seconds (24 hours).
const WINDOW_SECS: u64 = 86400;

/// Simple in-memory IP-based rate limiter.
///
/// Tracks generation timestamps per IP address and enforces a rolling
/// 24-hour window limit. Entries older than 24h are pruned on each check.
///
/// NOTE: State is lost on server restart. This is intentional for MVP —
/// a persistent store (Supabase) will replace this in a later milestone.
pub struct RateLimiter {
    entries: Mutex<HashMap<IpAddr, Vec<Instant>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
        }
    }

    /// Check if the given IP is under the rate limit, and record a new
    /// generation if so. Returns `(used_after_this_request, limit)`.
    ///
    /// Returns `AppError::RateLimited` if the IP has exhausted its quota.
    pub fn check_and_record(&self, ip: IpAddr) -> Result<(u32, u32), AppError> {
        let mut map = self.entries.lock().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        let cutoff = now - std::time::Duration::from_secs(WINDOW_SECS);

        let timestamps = map.entry(ip).or_insert_with(Vec::new);

        // Prune entries older than 24h.
        timestamps.retain(|t| *t > cutoff);

        let used = timestamps.len() as u32;
        if used >= FREE_DAILY_LIMIT {
            return Err(AppError::RateLimited(format!(
                "Daily generation limit reached ({}/{}). Try again in 24 hours.",
                used, FREE_DAILY_LIMIT
            )));
        }

        // Record this generation.
        timestamps.push(now);

        Ok((used + 1, FREE_DAILY_LIMIT))
    }
}
