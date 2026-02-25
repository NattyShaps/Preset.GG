/**
 * Solana connection and wallet adapter configuration.
 * TODO: Set up wallet adapter providers
 */

// $AUDIO SPL Token Mint Address (Solana Mainnet)
export const AUDIO_TOKEN_MINT = '9LzCMqDgTKYz9Drzqnpgee3SGa89up3a247ypMj2xrqM';

// Solana RPC endpoint
export const SOLANA_RPC_URL =
  import.meta.env.VITE_SOLANA_RPC_URL || 'https://api.mainnet-beta.solana.com';
