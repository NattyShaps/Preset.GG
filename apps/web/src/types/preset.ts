/**
 * Preset generation type definitions.
 */

export type PresetFormat = 'vital' | 'fxp';

export interface GenerationRequest {
  prompt: string;
  trackId?: string;
  trackUrl?: string;
  startTimestamp?: number;
  endTimestamp?: number;
  format: PresetFormat;
  walletPubkey?: string;
}

export interface GenerationResponse {
  presetId: string;
  downloadUrl: string;
  fileName: string;
  format: PresetFormat;
  createdAt: string;
}

export interface PresetMetadata {
  id: string;
  name: string;
  format: PresetFormat;
  prompt: string;
  sourceTrack?: string;
  sourceArtist?: string;
  downloadUrl: string;
  createdAt: string;
}
