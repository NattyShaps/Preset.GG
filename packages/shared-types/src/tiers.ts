/**
 * Tier thresholds and limits — shared between frontend and backend.
 */

export type UserTier = 'unauthenticated' | 'free' | 'pro' | 'studio';

export const TIER_THRESHOLDS: Record<string, number> = {
  free: 0,
  pro: 100,
  studio: 1000,
};

export const TIER_DAILY_LIMITS: Record<UserTier, number> = {
  unauthenticated: 1,
  free: 3,
  pro: 15,
  studio: Infinity,
};

export const TIER_LABELS: Record<UserTier, string> = {
  unauthenticated: 'Free',
  free: 'Basic',
  pro: 'Pro',
  studio: 'Studio',
};

export function getTierFromBalance(balance: number): UserTier {
  if (balance >= TIER_THRESHOLDS.studio) return 'studio';
  if (balance >= TIER_THRESHOLDS.pro) return 'pro';
  return 'free';
}
