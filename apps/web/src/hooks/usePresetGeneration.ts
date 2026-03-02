/**
 * Hook for generating presets via the Rust backend.
 *
 * Calls POST /api/generate and returns the result including
 * base64-encoded .vital file bytes for direct client-side download.
 */
import { useState } from 'react';
import { apiClient } from '@/lib/api-client';
import type { GenerateRequest, GenerateResponse } from '@/types/api';

export interface GenerationResult {
  presetId: string;
  downloadUrl: string;
  fileName: string;
  format: 'vital' | 'fxp';
  /** Base64-encoded .vital file bytes */
  presetData: string;
}

export interface GenerateParams {
  prompt?: string;
  trackId?: string;
  startTimestamp?: number;
  endTimestamp?: number;
  format?: string;
  walletPubkey?: string;
}

export function usePresetGeneration() {
  const [isGenerating, setIsGenerating] = useState(false);
  const [result, setResult] = useState<GenerationResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const generate = async (params: GenerateParams) => {
    setIsGenerating(true);
    setError(null);
    setResult(null);

    try {
      // Map camelCase params to snake_case for the backend.
      const body: GenerateRequest = {
        prompt: params.prompt,
        track_id: params.trackId,
        start_timestamp: params.startTimestamp,
        end_timestamp: params.endTimestamp,
        format: params.format || 'vital',
        wallet_pubkey: params.walletPubkey,
      };

      const response = await apiClient.post<GenerateResponse>(
        '/api/generate',
        body,
      );

      setResult({
        presetId: response.preset_id,
        downloadUrl: response.download_url,
        fileName: response.file_name,
        format: response.format as 'vital' | 'fxp',
        presetData: response.preset_data,
      });
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Generation failed';

      // Map backend errors to user-friendly messages.
      if (message.includes('too large')) {
        setError('Audio too large. Try selecting a shorter region with the focus window.');
      } else if (message.includes('limit') || message.includes('rate') || message.includes('Rate')) {
        setError('Generation limit reached. Connect your wallet and hold $AUDIO for more.');
      } else if (message.includes('AI generation failed') || message.includes('Gemini')) {
        setError('AI couldn\'t analyze this sound. Try a different prompt or track.');
      } else if (message.includes('fetch') || message.includes('network') || message.includes('Failed to fetch')) {
        setError('Could not reach the server. Please check your connection and try again.');
      } else {
        setError(message);
      }
    } finally {
      setIsGenerating(false);
    }
  };

  const reset = () => {
    setResult(null);
    setError(null);
  };

  return { isGenerating, result, error, generate, reset };
}
