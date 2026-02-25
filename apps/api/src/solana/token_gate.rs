/// Token gating — query $AUDIO SPL token balance for a Solana wallet.

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Query the $AUDIO token balance for a given wallet public key.
///
/// # Arguments
/// * `rpc_url` - Solana RPC endpoint URL
/// * `wallet_pubkey` - The wallet's public key (base58)
/// * `audio_mint` - The $AUDIO SPL token mint address (base58)
///
/// # Returns
/// The token balance as a f64 (adjusted for decimals).
pub fn get_audio_balance(
    rpc_url: &str,
    wallet_pubkey: &str,
    audio_mint: &str,
) -> Result<f64> {
    let client = RpcClient::new(rpc_url.to_string());
    let wallet = Pubkey::from_str(wallet_pubkey)?;
    let mint = Pubkey::from_str(audio_mint)?;

    // Derive the Associated Token Account (ATA) address
    let ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);

    // Fetch the token account balance
    match client.get_token_account_balance(&ata) {
        Ok(balance) => {
            let amount: f64 = balance.ui_amount.unwrap_or(0.0);
            Ok(amount)
        }
        Err(_) => {
            // Account doesn't exist = 0 balance
            Ok(0.0)
        }
    }
}
