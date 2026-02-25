use anchor_lang::prelude::*;

declare_id!("8PeG7KHhJ5dyLWFTRZuPGwcQ7xvkKaUTN7F9MKgACnAc");

/// Preset.gg — On-chain Program (Stub)
///
/// This program is scaffolded for future on-chain functionality.
/// For the hackathon MVP, token gating is done off-chain via RPC queries.
///
/// ## Post-Hackathon Roadmap:
/// - On-chain generation receipts (proof that a preset was generated)
/// - Staking $AUDIO for tier upgrades
/// - NFT preset minting (preset marketplace)
/// - On-chain access control (program-level token gate)
#[program]
pub mod preset_gate {
    use super::*;

    /// Initialize the program state.
    /// Placeholder for future program initialization logic.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Preset.gg program initialized: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
