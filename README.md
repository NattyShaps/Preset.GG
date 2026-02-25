# 🎛️ Preset.GG

> **AI-powered Audio-to-Preset Generator — The "Shazam for Synths"**

Preset.gg is a decentralized sound design copilot that allows music producers to search for any track on Audius, highlight a specific sound, and instantly generate a playable, royalty-free synthesizer preset (Vital or Xfer Serum).

**Built for the Solana Graveyard Hackathon.**

---

## Architecture

```
[ React/Vite Frontend ]
       |
       |-- 1. Search/Stream ────────→ [ Audius API ]
       |
       |-- 2. Connect Wallet ───────→ [ Solana Wallet Adapter ]
       |
       |-- 3. Prompt + Audio URL ──→ [ Rust Backend (Axum) ]
                                          |
                                          |── 4. Verify $AUDIO ──→ [ Solana RPC ]
                                          |── 5. Check Limits ───→ [ Supabase DB ]
                                          |── 6. Audio + Prompt ─→ [ Gemini 3.1 ]
                                          |── 7. Format .vital/.fxp
                                          |── 8. Upload ─────────→ [ Supabase Storage ]
                                          |
[ Frontend ] ←── 9. Download URL ────────|
```

## Project Structure

```
Preset.GG/
├── apps/
│   ├── web/                 # React/Vite/Tailwind frontend
│   │   └── src/
│   │       ├── components/  # UI components (decomposed)
│   │       ├── hooks/       # Custom React hooks
│   │       ├── lib/         # API client, Audius SDK, Solana helpers
│   │       ├── stores/      # Zustand state management
│   │       ├── types/       # TypeScript type definitions
│   │       └── styles/      # Tailwind + global styles
│   │
│   └── api/                 # Rust backend (Axum)
│       └── src/
│           ├── routes/      # HTTP endpoints
│           ├── services/    # Gemini, Audius, Supabase integrations
│           ├── solana/      # Token gating, tier logic
│           ├── preset/      # .vital/.fxp file generation
│           ├── middleware/   # CORS, auth, rate limiting
│           ├── config/      # Environment configuration
│           └── errors/      # Error types
│
├── packages/
│   └── shared-types/        # Shared TypeScript types & constants
│
└── programs/
    └── preset-gate/         # Solana on-chain program (Anchor stub)
```

## Prerequisites

- **Node.js** >= 20
- **Rust** (latest stable)
- **Solana CLI** + **Anchor CLI** (for on-chain program)

## Getting Started

### 1. Clone & Install

```bash
git clone <repo-url>
cd Preset.GG
npm install
```

### 2. Environment Setup

```bash
# Copy env templates
cp .env.example .env
cp apps/web/.env.example apps/web/.env
cp apps/api/.env.example apps/api/.env

# Fill in your API keys
```

### 3. Run Frontend

```bash
cd apps/web
npm run dev
# → http://localhost:3000
```

### 4. Run Backend

```bash
cd apps/api
cargo run
# → http://localhost:3001
```

### 5. Run Both (Turborepo)

```bash
npm run dev
```

## Token Gating Tiers

| Tier | $AUDIO Required | Daily Generations | Formats |
|------|----------------|-------------------|---------|
| Free (No Wallet) | — | 1 | Vital |
| Basic (Wallet Connected) | < 100 | 3 | Vital |
| Pro | 100+ | 15 | Vital + Serum |
| Studio | 1,000+ | Unlimited | Vital + Serum |

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | React, Vite, TailwindCSS v4 |
| Backend | Rust, Axum |
| Database | Supabase (PostgreSQL) |
| Storage | Supabase Object Storage |
| AI Engine | Google Gemini 3.1 Pro |
| Audio Data | Audius JavaScript SDK |
| Web3 | Solana, @solana/wallet-adapter |
| On-Chain | Anchor Framework |

## License

MIT
