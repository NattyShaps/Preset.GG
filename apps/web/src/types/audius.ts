/**
 * Audius-related type definitions.
 * Mapped from the raw Audius REST API response shape.
 */

export interface AudiusTrack {
  id: string;
  title: string;
  artist: string;           // from user.name
  artistHandle: string;     // from user.handle
  artworkUrl: string;       // from artwork.150x150 (thumbnail)
  artworkLarge: string;     // from artwork.480x480
  artworkMirrors: string[];  // from artwork.mirrors
  duration: number;          // in seconds
  genre: string;
  mood?: string;
  streamUrl: string;         // from stream.url (pre-signed direct URL)
  streamMirrors: string[];   // from stream.mirrors
  isStreamGated: boolean;
  playCount: number;
}
