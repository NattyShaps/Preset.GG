# 🗺️ Preset.GG — Final Execution Roadmap

> **From skeleton to working MVP**
>
> Incorporates feedback from all three agent reviews.
>
> Estimated total time: 10 days (hackathon timeline)
>
> Last Updated: February 25, 2026
>
> **Audius API context reviewed:** SDK v13.1.0, REST API at `api.audius.co/v1`, stream endpoints confirmed working with CORS-friendly content nodes.

---

## Table of Contents

- [Current State Assessment](#current-state-assessment)
- [Milestone 0.5 — Proof-of-Concept Spike](#milestone-05--proof-of-concept-spike-4-6-hours)
- [Milestone 1 — Audius Integration & Audio Player](#milestone-1--audius-integration--audio-player-days-1-2)
- [Milestone 2 — Backend Infrastructure & Gemini Pipeline](#milestone-2--backend-infrastructure--gemini-pipeline-days-3-4)
- [Milestone 3 — End-to-End Preset Download](#milestone-3--end-to-end-preset-download-days-5-6)
- [Milestone 4 — Solana Wallet & Token Gating](#milestone-4--solana-wallet--token-gating-days-7-8)
- [Milestone 5 — Polish, Features & Demo](#milestone-5--polish-features--demo-days-9-10)
- [Cross-Cutting Non-Functional Requirements](#cross-cutting-non-functional-requirements)
- [Risk Register & Contingency Plans](#risk-register--contingency-plans)
- [Critical Path](#critical-path-minimum-viable-demo)
- [Post-Hackathon Hardening](#post-hackathon-hardening-not-in-scope-documented-for-future)

---

## Current State Assessment

**What's done:**
- Turborepo monorepo (apps/web, apps/api, packages/shared-types, programs/preset-gate)
- Frontend: 9 components decomposed from App.tsx, 5 hooks, lib modules, type definitions
- Backend: Axum server scaffolded with routes, services, solana, preset modules (all stubs)
- Anchor program stub
- PRD documented
- Everything compiles and builds (TypeScript clean, Rust compiles, Vite builds)

**What's actually functional:**
- A pretty UI with mock data and zero real integrations
- A Rust server that compiles but every endpoint returns "not yet implemented"
- No connection to Audius, Gemini, Solana, or Supabase

---

## Milestone 0.5 — Proof-of-Concept Spike (4–6 hours)

**Goal: Prove the three critical unknowns before building anything real. Throwaway code, no architecture — just "does this work?"**

This is the most important milestone. If any of these three proofs fail, we pivot the approach before investing days.

### Spike A — Audius SDK (1–2 hours)

**What we're proving:** Can we search Audius and get a playable audio URL in the browser?

**Pre-step:** Register for an Audius API key at [api.audius.co/plans](https://api.audius.co/plans) or [audius.co/settings](https://audius.co/settings). Free tier gives 10 req/sec and 500K/month — plenty for our needs. We need an `apiKey` (read-only, safe for frontend) and optionally a `bearerToken` (for writes — we won't need this).

- Install `@audius/sdk` in `apps/web`
- Initialize SDK with `sdk({ apiKey: 'YOUR_KEY' })` (note: SDK uses `apiKey`, NOT `appName` — the scaffolded `lib/audius.ts` needs updating)
- Call `audiusSdk.tracks.searchTracks({ query: "Skrillex" })` (or equivalent)
- Log the results — do we get track objects with IDs, titles, artwork?
- **Key discovery from API testing:** Track objects already include a pre-signed `stream.url` field pointing directly to a content node, plus `stream.mirrors` for fallback. We may NOT need a separate "get stream URL" call.
- Try to play the `stream.url` from the track object in a plain HTML5 `<audio>` element
- Also test the REST endpoint directly: `GET https://api.audius.co/v1/tracks/{trackId}/stream` — this returns a 302 redirect to a signed content node URL
- **Test image loading:** Artwork comes with `mirrors` array. Try loading artwork — does it work on first try? Do we need mirror retry?
- Document findings: what works, what doesn't, what the actual API shape looks like

**What we already know from API testing:**
- `GET /v1/tracks/search?query=...` returns full track objects with `id`, `title`, `duration`, `genre`, `user.name`, `artwork` (with mirrors), `stream` (with `url` + `mirrors`), `access` info
- Audio is MP3 format, 320kbps, 48kHz (a 226-second Skrillex track was 8.7MB)
- Content nodes return `Access-Control-Allow-Origin: *` — CORS may not be as problematic as initially feared
- The API works without an `x-api-key` header, but we should register for one to avoid rate limits

**Success criteria:** We can search and get a real stream URL that plays audio.

**Failure contingency:** If the SDK is broken or deprecated, fall back to raw Audius REST API calls (`https://api.audius.co/v1/tracks/search?query=...` with `x-api-key` header).

### Spike B — Audio Proxy & Wavesurfer (1–2 hours)

**What we're proving:** Can wavesurfer render a waveform from Audius audio? Do we need a proxy, or can we use the pre-signed stream URL directly?

**Test 1 — Direct stream URL (try this first):**
- Track objects include a pre-signed `stream.url` pointing directly to a content node
- Content nodes return `Access-Control-Allow-Origin: *` headers
- Point wavesurfer.js directly at the `stream.url` from a track object
- Does the waveform render? Can we play it? Can we scrub?
- If this works, we can skip building a proxy route for frontend playback (still need backend audio fetch for Gemini pipeline)

**Test 2 — Proxy route (if direct fails):**
- Add a quick `GET /api/stream/{trackId}` route in the Rust backend
- Rust calls `GET https://api.audius.co/v1/tracks/{trackId}/stream`, follows the 302 redirect, fetches the audio bytes
- Pipe bytes back to frontend with `Content-Type: audio/mpeg` and `Access-Control-Allow-Origin: *`
- Point wavesurfer at `http://localhost:3001/api/stream/{trackId}`
- Does the waveform render? Can we play it? Can we scrub?

- Test with 2–3 different tracks to make sure it's not a fluke

**Success criteria:** Wavesurfer renders a waveform and plays audio (via either direct URL or proxy).

**Failure contingency:** If wavesurfer can't handle streamed audio from either approach, fall back to a simpler player (HTML5 `<audio>` element with a custom progress bar and manual timestamp text inputs instead of a visual waveform).

### Spike C — Gemini Audio-to-JSON (1–2 hours)

**What we're proving:** Can Gemini 3.1 accept an audio file and return parseable JSON that looks like Vital preset parameters?

- Use `curl` or a quick Rust script
- Download a short audio clip from Audius (or use any MP3)
- Base64 encode it
- Send to Gemini API with a basic prompt: "Analyze this audio and return a JSON object with synthesizer parameters like oscillator type, filter cutoff, envelope attack/decay/sustain/release, LFO rate"
- Does Gemini return JSON? Is it parseable? Are the values sensible?
- Try 3 different audio clips and prompts
- Test the timestamp approach: "Focus only on the segment from 0:30 to 0:45"
- Test the template approach: provide a partial Init.vital template and ask Gemini to modify specific parameters

**Success criteria:** Gemini returns parseable JSON with parameter values that are at least plausible for the given audio.

**Failure contingency:** If Gemini can't handle audio well, test with Gemini's latest model variant. If audio-to-preset fundamentally doesn't work, pivot to text-to-preset as a fallback (less impressive but still functional).

### Spike Wrap-Up

- Document all findings in a `docs/SPIKE-RESULTS.md` file
- Identify any API quirks, rate limits, or format requirements discovered
- Adjust the roadmap if any spike revealed unexpected constraints
- Delete throwaway code, carry forward only the learnings

---

## Milestone 1 — Audius Integration & Audio Player (Days 1–2)

**Goal: A user can search for real music on Audius and play it in the browser with a waveform and timestamp selection.**

### Day 1: Audius Search & Audio Proxy

**Task 1.1 — Install & Initialize Audius SDK**
- Install `@audius/sdk` in `apps/web`
- Create proper SDK instance in `lib/audius.ts` using `sdk({ apiKey: 'YOUR_KEY' })` (NOT `appName` — this was the old API)
- For read-only frontend use, only `apiKey` is needed (safe to expose in client code)
- `bearerToken` is only needed for write operations (uploads, favorites) — we don't need it
- Handle SDK initialization and export helper functions: `searchTracks()`, `getStreamUrl()`
- **Note:** Based on API testing, we may be able to use the REST API directly (`fetch` with `x-api-key` header) if the SDK proves problematic. The REST API at `https://api.audius.co/v1` is well-documented and straightforward.

**Task 1.2 — Wire Up Real Search**
- Update `hooks/useAudiusSearch.ts` to call the real Audius SDK
- Add debouncing (300ms) to avoid hammering the API on every keystroke
- Map Audius API response to our `AudiusTrack` type (id, title, artist, artwork, duration)
- Handle loading, empty results, and error states

**Task 1.3 — Update Search UI Components**
- Update `SearchDropdown.tsx`: replace mock data with real results
- Display track artwork thumbnails using an `AudiusImage` component (see Task 1.3b)
- Show track duration formatted as `mm:ss`
- Show artist name
- Handle "no results found" and "searching..." loading states

**Task 1.3b — Create `AudiusImage` Component (Mirror Retry)**
- Audius docs strongly recommend NEVER using raw `<img>` for Audius content
- Artwork URLs include a `mirrors` array of alternate content node hosts
- Create `components/ui/AudiusImage.tsx`:
  - Accepts `src`, `mirrors[]`, and standard img props
  - On load failure, swap the URL host with the next mirror and retry
  - Cycle through all mirrors before showing a fallback/placeholder
  - Pick the size variant (`150x150`, `480x480`, `1000x1000`) closest to the rendered size
- Use this component everywhere we display Audius artwork (search results, selected song badge, dashboard)

**Task 1.4 — Track Selection Flow**
- When user clicks a track, store the full track object (not just a string name)
- We need: `id`, `title`, `user.name`, `duration`, `artwork`, `stream.url`, `stream.mirrors`
- **Key insight:** The track object already includes a pre-signed `stream.url` — no separate API call needed to get a playable URL
- Update `SelectedSongBadge.tsx` to show artwork (via `AudiusImage`) + title + artist
- Consider introducing Zustand store at this point for cross-component state

**Task 1.5 — Build Audio Proxy Route (Rust Backend) — If Needed**
- **May be optional for frontend playback:** If Spike B proves that wavesurfer can load audio directly from Audius's pre-signed `stream.url` (content nodes do send `Access-Control-Allow-Origin: *`), this proxy is only needed for the backend Gemini pipeline (Milestone 2)
- **If needed for frontend playback, or for backend use regardless:**
- Add `GET /api/stream/{trackId}` to the Rust backend
- Rust calls `GET https://api.audius.co/v1/tracks/{trackId}/stream` (with `x-api-key` header)
- Follow the 302 redirect server-side, fetch audio bytes from the content node
- Pipe bytes back to client with proper headers:
  - `Content-Type: audio/mpeg`
  - `Access-Control-Allow-Origin: *`
  - `Accept-Ranges: bytes` (for seeking support)
- Add a **15-second timeout** on the upstream fetch
- Add a **max file size check** (reject if > 20MB to prevent abuse)
- **Note:** The stream endpoint returns signed URLs that handle auth automatically — no need to construct content node URLs manually

### Day 2: Waveform Player & Timestamp Selection

**Task 1.6 — Install & Set Up Wavesurfer**
- Install `wavesurfer.js` and `@wavesurfer/react` in `apps/web`
- Create `components/player/WaveformPlayer.tsx`
- Configure wavesurfer to load audio from:
  - **Option A (preferred):** Direct Audius `stream.url` from the track object (if Spike B confirms this works)
  - **Option B (fallback):** Our proxy endpoint `http://localhost:3001/api/stream/{trackId}`
- Style the waveform to match the purple/magenta gradient aesthetic

**Task 1.7 — Playback Controls**
- Play/pause button
- Current time / total duration display (`1:23 / 3:45`)
- Click-to-seek on the waveform
- Volume control (optional, low priority)

**Task 1.8 — Timestamp Region Selection**
- Install wavesurfer's `regions` plugin
- Allow user to click-drag on the waveform to highlight a time range
- Display selected range: `Selected: 0:45 — 1:00`
- Store `startTimestamp` and `endTimestamp` in state
- Default behavior: if no region selected, the system will use the full track (capped by audio budget in backend)
- Add a "clear selection" button to remove the region

**Task 1.9 — Update `useAudiusPlayer.ts` Hook**
- Wire up to real wavesurfer instance
- Expose: `play()`, `pause()`, `seek()`, `setRegion()`, `clearRegion()`
- Track: `isPlaying`, `currentTime`, `duration`, `selectedRegion { start, end }`
- Handle wavesurfer lifecycle (destroy on unmount, reinitialize on track change)

### Milestone 1 Exit Criteria

> User types "Skrillex" → sees real Audius results with artwork → clicks a track → waveform renders via our proxy → audio plays → user can select a 15-second region on the waveform. All real data, no mocks.

---

## Milestone 2 — Backend Infrastructure & Gemini Pipeline (Days 3–4)

**Goal: Send real audio + a prompt to Gemini 3.1 and receive a valid Vital preset JSON, with all supporting infrastructure in place.**

### Day 3: Supabase & Backend Infrastructure

**Task 2.1 — Set Up Supabase Project**
- Create a Supabase project (free tier)
- Create database tables:
  - `generations` table:
    - `id` (uuid, primary key)
    - `wallet_pubkey` (text, nullable — null for unauthenticated users)
    - `ip_address` (text — for unauthenticated rate limiting)
    - `prompt` (text)
    - `enhanced_prompt` (text, nullable — the post-enhancement version)
    - `track_id` (text, nullable)
    - `track_title` (text, nullable)
    - `start_timestamp` (float, nullable)
    - `end_timestamp` (float, nullable)
    - `preset_id` (text)
    - `format` (text — "vital" or "fxp")
    - `download_url` (text)
    - `created_at` (timestamptz, default now())
  - Index on `(wallet_pubkey, created_at)` for daily count queries
  - Index on `(ip_address, created_at)` for unauthenticated rate limiting
- Create Storage bucket: `presets` (public read access)
- Get project URL + service role key → add to `apps/api/.env`

**Task 2.2 — Implement Supabase Service (`services/supabase.rs`)**
- Use `reqwest` to call Supabase REST API (PostgREST)
- Implement `record_generation()` — insert row into `generations` table
- Implement `get_daily_generation_count_by_wallet()` — count today's rows for a wallet pubkey
- Implement `get_daily_generation_count_by_ip()` — count today's rows for an IP address
- Implement `upload_preset_file()` — upload bytes to storage bucket, return public URL
- Implement `get_user_presets()` — query generation history for a wallet
- Add error handling: map Supabase errors to our `AppError` types

**Task 2.3 — Define Audio Budget Policy**
- Add constants to `config/mod.rs`:
  - `MAX_AUDIO_DURATION_SECONDS: u64 = 120` (2 minutes)
  - `MAX_AUDIO_SIZE_BYTES: usize = 10_485_760` (10MB)
  - `GEMINI_REQUEST_TIMEOUT_SECONDS: u64 = 30`
  - `AUDIUS_FETCH_TIMEOUT_SECONDS: u64 = 15`
  - `GEMINI_MAX_RETRIES: u32 = 1`
- These are simple constants, not a middleware framework
- Enforce in the relevant service functions

**Task 2.4 — Implement Audius Audio Fetching (`services/audius.rs`)**
- Use `reqwest` to fetch audio from Audius for the Gemini pipeline
- **Recommended approach:** Call `GET https://api.audius.co/v1/tracks/{trackId}/stream` with `x-api-key` header — this returns a 302 redirect to a signed content node URL. Configure `reqwest` to follow redirects automatically.
- **Alternative:** If the frontend passes the pre-signed `stream.url` from the track object, the backend can fetch directly from that URL (skips the redirect hop)
- Audio format is MP3 (320kbps, 48kHz) — confirmed from API testing
- Enforce size limit: if response body exceeds `MAX_AUDIO_SIZE_BYTES` (10MB ≈ 3.5 minutes at 320kbps), abort and return error with message "This track is too long. Please select a shorter region."
- No ffmpeg, no clipping — we pass the full audio to Gemini
- If timestamps are provided, we include them in the Gemini prompt instead
- Return the audio as `Vec<u8>` (raw bytes)

### Day 4: Gemini Integration & Init.vital Template

**Task 2.5 — Create the Init.vital Template**
- Download and install Vital synth (free version)
- Open Vital → don't touch anything → File → Save Preset → save as `init_preset.vital`
- Unzip the file (it's gzipped JSON)
- Copy the raw JSON content
- Store it as a constant string in `preset/schema.rs`: `pub const INIT_VITAL_TEMPLATE: &str = r#"{ ... }"#;`
- Parse it on startup to verify it's valid JSON
- Update `VitalPreset` struct to match the actual schema we see in the file

**Task 2.6 — Define the Vital Parameter Allowlist**
- Study the Init.vital JSON — identify the key synthesis parameters:
  - Oscillator params: `osc_1_wave_frame`, `osc_1_level`, `osc_1_tune`, `osc_1_unison_voices`, `osc_1_unison_detune`, etc.
  - Filter params: `filter_1_cutoff`, `filter_1_resonance`, `filter_1_type`, `filter_1_drive`, etc.
  - Envelope params: `env_1_attack`, `env_1_decay`, `env_1_sustain`, `env_1_release`, etc.
  - LFO params: `lfo_1_frequency`, `lfo_1_shape`, `lfo_1_amount`, etc.
  - Effects: `reverb_mix`, `delay_mix`, `distortion_amount`, etc.
- Create a `HashMap` or `HashSet` of allowed keys with their valid ranges:
  - `"filter_1_cutoff" => (20.0, 20000.0)`
  - `"env_1_attack" => (0.0, 4.0)`
  - etc.
- Store in `preset/schema.rs` as `ALLOWED_PARAMS` or similar
- This is our merge validator — any key not in this list gets dropped, any value outside the range gets clamped

**Task 2.7 — Craft the Gemini System Prompt**
- This is the most important piece of text in the entire project
- The system prompt must:
  1. Establish role: "You are an expert synthesizer sound designer"
  2. Explain the task: "Analyze the provided audio and determine what synthesizer parameters would recreate this sound"
  3. Handle timestamps: "If a time range is specified, focus ONLY on the sound heard during that window"
  4. Specify output format: "Return ONLY a JSON object containing parameter modifications"
  5. Provide the allowlist: "You may ONLY use the following parameter keys: [list]"
  6. Provide value ranges: "Each parameter must be within these ranges: [list]"
  7. Provide 2–3 examples of good output for different sound types
  8. Explicitly forbid: "Do not include markdown, explanations, or any text outside the JSON object"
- Store as a constant in `services/gemini.rs`

**Task 2.8 — Implement Gemini Service (`services/gemini.rs`)**
- Use `reqwest` to call Gemini API
- Construct the multimodal request:
  - System instruction: our crafted system prompt
  - User content part 1: inline audio data (base64-encoded bytes)
  - User content part 2: user's text prompt + timestamp instructions (if any)
- Set request timeout to `GEMINI_REQUEST_TIMEOUT_SECONDS`
- Parse the response:
  - Extract the text content from Gemini's response
  - Strip any markdown code fences (`` ```json ... ``` ``) if Gemini wraps the output
  - Parse as JSON
  - If parsing fails and retries remain: retry once with an additional prompt nudge ("respond with ONLY valid JSON, no other text")
  - If parsing still fails: return error
- Return the parsed `serde_json::Value`

**Task 2.9 — Implement Merge & Validate Logic (`preset/vital.rs`)**
- Load the `INIT_VITAL_TEMPLATE` as a base `serde_json::Value`
- Take Gemini's response (a partial JSON of parameter deltas)
- For each key-value pair in Gemini's output:
  - Check if the key is in `ALLOWED_PARAMS` — if not, drop it silently
  - Check if the value is within the valid range — if not, clamp it
  - Merge it into the template (overwrite the default value)
- Serialize the merged JSON
- Gzip compress it
- Return the bytes (this is a valid `.vital` file)

**Task 2.10 — Wire Up the Generate Endpoint (First Pass)**
- Update `routes/generate.rs` with the actual pipeline:
  1. Parse and validate request
  2. Fetch audio from Audius (`services/audius`)
  3. Enforce audio size budget
  4. Send audio + prompt to Gemini (`services/gemini`)
  5. Parse and validate Gemini response
  6. Merge into Init.vital template (`preset/vital`)
  7. Return the raw preset JSON (file upload comes in M3)
- Test with `curl`:
  ```
  curl -X POST http://localhost:3001/api/generate \
    -H "Content-Type: application/json" \
    -d '{"prompt": "heavy dubstep bass", "track_id": "some-real-audius-id"}'
  ```

### Milestone 2 Exit Criteria

> `POST /api/generate` with a real Audius track ID and prompt → backend fetches audio → sends to Gemini with our crafted system prompt → receives JSON parameter deltas → merges into Init.vital template → returns valid preset JSON. Supabase project exists with tables and storage bucket ready.

---

## Milestone 3 — End-to-End Preset Download (Days 5–6)

**Goal: User generates a preset from the UI, downloads a `.vital` file, drags it into Vital, and it makes a sound that resembles what they asked for.**

### Day 5: Complete Backend Pipeline & Storage

**Task 3.1 — Complete File Upload Flow**
- After merge/validate produces the `.vital` bytes:
  - Generate a unique filename: `preset_{uuid}.vital`
  - Upload to Supabase Storage via `services/supabase::upload_preset_file()`
  - Record the generation in the `generations` table
  - Return response: `{ preset_id, download_url, file_name, format }`

**Task 3.2 — Implement Download Route**
- Update `routes/presets.rs`:
  - `GET /api/presets/{id}/download` → look up the generation record in Supabase → return the Supabase Storage public URL as a redirect
  - Or: return the file directly with proper download headers:
    - `Content-Type: application/octet-stream`
    - `Content-Disposition: attachment; filename="preset_generated.vital"`

**Task 3.3 — Implement Unauthenticated Rate Limiting**
- In the generate endpoint, before processing:
  - If no `wallet_pubkey` provided: extract client IP from request headers
  - Query `get_daily_generation_count_by_ip()` from Supabase
  - If count >= 1: return `429 Too Many Requests` with message "Connect your Solana wallet to unlock more generations"
  - If count < 1: proceed with generation

**Task 3.4 — Test with Vital Standalone**
- Generate a preset via the API
- Download the `.vital` file
- Open Vital synth → drag the file into the preset browser
- **Does it load?** If not: debug the JSON structure (compare against Init.vital template)
- **Does it make sound?** If not: check that oscillator levels aren't zero'd out
- **Does it sound anything like the prompt?** If not: iterate on the Gemini system prompt
- This is an iteration loop — expect to spend 2–4 hours here going back and forth between system prompt tweaks and testing in Vital
- Try 3–5 different sound types: bass, pad, lead, pluck, FX

### Day 6: Frontend Integration

**Task 3.5 — Connect Frontend to Backend API**
- Update `lib/api-client.ts`: add `generatePreset()` function
- Update `hooks/usePresetGeneration.ts`:
  - Send: `{ prompt, trackId, startTimestamp, endTimestamp, format: "vital" }`
  - Receive: `{ presetId, downloadUrl, fileName }`
  - Handle loading, success, and error states properly

**Task 3.6 — Update Generation Flow UI**
- Wire up the "Generate Preset" button (ArrowRight in PromptInput) to call the real API
- Loading state: spinning Audius logo (already implemented in SpinnerLogo)
- Success: show `SuccessModal` with the real download URL
- Update `SuccessModal`:
  - "Save" button triggers actual file download (`window.open(downloadUrl)` or `<a download>`)
  - Show the generated preset name and format
  - "Open" button is cosmetic for now (would launch Vital if we had a protocol handler)
- Error: show an error message (inline or toast)
  - "Audio too large" → "Try selecting a shorter region"
  - "AI generation failed" → "Try a different prompt or track"
  - "Rate limited" → trigger the tier gate modal

**Task 3.7 — Build the Tier Gate Modal**
- Create `components/wallet/TierModal.tsx`
- Triggered when:
  - Unauthenticated user has used their 1 free generation
  - Any rate limit error from the backend
- Content:
  - "Want more generations?"
  - Explain the $AUDIO token gating model
  - Show the tier table (Free / Pro / Studio with limits)
  - CTA: "Connect Wallet" button (wired up in Milestone 4)
  - Secondary: link to buy $AUDIO on a DEX

**Task 3.8 — Frontend Rate Limit Tracking**
- Track generation count in `localStorage` for unauthenticated users
- On app load: check `localStorage` for today's count
- Before calling API: if count >= 1 and no wallet connected, show TierModal immediately (don't waste an API call)
- On successful generation: increment the localStorage counter
- Reset counter daily (store the date alongside the count)
- This is a client-side convenience check — the backend enforces the real limit via Supabase

**Task 3.9 — Fake Serum Export Button**
- Add a "Download .fxp" button to the SuccessModal
- Show a lock icon on it
- If user is not Pro tier: button is disabled, tooltip says "Hold 100+ $AUDIO to unlock Serum exports"
- If user IS Pro tier: clicking shows a toast: "Serum .fxp export is in Beta. Downloading Vital format instead."
- Generates the Vital file either way
- This gives judges the token-gated UI without us needing to reverse-engineer binary formats

### Milestone 3 Exit Criteria

> Full end-to-end flow works: User searches a real track → selects a waveform region → types a prompt → clicks Generate → loading spinner → `.vital` file downloads → user drags into Vital → **it makes a sound.** Rate limiting works for unauthenticated users. Tier gate modal appears when limit is hit.

---

## Milestone 4 — Solana Wallet & Token Gating (Days 7–8)

**Goal: Wallet connection works. $AUDIO balance determines tier. Generation limits are enforced based on tier.**

### Security & Trust Model (Hackathon MVP)

> **Explicit acknowledgment:** For the hackathon MVP, we operate under a trust model. The frontend-provided `publicKey` is accepted without cryptographic signature verification (SIWS). This means tier spoofing is theoretically possible — a user could submit another wallet's publicKey to inherit their tier. This is acceptable for a demo environment. SIWS is planned as the first post-hackathon security hardening task. This is documented in the codebase via comments in `routes/auth.rs`.

### Day 7: Frontend Wallet Integration

**Task 4.1 — Install Solana Wallet Dependencies**
- Install in `apps/web`:
  - `@solana/wallet-adapter-base`
  - `@solana/wallet-adapter-react`
  - `@solana/wallet-adapter-react-ui`
  - `@solana/wallet-adapter-wallets`
  - `@solana/web3.js`
- Note: these packages may have peer dependency issues with React 19 — test and resolve

**Task 4.2 — Set Up Wallet Providers**
- Update `main.tsx` to wrap `<App>` with:
  - `<ConnectionProvider endpoint={SOLANA_RPC_URL}>`
  - `<WalletProvider wallets={[PhantomWalletAdapter, BackpackWalletAdapter, SolflareWalletAdapter]}>`
  - `<WalletModalProvider>`
- The wallet adapter provides its own connect modal UI

**Task 4.3 — Update Header Wallet Button**
- Replace the static `[connect wallet]` button in `Header.tsx`
- Use the `<WalletMultiButton>` component from wallet adapter UI
- Style to match the XP aesthetic (custom CSS override or wrapper component)
- When connected, show truncated public key: `7xK3...9fPm`
- Dropdown with disconnect option

**Task 4.4 — Update `useWalletAuth.ts` Hook**
- Replace mock implementation with actual `useWallet()` from `@solana/wallet-adapter-react`
- Expose: `publicKey`, `isConnected`, `isConnecting`, `connect()`, `disconnect()`
- On wallet connect: automatically call backend `/api/auth/verify` to get tier info
- On wallet disconnect: reset tier to unauthenticated

**Task 4.5 — Update `useUserTier.ts` Hook**
- On wallet connect: call `POST /api/auth/verify` with `{ wallet_pubkey: publicKey }`
- Store returned tier info: `{ tier, audioBalance, dailyGenerationsUsed, dailyGenerationsLimit }`
- Expose these values for UI consumption
- Re-fetch tier info after each successful generation (to update the counter)

### Day 8: Backend Token Gating

**Task 4.6 — Test Token Gate Function**
- The `solana/token_gate.rs` code is already scaffolded
- Test against a known wallet that holds $AUDIO → verify correct balance returned
- Test against a wallet with zero $AUDIO → verify returns 0.0
- Test against an invalid pubkey → verify graceful error handling
- Test RPC timeout handling (what if Solana RPC is slow?)
- Add a cache consideration: balance queries don't need to be real-time. Cache for 60 seconds to reduce RPC calls.

**Task 4.7 — Implement Auth Verify Endpoint**
- Update `routes/auth.rs`:
  1. Receive `{ wallet_pubkey }` (no signature verification per trust model)
  2. Query $AUDIO balance via `solana/token_gate::get_audio_balance()`
  3. Determine tier via `solana/tiers::tier_from_balance()`
  4. Query Supabase for today's generation count for this wallet
  5. Get the daily limit for the tier
  6. Return: `{ tier, audio_balance, daily_generations_used, daily_generations_limit }`
- Add comment: `// HACKATHON MVP: No signature verification. See docs/ROADMAP.md security section.`

**Task 4.8 — Enforce Tiers in Generate Endpoint**
- Update `routes/generate.rs`:
  - If `wallet_pubkey` is provided:
    - Query $AUDIO balance → determine tier
    - Query daily generation count from Supabase
    - If count >= tier limit: return `429` with message and tier info
    - If Serum format requested and tier < Pro: return `403` with upgrade message
  - If no `wallet_pubkey`:
    - Fall back to IP-based limiting (already implemented in M3)
  - On successful generation: record with wallet_pubkey in Supabase

**Task 4.9 — Frontend Tier-Aware UI Updates**
- Update the generation counter display in `PromptInput.tsx`:
  - Currently hardcoded `Gen: 1/1`
  - Replace with dynamic: `Gen: {used}/{limit}` from `useUserTier` hook
  - Color coding: green when plenty left, yellow when low, red when at limit
- Update `TierModal.tsx`:
  - Show user's current tier and $AUDIO balance
  - Show what they need to reach the next tier
  - "You hold 45 $AUDIO (Basic tier). Hold 100+ $AUDIO to unlock Pro."
- Update Serum button visibility:
  - Pro/Studio users: button is enabled
  - Free/Unauthenticated: button shows lock icon with tooltip
- Show tier badge near wallet button: `[PRO]` or `[STUDIO]`

### Milestone 4 Exit Criteria

> User connects Phantom wallet → app queries $AUDIO balance → displays correct tier badge → generation counter shows real limits (`Gen: 2/15`) → exceeding limit shows upgrade modal → user with 100+ $AUDIO sees unlocked Serum button → all limits enforced on the backend.

---

## Milestone 5 — Polish, Features & Demo (Days 9–10)

**Goal: Everything is polished, the enhance prompt feature works, and we have a compelling demo video and pitch deck.**

### Day 9: Feature Completion & Polish

**Task 5.1 — Quick-Tag Pills**
- Create `components/prompt/QuickTags.tsx`
- Tags: `[Bass]`, `[Lead]`, `[Pad]`, `[Pluck]`, `[FX]`, `[Arp]`
- Clicking a tag appends the term to the current prompt text
- Tags are toggleable — clicking again removes the term
- Multiple tags can be active
- Styled as small pill buttons rendered below the main prompt input
- Wire into `App.tsx` or prompt section

**Task 5.2 — "Enhance Prompt" Feature**
- Create `components/prompt/EnhanceToggle.tsx`
- A toggle switch rendered near the prompt input
- When enabled, before sending to the main Gemini audio call:
  1. Send the user's raw prompt to a lightweight Gemini text-only call
  2. System prompt: "You are a synthesizer expert. Translate this casual description into precise technical synthesis terminology. Include specific parameter suggestions like oscillator type, filter type, envelope shape, modulation routing. Return only the enhanced text, no explanation."
  3. Example: Input "heavy wubby bass" → Output "FM synthesis bass patch with 2 detuned oscillators, low-pass filter at 200Hz with high resonance and LFO modulation on cutoff at 1/4 note rate, short attack, medium decay, low sustain, medium release"
- Show the enhanced prompt to the user in a collapsible "see enhanced prompt" section
- The enhanced prompt is what gets sent to the main audio analysis call
- User can edit the enhanced prompt before generating

**Task 5.3 — User Dashboard ("My Presets")**
- Create `pages/Dashboard.tsx`
- Add simple tab-based navigation: "Generate" tab (main view) and "My Presets" tab
- No need for react-router — just conditional rendering based on active tab
- Dashboard queries `GET /api/presets` with wallet pubkey header
- Display list of previously generated presets:
  - Preset name (derived from prompt)
  - Format badge (Vital / Serum)
  - Date generated
  - Original prompt text
  - Re-download button
- Empty state: "No presets yet. Generate your first one!" with CTA button
- Only visible when wallet is connected

**Task 5.4 — Implement List Presets Endpoint**
- Update `routes/presets.rs`:
  - `GET /api/presets` → extract wallet pubkey from `X-Wallet-Pubkey` header
  - Query Supabase `generations` table for this wallet, ordered by `created_at DESC`
  - Return array of preset metadata

**Task 5.5 — UI Polish & Animations**
- Add `motion` (framer-motion) animations:
  - `AnimatePresence` for modal enter/exit (fade + scale)
  - Slide-up animation for search dropdown appearing
  - Subtle pulse on the Generate button when both prompt and track are selected (ready state)
  - Smooth opacity transition between app states
- Loading state improvements:
  - The Audius logo spin is good; add a subtle glow pulse effect
  - Show elapsed time: "Generating... 3s"
  - Show a progress message sequence: "Fetching audio..." → "Analyzing sound..." → "Building preset..."
- Success state: add a subtle confetti or sparkle effect (keep it tasteful)
- Error states: add toast notification component for non-blocking errors

**Task 5.6 — Edge Cases & Error Handling**
- Handle each failure gracefully with user-friendly messages:
  - Audius search returns no results → "No tracks found. Try a different search."
  - Audio stream fails → "Couldn't load this track. Try another one."
  - Audio too large → "This track is too long. Select a shorter region."
  - Gemini returns garbage → "AI couldn't analyze this sound. Try a different prompt or track." (with retry button)
  - Gemini rate limited → "AI service is busy. Please try again in a moment."
  - Supabase upload fails → "Couldn't save your preset. Please try again."
  - Solana RPC timeout → "Couldn't verify wallet balance. Please try again."
  - Wallet connection fails → "Wallet connection failed. Make sure Phantom is installed."
- Add retry logic: one automatic retry on Gemini JSON parse failure (already in pipeline)
- Add a manual "Try Again" button on error states

### Day 10: Demo & Submission

**Task 5.7 — End-to-End Testing Matrix**

Test each combination:

| Scenario | Expected Result |
|---|---|
| Search "Skrillex", select track, prompt "heavy bass", generate | .vital downloads, loads in Vital, sounds bass-like |
| Search "ambient", select track, prompt "ethereal pad", generate | .vital downloads, loads in Vital, sounds pad-like |
| Generate with no track selected, text-only prompt | Should still work (text-to-preset fallback) |
| Generate without wallet (first time) | Works, downloads preset |
| Generate without wallet (second time) | Blocked, shows TierModal |
| Connect wallet with 0 $AUDIO | Shows "Basic" tier, 3 generations/day |
| Connect wallet with 150 $AUDIO | Shows "Pro" tier, 15 generations/day, Serum button unlocked |
| Click Serum button as Pro user | Shows "Beta" toast, downloads Vital |
| Disconnect wallet | Resets to unauthenticated state |
| Very long track (>5 min) with no region selected | Backend handles gracefully (audio budget caps it) |
| Empty prompt with track selected | Should still work (Gemini analyzes audio without specific direction) |

**Task 5.8 — Demo Video Script & Recording**
- Tool: OBS Studio or similar screen recorder
- Duration: 2–3 minutes max
- Script:
  1. **Hook (10 sec):** "What if you could hear any sound and instantly recreate it as a playable synth preset?"
  2. **Show the UI (15 sec):** Open preset.gg, show the clean interface, explain the concept
  3. **Search (15 sec):** Type a search, show real Audius results populating
  4. **Select & Region (15 sec):** Click a track, waveform appears, select a region on the drop
  5. **Prompt & Generate (20 sec):** Type "Main FM bass", click Generate, show the loading animation
  6. **The Magic (20 sec):** Preset downloads. Switch to DAW. Drag the .vital file into Vital. Play a MIDI note. React to the sound.
  7. **Token Gating (20 sec):** Show wallet connect. Show tier badge. Show the generation counter. Show what happens when you hit the limit.
  8. **Tech (15 sec):** Quick overlay showing the architecture: "Audius → Gemini 3.1 → Vital, all gated by $AUDIO on Solana"
  9. **Close (10 sec):** "Preset.gg — the Shazam for Synths. Built on Solana."

**Task 5.9 — Pitch Deck**
- 5–7 slides max (Google Slides or similar):
  1. **Title:** Preset.gg — AI-powered Shazam for Synths
  2. **Problem:** Sound design bottleneck. 3 bullet points (time, static audio, hallucination gap)
  3. **Solution:** Audio-to-Preset, not Text-to-Preset. Show the before/after
  4. **Demo:** Embedded video or screenshot sequence
  5. **Why Solana + Audius:** Permissionless catalog + $AUDIO token utility. Show the tier table.
  6. **Architecture:** The data flow diagram from the PRD
  7. **Roadmap:** DAW plugin → NFT marketplace → staking. Show this isn't a throwaway.

**Task 5.10 — Submission Prep**
- Clean up codebase:
  - Remove any `console.log` / debug statements
  - Remove any hardcoded test data
  - Verify all `.env.example` files are accurate and complete
  - Run `cargo clippy` on Rust code, fix any warnings
  - Run `tsc --noEmit` on frontend, verify clean
- Update `README.md`:
  - Final setup instructions
  - Link to live demo (if deployed) or demo video
  - Screenshots
- Final git commit: `feat: hackathon MVP complete`
- Push to GitHub
- Write hackathon submission description (2–3 paragraphs):
  - What it does
  - How it uses Solana/Audius
  - What makes it technically unique (Audio-to-Preset vs Text-to-Preset)
- Submit before deadline

### Milestone 5 Exit Criteria

> A polished, demo-ready dApp. Quick-tag pills and enhance prompt work. My Presets dashboard shows history. Animations are smooth. Error handling is graceful. Demo video is recorded. Pitch deck is ready. Submission is prepared.

---

## Cross-Cutting Non-Functional Requirements

These apply across all milestones and should be implemented as we go, not as a separate phase:

| Requirement | Implementation | Where |
|---|---|---|
| **Request timeouts** | 30s on Gemini, 15s on Audius fetch, 10s on Supabase calls | `reqwest` client config in each service |
| **Retry policy** | 1 retry on Gemini JSON parse failure only. No retry on other failures. | `services/gemini.rs` |
| **Max audio size** | 10MB hard cap. Reject with 413 error. | `services/audius.rs` + `routes/generate.rs` |
| **Request logging** | Log every generate request: audio size, prompt length, response time, success/failure | `tracing::info!()` in `routes/generate.rs` |
| **CORS** | Allow frontend origin (`localhost:3000` in dev) | Already configured in `middleware/cors.rs` |
| **Error responses** | All errors return consistent JSON: `{ "error": "message", "statusCode": 400 }` | Already defined in `errors/mod.rs` |

---

## Risk Register & Contingency Plans

| Risk | Likelihood | Impact | Mitigation |
|---|:-:|:-:|---|
| **Gemini returns unparseable JSON** | High | High | Strip markdown fences, 1 retry with nudge prompt, template approach guarantees structure |
| **Generated presets sound bad** | Medium | High | Iterate on system prompt in M2/M3. Template approach + parameter clamping ensures playable sound. Accept "close enough" for hackathon. |
| **Audius SDK has issues** | Medium | Medium | Fallback to raw REST API (`api.audius.co/v1`). API confirmed working via direct testing. Spike (M0.5) catches SDK-specific issues early. |
| **Wavesurfer CORS problems** | Low (mitigated) | Medium | Content nodes send `Access-Control-Allow-Origin: *`. Pre-signed `stream.url` may work directly. Audio proxy through Rust backend as fallback. Spike B tests both approaches. |
| **Solana RPC rate limits** | Low | Low | Use Helius/Triton free tier for reliable RPC. Cache balance for 60 seconds. |
| **Serum .fxp doesn't work** | High | Low | Already de-scoped. Button shows "Beta" toast. |
| **Wallet adapter + React 19 conflicts** | Medium | Medium | Check peer dependencies early in M4. May need adapter version pinning. |
| **Run out of time** | Medium | High | Priority order: M0.5 → M1 → M2 → M3 → M4 → M5. M1–M4 = core demo. M5 = polish. Skip dashboard, skip enhance prompt if needed. Quick-tags are 30 minutes, do those regardless. |

---

## Critical Path (Minimum Viable Demo)

If time runs short, this is the absolute minimum for a compelling hackathon demo:

```
Spike (M0.5) → Search real music (M1) → Send to Gemini (M2) → Download .vital that works in Vital (M3) → Wallet + $AUDIO gate (M4)
```

Everything in Milestone 5 is polish. If we nail M0.5 through M4, we have a winning demo.

---

## Post-Hackathon Hardening (Not in scope, documented for future)

| Task | Priority |
|---|---|
| Implement SIWS (Sign-In With Solana) signature verification | P0 |
| Deploy to production (Railway/Render for Rust, Vercel for frontend) | P0 |
| Add proper authentication flow (JWT tokens from verified wallet signatures) | P1 |
| Implement real Serum `.fxp` binary export | P1 |
| Add audio caching layer (don't re-fetch same track from Audius) | P2 |
| Gemini response caching (same track + similar prompt = cached result) | P2 |
| Cost monitoring and billing for Gemini API usage | P1 |
| Rate limiting middleware (proper per-IP with sliding window, not just Supabase count) | P2 |
| Mobile responsive design | P2 |
| DAW VST/AU plugin (C++/JUCE) | P3 |
| NFT preset marketplace | P3 |

---

## Audius API Quick Reference

Key findings from documentation and live API testing (February 25, 2026):

| Resource | URL |
|---|---|
| REST API Base | `https://api.audius.co/v1` |
| API Plans (get keys) | `https://api.audius.co/plans` |
| SDK (npm) | `@audius/sdk` v13.1.0 |
| Swagger/OpenAPI Spec | `https://api.audius.co/v1/swagger.yaml` |
| Agent Context | `https://audius.co/agents.md` |
| SDK Code Guide | `https://audius.co/skill.md` |

**Key Endpoints:**
- `GET /v1/tracks/search?query=...&limit=...` — Search tracks (returns full track objects with artwork, stream URLs)
- `GET /v1/tracks/{trackId}` — Get single track details
- `GET /v1/tracks/{trackId}/stream` — Stream audio (302 redirect to signed content node URL)
- `GET /v1/tracks/trending?limit=...` — Trending tracks

**SDK Initialization:**
```js
import { sdk } from '@audius/sdk'
const audiusSdk = sdk({ apiKey: 'YOUR_API_KEY' }) // bearerToken only needed for writes
```

**Track Object Shape (key fields):**
```
id, title, duration (seconds), genre, mood, user.name, user.handle,
artwork.{150x150, 480x480, 1000x1000, mirrors[]},
stream.{url, mirrors[]},
access.{stream, download},
is_stream_gated, is_downloadable, play_count
```

**Audio Format:** MP3, 320kbps, 48kHz. ~2.4MB per minute.

**Auth:** `x-api-key` header. Free tier: 10 req/sec, 500K/month. API works without key but may be rate-limited.

**CORS:** Content nodes return `Access-Control-Allow-Origin: *`. The 302 redirect from `api.audius.co` may still require a proxy for Web Audio API contexts.

**Images:** Always use mirror retry. Never raw `<img>`. Artwork includes `mirrors[]` for fallback hosts.

---

*This is a living document. Updated as milestones are completed.*
