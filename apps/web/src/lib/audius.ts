/**
 * Audius REST API helpers.
 * Using raw fetch instead of @audius/sdk to avoid 1.8MB bundle overhead.
 */

import { API_URL, AUDIUS_API_BASE, AUDIUS_API_KEY, AUDIUS_SEARCH_LIMIT } from './constants';
import type { AudiusTrack } from '@/types/audius';

/**
 * Search Audius for tracks matching a query.
 */
export async function searchTracks(
  query: string,
  signal?: AbortSignal
): Promise<AudiusTrack[]> {
  const url = `${AUDIUS_API_BASE}/tracks/search?query=${encodeURIComponent(query)}&limit=${AUDIUS_SEARCH_LIMIT}`;

  const res = await fetch(url, {
    headers: AUDIUS_API_KEY ? { 'x-api-key': AUDIUS_API_KEY } : {},
    signal,
  });

  if (!res.ok) {
    throw new Error(`Audius search failed: ${res.status} ${res.statusText}`);
  }

  const json = await res.json();
  const rawTracks: unknown[] = json.data ?? [];

  return rawTracks.map(mapApiTrackToAudiusTrack);
}

/**
 * Get the redirect-based stream URL for a track.
 * The browser / wavesurfer will follow the 302 to the content node.
 *
 * This is the **preferred** playback URL — it always generates a fresh
 * signed redirect, whereas the pre-signed `stream.url` from search
 * results can expire if the user waits before pressing play.
 */
export function getStreamUrl(trackId: string): string {
  return `${API_URL}/api/stream/${trackId}`;
}

/**
 * Format seconds as mm:ss.
 */
export function formatDuration(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}

// ---------------------------------------------------------------------------
// Internal mapper — isolates us from raw API shape changes
// ---------------------------------------------------------------------------

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function mapApiTrackToAudiusTrack(raw: any): AudiusTrack {
  return {
    id: raw.id ?? '',
    title: raw.title ?? 'Untitled',
    artist: raw.user?.name ?? 'Unknown Artist',
    artistHandle: raw.user?.handle ?? '',
    artworkUrl: raw.artwork?.['150x150'] ?? raw.artwork?.['_150x150'] ?? '',
    artworkLarge: raw.artwork?.['480x480'] ?? raw.artwork?.['_480x480'] ?? '',
    artworkMirrors: raw.artwork?.mirrors ?? [],
    duration: raw.duration ?? 0,
    genre: raw.genre ?? '',
    mood: raw.mood ?? undefined,
    streamUrl: raw.stream?.url ?? '',
    streamMirrors: raw.stream?.mirrors ?? [],
    isStreamGated: raw.is_stream_gated ?? false,
    playCount: raw.play_count ?? 0,
  };
}
