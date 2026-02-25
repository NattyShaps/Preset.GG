/**
 * Wallet and tier type definitions.
 */

export type UserTier = 'unauthenticated' | 'free' | 'pro' | 'studio';

export interface WalletState {
  publicKey: string | null;
  isConnected: boolean;
  isConnecting: boolean;
}

export interface TierInfo {
  tier: UserTier;
  audioBalance: number;
  dailyGenerations: {
    used: number;
    limit: number;
  };
}

export const TIER_THRESHOLDS = {
  free: 0,
  pro: 100,
  studio: 1000,
} as const;

export const TIER_LABELS: Record<UserTier, string> = {
  unauthenticated: 'Free',
  free: 'Basic',
  pro: 'Pro',
  studio: 'Studio',
};
