/**
 * Hook for searching tracks on Audius.
 * TODO: Integrate with Audius JavaScript SDK
 */
import { useState } from 'react';

export interface AudiusTrack {
  id: string;
  title: string;
  artist: string;
  artworkUrl?: string;
  duration: number;
  streamUrl?: string;
}

export function useAudiusSearch() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<AudiusTrack[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const search = async (searchQuery: string) => {
    setQuery(searchQuery);
    setIsLoading(true);
    setError(null);

    try {
      // TODO: Replace with actual Audius SDK call
      // const sdk = audiusSdk();
      // const { data } = await sdk.tracks.searchTracks({ query: searchQuery });
      // setResults(data.map(mapAudiusTrack));
      setResults([]);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Search failed');
    } finally {
      setIsLoading(false);
    }
  };

  const clear = () => {
    setQuery('');
    setResults([]);
    setError(null);
  };

  return { query, results, isLoading, error, search, clear };
}
