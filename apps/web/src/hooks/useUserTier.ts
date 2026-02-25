/**
 * Hook for determining user tier based on $AUDIO token balance.
 * TODO: Wire up to backend tier check endpoint
 */
import { useState, useEffect } from 'react';
import { UserTier, TIER_THRESHOLDS } from '../types/wallet';

export function useUserTier(publicKey: string | null) {
  const [tier, setTier] = useState<UserTier>('unauthenticated');
  const [audioBalance, setAudioBalance] = useState<number>(0);
  const [dailyGenerations, setDailyGenerations] = useState({ used: 0, limit: 1 });

  useEffect(() => {
    if (!publicKey) {
      setTier('unauthenticated');
      setAudioBalance(0);
      setDailyGenerations({ used: 0, limit: 1 });
      return;
    }

    // TODO: Fetch from backend
    // const response = await apiClient.get(`/api/auth/tier?wallet=${publicKey}`);
    // setTier(response.tier);
    // setAudioBalance(response.audioBalance);
    // setDailyGenerations(response.dailyGenerations);

    // Mock: connected wallet with no tokens
    setTier('free');
    setAudioBalance(0);
    setDailyGenerations({ used: 0, limit: 3 });
  }, [publicKey]);

  return { tier, audioBalance, dailyGenerations };
}

export function getTierFromBalance(balance: number): UserTier {
  if (balance >= TIER_THRESHOLDS.studio) return 'studio';
  if (balance >= TIER_THRESHOLDS.pro) return 'pro';
  return 'free';
}
