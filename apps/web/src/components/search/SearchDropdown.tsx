import { Search, Play, Loader2 } from 'lucide-react';
import { formatDuration } from '@/lib/audius';
import type { AudiusTrack } from '@/types/audius';

interface SearchDropdownProps {
  searchQuery: string;
  onSearchQueryChange: (value: string) => void;
  results: AudiusTrack[];
  isLoading: boolean;
  onSelectTrack: (track: AudiusTrack) => void;
}

export default function SearchDropdown({
  searchQuery,
  onSearchQueryChange,
  results,
  isLoading,
  onSelectTrack,
}: SearchDropdownProps) {
  const showEmpty = !isLoading && results.length === 0 && searchQuery.trim().length >= 2;
  const showHint = searchQuery.trim().length < 2;

  return (
    <div className="absolute top-full left-0 right-10 mt-3 bg-white/10 backdrop-blur-xl border border-white/20 rounded-2xl p-2 z-20 shadow-[0_8px_32px_rgba(0,0,0,0.2)]">
      <div className="bg-white/10 border border-white/20 rounded-xl flex items-center p-2 mb-2">
        <Search className="w-4 h-4 text-white/60 mr-2 ml-1" />
        <input
          type="text"
          value={searchQuery}
          onChange={(e) => onSearchQueryChange(e.target.value)}
          placeholder="Search Audius for a specific track..."
          className="w-full outline-none text-sm p-1 bg-transparent text-white placeholder-white/50"
          autoFocus
        />
        {isLoading && (
          <Loader2 className="w-4 h-4 text-white/60 animate-spin ml-2 shrink-0" />
        )}
      </div>

      <div className="max-h-64 overflow-y-auto rounded-lg">
        {showHint && (
          <div className="p-4 text-center text-white/50 text-sm">
            Type to search Audius...
          </div>
        )}

        {showEmpty && (
          <div className="p-4 text-center text-white/60 text-sm">
            No tracks found.
          </div>
        )}

        {results.map((track) => (
          <div
            key={track.id}
            onClick={() => onSelectTrack(track)}
            className="p-2 hover:bg-white/20 cursor-pointer text-sm flex items-center border-b border-white/10 last:border-0 text-white transition-colors rounded-lg mb-0.5"
          >
            {/* Artwork thumbnail */}
            {track.artworkUrl ? (
              <img
                src={track.artworkUrl}
                alt=""
                className="w-10 h-10 rounded-md mr-3 object-cover shrink-0 bg-white/10"
                loading="lazy"
              />
            ) : (
              <div className="w-10 h-10 rounded-md mr-3 bg-white/10 shrink-0 flex items-center justify-center">
                <Play className="w-4 h-4 opacity-40" />
              </div>
            )}

            {/* Track info */}
            <div className="flex-1 min-w-0">
              <div className="font-bold truncate">{track.title}</div>
              <div className="opacity-60 text-xs truncate">{track.artist}</div>
            </div>

            {/* Duration — highlight long tracks (>3 min) */}
            <span className={`text-xs ml-3 shrink-0 tabular-nums ${
              track.duration > 180
                ? 'text-yellow-300/80'
                : 'opacity-50'
            }`}>
              {formatDuration(track.duration)}
              {track.duration > 3600 && ' 🎧'}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}
