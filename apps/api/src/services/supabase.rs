/// Supabase integration.
///
/// Handles database CRUD operations and file storage.

use anyhow::Result;

/// Record a generation in the database.
pub async fn record_generation(
    _supabase_url: &str,
    _supabase_key: &str,
    _wallet_pubkey: &str,
    _preset_id: &str,
    _prompt: &str,
) -> Result<()> {
    // TODO: Insert a row into the `generations` table
    anyhow::bail!("Supabase record_generation not yet implemented")
}

/// Get the number of generations a user has made today.
pub async fn get_daily_generation_count(
    _supabase_url: &str,
    _supabase_key: &str,
    _wallet_pubkey: &str,
) -> Result<u32> {
    // TODO: Query the `generations` table for today's count
    anyhow::bail!("Supabase get_daily_generation_count not yet implemented")
}

/// Upload a preset file to Supabase Storage.
pub async fn upload_preset_file(
    _supabase_url: &str,
    _supabase_key: &str,
    _file_name: &str,
    _file_data: &[u8],
) -> Result<String> {
    // TODO: Upload to the `presets` storage bucket and return the public URL
    anyhow::bail!("Supabase upload not yet implemented")
}

/// Get a user's preset generation history.
pub async fn get_user_presets(
    _supabase_url: &str,
    _supabase_key: &str,
    _wallet_pubkey: &str,
) -> Result<Vec<serde_json::Value>> {
    // TODO: Query the `generations` table for the user's history
    anyhow::bail!("Supabase get_user_presets not yet implemented")
}
