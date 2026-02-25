/**
 * Audius-related type definitions.
 */

export interface AudiusTrack {
  id: string;
  title: string;
  artist: string;
  artworkUrl?: string;
  duration: number;
  streamUrl?: string;
  genre?: string;
  mood?: string;
}

export interface AudiusArtist {
  id: string;
  name: string;
  handle: string;
  profilePictureUrl?: string;
}

export interface AudiusSearchResults {
  tracks: AudiusTrack[];
  artists: AudiusArtist[];
}
