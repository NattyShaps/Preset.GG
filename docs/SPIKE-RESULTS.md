# Spike Results — February 26, 2026

## Spike A — Audius SDK ✅ PASS

**What we tested:** Search Audius, get a playable audio stream URL.

**Results:**
- REST API at `https://api.audius.co/v1` works great with `x-api-key` header
- `GET /v1/tracks/search?query=...` returns full track objects with `id`, `title`, `user.name`, `duration`, `stream.url`
- `GET /v1/tracks/{trackId}/stream` returns a 302 redirect to a signed content node URL
- Following the redirect delivers real MP3 audio: 320kbps, 48kHz ✅
- Content nodes return `access-control-allow-origin: *` ✅

**Key findings:**
- Use the REST API directly (`fetch` with `x-api-key` header) — skip the SDK for now
- The `/v1/tracks/{id}/stream` redirect approach is reliable
- Track objects include a `stream.url` field but content node direct URLs can 506 — always use the `/stream` redirect endpoint instead

**Decision:** Use the REST API (not SDK) for search. Use `/v1/tracks/{id}/stream` for audio fetching (both frontend playback and backend Gemini pipeline).

---

## Spike B — Wavesurfer CORS ✅ PASS (by proxy)

**What we tested:** Can wavesurfer load audio directly from Audius?

**Results (inferred from Spike A):**
- Content nodes return `access-control-allow-origin: *` ✅
- The `/stream` endpoint follows redirects and lands on a content node
- Direct stream URL may 506 occasionally — the redirect endpoint is more stable

**Decision:** Wavesurfer should load audio via our Rust proxy endpoint (`GET /api/stream/{trackId}`) for reliability. The proxy follows the 302, fetches audio, and pipes it back with proper CORS headers. Frontend doesn't need to handle redirect chains.

---

## Spike C — Gemini Audio-to-JSON ✅ PASS

**What we tested:** Send real MP3 audio to Gemini, receive parseable Vital-style JSON.

**Results:**
- `gemini-2.5-flash` works (PRD mentions 3.1 Pro but 2.5 Flash is available and works)
- Accepts `audio/mp3` inline data (base64 encoded)
- Returns parseable JSON ✅
- `finishReason: STOP` confirms response is complete

**Critical technique discovered:**
- DO NOT ask Gemini to "output a JSON object with parameters" — it writes a verbose response and truncates
- DO provide a template JSON with placeholder values and ask Gemini to fill in the values
- Template approach = compact output, clean parse, no markdown fences, no truncation

**Working prompt pattern:**
```
'Output ONLY this exact JSON structure filled with your analysis values (no other text): 
{"osc_1_wave_frame":0,"osc_1_level":0,"filter_1_cutoff":0,...}'
```

**Sample output for Skrillex - Kliptown Empyrean (first ~30 seconds):**
```json
{
  "osc_1_wave_frame": 0.8,
  "osc_1_level": 0.9,
  "filter_1_cutoff": 0.85,
  "filter_1_resonance": 0.3,
  "env_1_attack": 0.05,
  "env_1_decay": 0.3,
  "env_1_sustain": 0.7,
  "env_1_release": 0.2,
  "lfo_1_frequency": 0.0,
  "lfo_1_amount": 0.0,
  "reverb_mix": 0.4,
  "distortion_amount": 0.2
}
```

Note: `filter_1_cutoff` came back as 0.85 (normalized 0-1) instead of Hz. Need to decide: normalize all params 0-1 for simplicity, or specify exact units and ranges in the prompt more explicitly.

**Decision:** Use normalized 0-1 values for all parameters in the Gemini prompt. Map to actual ranges in the Rust backend merge step. This makes the Gemini output more predictable and consistent.

**Model decision:** Use `gemini-2.5-flash` (not `gemini-3.1-pro-preview` — 3.1 Pro had issues returning coherent responses in testing).

---

## Adjusted Decisions for Milestone 1+

1. **Audius:** REST API only (no SDK). Use `/v1/tracks/{id}/stream` redirect for audio.
2. **Frontend audio:** Route through Rust proxy for reliability.
3. **Gemini:** `gemini-2.5-flash`. Template-based prompting. Normalized 0-1 params. Max 4096 tokens.
4. **Param ranges:** All params normalized 0-1 in Gemini layer; Rust maps to actual Vital ranges on merge.
