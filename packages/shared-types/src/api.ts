/**
 * Shared API type definitions.
 */

import type { PresetFormat } from './preset-schema';

export interface GenerateRequest {
  prompt: string;
  trackId?: string;
  trackUrl?: string;
  startTimestamp?: number;
  endTimestamp?: number;
  format: PresetFormat;
  walletPubkey?: string;
}

export interface GenerateResponse {
  presetId: string;
  downloadUrl: string;
  fileName: string;
  format: PresetFormat;
  createdAt: string;
}
