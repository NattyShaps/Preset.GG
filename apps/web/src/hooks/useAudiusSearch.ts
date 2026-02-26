/**
 * Hook for searching tracks on Audius.
 * Uses raw REST API with debouncing and request cancellation.
 */
import { useState, useRef, useCallback } from 'react';
import { searchTracks } from '@/lib/audius';
import { AUDIUS_DEBOUNCE_MS } from '@/lib/constants';
import type { AudiusTrack } from '@/types/audius';

export function useAudiusSearch() {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<AudiusTrack[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const abortRef = useRef<AbortController | null>(null);

  const search = useCallback((searchQuery: string) => {
    setQuery(searchQuery);

    // Clear previous debounce
    if (debounceRef.current) {
      clearTimeout(debounceRef.current);
    }

    // Cancel in-flight request
    if (abortRef.current) {
      abortRef.current.abort();
      abortRef.current = null;
    }

    // Don't search for very short queries
    if (searchQuery.trim().length < 2) {
      setResults([]);
      setIsLoading(false);
      setError(null);
      return;
    }

    setIsLoading(true);
    setError(null);

    debounceRef.current = setTimeout(async () => {
      const controller = new AbortController();
      abortRef.current = controller;

      try {
        const tracks = await searchTracks(searchQuery.trim(), controller.signal);
        // Only update if this request wasn't aborted
        if (!controller.signal.aborted) {
          setResults(tracks);
          setIsLoading(false);
        }
      } catch (err) {
        if (err instanceof DOMException && err.name === 'AbortError') {
          // Request was cancelled — ignore
          return;
        }
        if (!controller.signal.aborted) {
          setError(err instanceof Error ? err.message : 'Search failed');
          setResults([]);
          setIsLoading(false);
        }
      }
    }, AUDIUS_DEBOUNCE_MS);
  }, []);

  const clear = useCallback(() => {
    if (debounceRef.current) clearTimeout(debounceRef.current);
    if (abortRef.current) abortRef.current.abort();
    setQuery('');
    setResults([]);
    setIsLoading(false);
    setError(null);
  }, []);

  return { query, results, isLoading, error, search, clear };
}
