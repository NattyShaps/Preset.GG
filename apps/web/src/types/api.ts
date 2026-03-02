/**
 * API request/response shape definitions.
 */

export interface ApiError {
  error: string;
  statusCode: number;
}

export interface HealthCheckResponse {
  status: 'ok';
  version: string;
  uptime: number;
}

// ── Generate ──────────────────────────────────────────────────────────────────

export interface GenerateRequest {
  prompt?: string;
  track_id?: string;
  start_timestamp?: number;
  end_timestamp?: number;
  format?: string;
  wallet_pubkey?: string;
}

export interface GenerateResponse {
  preset_id: string;
  download_url: string;
  file_name: string;
  format: string;
  /** Base64-encoded .vital file bytes for direct client-side download. */
  preset_data: string;
}

// ── Auth ──────────────────────────────────────────────────────────────────────

export interface AuthVerifyRequest {
  wallet_pubkey: string;
}

export interface AuthVerifyResponse {
  tier: string;
  audio_balance: number;
  daily_generations_used: number;
  daily_generations_limit: number;
}
