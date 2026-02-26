/**
 * App-wide constants.
 */

export const APP_NAME = 'Preset.gg';

export const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3001';

// Audius API
export const AUDIUS_API_BASE = 'https://api.audius.co/v1';
export const AUDIUS_API_KEY = import.meta.env.VITE_AUDIUS_API_KEY || '';
export const AUDIUS_SEARCH_LIMIT = 8;
export const AUDIUS_DEBOUNCE_MS = 300;

export const TIER_THRESHOLDS = {
  free: 0,
  pro: 100,
  studio: 1000,
} as const;

export const TIER_LIMITS = {
  unauthenticated: 1,
  free: 3,
  pro: 15,
  studio: Infinity,
} as const;

export const SUPPORTED_FORMATS = ['vital', 'fxp'] as const;
export type PresetFormat = (typeof SUPPORTED_FORMATS)[number];
