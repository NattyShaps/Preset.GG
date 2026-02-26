# 🎛️ Preset.GG — Product Requirements Document (PRD)

> **AI-powered Audio-to-Preset Generator — The "Shazam for Synths"**
>
> Target Event: **Solana Graveyard Hackathon**
>
> Last Updated: February 25, 2026

---

## Table of Contents

- [Executive Summary](#executive-summary)
  - [The Vision](#the-vision)
  - [The Problem](#the-problem)
  - [The Solution](#the-solution)
  - [The Web3 / Solana Strategy](#the-web3--solana-strategy)
- [Project Overview](#project-overview)
- [Tech Stack](#tech-stack)
- [Core Features & Requirements](#core-features--requirements)
  - [3.1 Audius Search & Player](#31-audius-search--player)
  - [3.2 Prompting Engine](#32-prompting-engine)
  - [3.3 AI Audio-to-Preset Pipeline](#33-ai-audio-to-preset-pipeline)
  - [3.4 Solana Token Gating & Auth](#34-solana-token-gating--auth)
  - [3.5 User Dashboard](#35-user-dashboard)
- [User Flow](#user-flow)
- [System Architecture & Data Flow](#system-architecture--data-flow)
- [Token Gating Tiers](#token-gating-tiers)
- [Hackathon Milestones](#hackathon-milestones)
- [Out of Scope (Future Roadmap)](#out-of-scope-future-roadmap)

---

## Executive Summary

### The Vision

Preset.gg is the AI-powered "Shazam for Synths." It is a decentralized sound design copilot that allows music producers to search for any track on Audius, highlight a specific sound, and instantly generate a playable, royalty-free synthesizer preset (Vital or Xfer Serum).

### The Problem

**The Sound Design Bottleneck:** Recreating a specific synth sound by ear is a highly technical, time-consuming "dark art." Producers spend hours preset-surfing instead of composing.

**The Flaw of Audio Extraction:** Current AI tools (like stem splitters) output static `.WAV` files. Sampled audio is "baked" — you cannot change the filter speed, wavetable, or ADSR envelope without ruining the audio quality. Furthermore, sampling copyrighted audio carries legal risks.

**The AI Hallucination Gap:** Competitors like Muse.art attempt "Text-to-Preset" generation. Because they rely solely on text prompts (e.g., "make a Skrillex bass"), the AI guesses the parameters from memory, leading to inaccurate, generic sounds.

### The Solution

Preset.gg pioneers **Audio-to-Preset generation.** By integrating the permissionless Audius catalog, users can stream actual audio directly into Gemini 3.1's multimodal engine. The AI listens to the acoustic data, reverse-engineers the synthesis parameters, and outputs a mathematical recipe (a JSON-based preset file).

**The Result:** The user gets the exact sound they want, but as an infinitely customizable, royalty-free, playable MIDI instrument.

### The Web3 / Solana Strategy

Preset.gg utilizes a "Freemium-to-Web3" onboarding funnel to drive real utility for the Audius ecosystem on Solana.

- **The "Aha!" Moment (Free):** Users can generate 1 free Vital preset with zero friction (no wallet required).
- **The Utility Gate:** To unlock daily generation limits and premium formats (Xfer Serum `.fxp` exports), users must connect their Phantom/Backpack wallet.
- **Staking for Bandwidth:** Instead of a Web2 SaaS subscription, Preset.gg checks the user's Solana wallet for `$AUDIO` SPL tokens. Holding `$AUDIO` acts as a decentralized API key, unlocking higher tiers of AI compute.

---

## Project Overview

**Objective:** Build a functional Web dApp MVP that successfully takes an Audius track, processes a user prompt via Gemini 3.1, and outputs a working `.vital` (and experimental `.fxp`) preset file, gated by Solana token balances.

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | React, Vite, TailwindCSS v4 |
| Web3/Crypto | `@solana/wallet-adapter`, Solana Web3.js (RPC queries) |
| Audio Data | Audius JavaScript SDK |
| Backend | Rust (Axum) |
| Database & Storage | Supabase (PostgreSQL for user limits, Object Storage for preset files) |
| AI Engine | Google Gemini 3.1 Pro (Multimodal Audio + Text API) |
| On-Chain | Anchor Framework (stub for post-hackathon) |

---

## Core Features & Requirements

### 3.1 Audius Search & Player

- [ ] A search bar that queries the Audius API for tracks, artists, or playlists
- [ ] A waveform audio player that allows the user to scrub through the track
- [ ] Timestamp selection — user can highlight a specific range (e.g., `0:45 - 1:00`)

### 3.2 Prompting Engine

- [ ] A text input for the user to describe the sound (e.g., "Extract the heavy wubby bass")
- [ ] Quick-tag pills (`[Pluck]`, `[Pad]`, `[Lead]`, `[Bass]`, `[FX]`) to auto-fill context
- [ ] An "Enhance Prompt" toggle that uses a lightweight LLM call to translate layman terms into technical synthesis vocabulary before sending to the main audio model

### 3.3 AI Audio-to-Preset Pipeline

- [ ] Rust backend receives the Audius stream URL, timestamp range, and enhanced prompt
- [ ] Rust passes the audio buffer and prompt to the Gemini 3.1 API
- [ ] System prompt directs Gemini to output **only** a strictly formatted JSON object matching the Vital synthesizer schema
- [ ] Rust parses the JSON, validates against schema, and saves as a `.vital` file (gzipped JSON)
- [ ] Upload preset file to Supabase Storage bucket
- [ ] Return download URL to frontend

### 3.4 Solana Token Gating & Auth

- [ ] Integration of Phantom/Backpack wallets via `@solana/wallet-adapter`
- [ ] Rust backend queries Solana RPC node to check connected wallet's `$AUDIO` token balance
- [ ] Tier logic enforced per daily generation limits (see [Token Gating Tiers](#token-gating-tiers))
- [ ] Unauthenticated usage tracked via IP / Local Storage

### 3.5 User Dashboard

- [ ] A "My Presets" tab where authenticated users can view their generation history
- [ ] Re-download previously generated `.vital` or `.fxp` files
- [ ] Display remaining daily generation count

---

## User Flow

> The "Magic Moment"

```
1. LANDING
   User visits preset.gg. No login required.

2. SEARCH
   User searches "Skrillex" and selects a track from the Audius results.

3. ISOLATE
   User highlights the drop section on the waveform and types "Main FM Bass".

4. GENERATE
   User clicks "Generate Preset."

5. DELIVERY
   A loading state appears. Within ~10 seconds, a success screen shows
   a "Download .vital" button.

6. THE HOOK
   User drags the file into Vital in their DAW and plays the exact sound.

7. THE GATE
   User tries to generate a second sound. A modal appears:
   "Connect your Solana wallet and hold $AUDIO to unlock more
   generations and Serum exports."
```

---

## System Architecture & Data Flow

```
[ React/Vite Frontend ]
       |
       |── 1. Search/Stream ─────────→ [ Audius API ]
       |
       |── 2. Connect Wallet ────────→ [ Solana Wallet Adapter ]
       |
       |── 3. Send Prompt, Audio URL,
       |      Wallet PubKey ─────────→ [ Rust Backend (Axum) ]
                                            |
                                            |── 4. Verify $AUDIO Balance ──→ [ Solana RPC Node ]
                                            |
                                            |── 5. Check Daily Limits ─────→ [ Supabase DB ]
                                            |
                                            |── 6. Stream Audio + Prompt ──→ [ Gemini 3.1 API ]
                                            |
                                            |── 7. Receive JSON, format to .vital/.fxp
                                            |
                                            |── 8. Upload File ────────────→ [ Supabase Storage ]
                                            |
[ React/Vite Frontend ] ←── 9. Return Download URL ─────────────────────────|
```

---

## Token Gating Tiers

| Tier | Wallet Required | $AUDIO Balance | Daily Generations | Formats Available |
|------|:-:|---|:-:|---|
| **Unauthenticated** | No | — | 1 | Vital only |
| **Free** (Wallet Connected) | Yes | < 100 | 3 | Vital only |
| **Pro** | Yes | 100+ | 15 | Vital + Serum `.fxp` |
| **Studio** | Yes | 1,000+ | Unlimited | Vital + Serum `.fxp` |

**$AUDIO Token Mint Address (Solana):** `9LzCMqDgTKYz9Drzqnpgee3SGa89up3a247ypMj2xrqM`

---

## Hackathon Milestones

### Milestone 1 — Frontend Foundation (Days 1–2)
- [x] Scaffold React/Vite frontend
- [x] Monorepo structure with Turborepo
- [x] Component decomposition (Header, SearchDropdown, PromptInput, SuccessModal, etc.)
- [ ] Integrate Audius SDK — successfully search and play music in the browser
- [ ] Waveform player with timestamp selection

### Milestone 2 — Backend & AI Pipeline (Days 3–4)
- [x] Set up Rust backend (Axum) with route structure
- [ ] Set up Supabase project (database + storage)
- [ ] Successfully pass an audio file to Gemini 3.1 via API
- [ ] Receive a valid JSON response matching Vital schema

### Milestone 3 — Preset Generation (Days 5–6)
- [ ] Map Gemini JSON output to a downloadable `.vital` file (gzipped JSON)
- [ ] Test by dragging preset into Vital standalone app — verify sound accuracy
- [ ] Implement download flow end-to-end (generate → upload → download URL)

### Milestone 4 — Solana Integration (Days 7–8)
- [x] Scaffold Solana token gating logic (tiers, balance query)
- [ ] Integrate Solana Wallet Adapter (Phantom/Backpack) in frontend
- [ ] Wire up Rust backend to query `$AUDIO` balances via RPC
- [ ] Enforce Supabase daily limit tiers based on wallet balance

### Milestone 5 — Polish & Demo (Days 9–10)
- [ ] "Enhance Prompt" feature (lightweight LLM pre-processing)
- [ ] Quick-tag pills UI
- [ ] User dashboard — "My Presets" history
- [ ] UI polish, animations (motion library)
- [ ] Pitch deck / demo video recording
- [ ] Experimental Serum `.fxp` export (stretch goal)

---

## Out of Scope (Future Roadmap)

These features are intentionally excluded from the hackathon MVP to maintain focus on the core AI/Audio utility:

| Feature | Reason | Target Phase |
|---------|--------|:--:|
| **DAW VST/AU Plugin** | Requires C++/JUCE — entirely different codebase | Phase 3 |
| **NFT Preset Marketplace** | Adds smart contract complexity, distracts from core utility | Phase 2 |
| **100% Perfect Serum (.fxp)** | Proprietary binary format — experimental Python/Rust wrappers needed. Vital serves as primary demo. | Phase 2 |
| **On-Chain Generation Receipts** | Nice-to-have but not critical for MVP demo | Phase 2 |
| **$AUDIO Staking Vault** | Requires custom Solana program beyond stub | Phase 2 |
| **Mobile Responsive Design** | Desktop-first for producer workflow | Phase 2 |

---

*This is a living document. Check boxes are updated as features are implemented.*
