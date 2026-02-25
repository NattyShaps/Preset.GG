/**
 * Vital preset JSON schema type definitions.
 */

export interface VitalPreset {
  preset_name: string;
  author: string;
  comments: string;
  [key: string]: unknown; // Flattened synthesis parameters
}

export type PresetFormat = 'vital' | 'fxp';

export const SUPPORTED_FORMATS: PresetFormat[] = ['vital', 'fxp'];
