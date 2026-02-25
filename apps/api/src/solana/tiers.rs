/// Tier determination based on $AUDIO token balance.

use serde::Serialize;

/// User tier levels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserTier {
    Unauthenticated,
    Free,
    Pro,
    Studio,
}

/// Tier threshold constants (in $AUDIO tokens).
pub const TIER_THRESHOLD_PRO: f64 = 100.0;
pub const TIER_THRESHOLD_STUDIO: f64 = 1000.0;

/// Daily generation limits per tier.
pub const LIMIT_UNAUTHENTICATED: u32 = 1;
pub const LIMIT_FREE: u32 = 3;
pub const LIMIT_PRO: u32 = 15;
pub const LIMIT_STUDIO: u32 = u32::MAX; // Unlimited

/// Determine user tier from $AUDIO balance.
pub fn tier_from_balance(balance: f64) -> UserTier {
    if balance >= TIER_THRESHOLD_STUDIO {
        UserTier::Studio
    } else if balance >= TIER_THRESHOLD_PRO {
        UserTier::Pro
    } else {
        UserTier::Free
    }
}

/// Get daily generation limit for a tier.
pub fn daily_limit(tier: &UserTier) -> u32 {
    match tier {
        UserTier::Unauthenticated => LIMIT_UNAUTHENTICATED,
        UserTier::Free => LIMIT_FREE,
        UserTier::Pro => LIMIT_PRO,
        UserTier::Studio => LIMIT_STUDIO,
    }
}
