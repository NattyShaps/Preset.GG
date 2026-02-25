/**
 * API request/response shape definitions.
 */

export interface ApiError {
  message: string;
  code?: string;
  statusCode: number;
}

export interface ApiResponse<T> {
  data: T;
  success: boolean;
}

export interface HealthCheckResponse {
  status: 'ok';
  version: string;
  uptime: number;
}

export interface AuthVerifyRequest {
  walletPubkey: string;
  signature: string;
  message: string;
}

export interface AuthVerifyResponse {
  tier: string;
  audioBalance: number;
  dailyGenerationsUsed: number;
  dailyGenerationsLimit: number;
}
