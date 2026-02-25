# Preset Gate — Solana On-Chain Program

> **Status:** Stub — scaffolded for the Solana Graveyard Hackathon

This Anchor program is a placeholder for future on-chain functionality.

## Current MVP Approach
Token gating is done **off-chain** in the Rust API backend:
- The API queries the user's Solana wallet for `$AUDIO` SPL token balance via RPC
- Tier (Free / Pro / Studio) is determined based on balance thresholds
- Daily generation limits are enforced via Supabase

## Post-Hackathon On-Chain Roadmap
- **Generation Receipts:** Mint a PDA-based record each time a preset is generated (on-chain proof)
- **$AUDIO Staking:** Allow users to stake $AUDIO into a vault for tier upgrades
- **NFT Preset Marketplace:** Mint generated presets as NFTs (Metaplex)
- **Program-Level Token Gate:** Move tier verification on-chain for trustless access control

## Build
```bash
anchor build
```

## Test
```bash
anchor test
```
