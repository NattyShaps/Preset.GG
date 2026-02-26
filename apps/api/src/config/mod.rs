use serde::Deserialize;

/// Application configuration, parsed from environment variables.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Gemini API key
    pub gemini_api_key: String,

    /// Supabase project URL
    pub supabase_url: String,

    /// Supabase service role key
    pub supabase_service_key: String,

    /// Solana RPC endpoint
    pub solana_rpc_url: String,

    /// $AUDIO SPL Token mint address
    pub audio_token_mint: String,

    /// Server port
    pub api_port: u16,

    /// Allowed CORS origin
    pub cors_origin: String,
}

impl AppConfig {
    /// Load configuration from environment variables.
    pub fn from_env() -> anyhow::Result<Self> {
        // Railway-style platforms provide PORT. Prefer it, then API_PORT, then default.
        let api_port = std::env::var("PORT")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .or_else(|| std::env::var("API_PORT").ok().filter(|v| !v.trim().is_empty()))
            .unwrap_or_else(|| "3001".to_string())
            .parse()?;

        Ok(Self {
            gemini_api_key: std::env::var("GEMINI_API_KEY")?,
            supabase_url: std::env::var("SUPABASE_URL")?,
            supabase_service_key: std::env::var("SUPABASE_SERVICE_KEY")?,
            solana_rpc_url: std::env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
            audio_token_mint: std::env::var("AUDIO_TOKEN_MINT")
                .unwrap_or_else(|_| "9LzCMqDgTKYz9Drzqnpgee3SGa89up3a247ypMj2xrqM".to_string()),
            api_port,
            cors_origin: std::env::var("CORS_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
        })
    }
}
