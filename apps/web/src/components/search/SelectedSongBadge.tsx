import type { AudiusTrack } from '@/types/audius';

interface SelectedSongBadgeProps {
  track: AudiusTrack;
  onClear: () => void;
  disabled: boolean;
}

export default function SelectedSongBadge({ track, onClear, disabled }: SelectedSongBadgeProps) {
  return (
    <div className="mt-4 flex items-center space-x-2 text-white font-pixel text-lg bg-white/10 border border-white/20 px-4 py-1.5 rounded-full backdrop-blur-md shadow-sm">
      {track.artworkUrl && (
        <img
          src={track.artworkUrl}
          alt=""
          className="w-6 h-6 rounded-sm object-cover shrink-0"
        />
      )}
      <span className="truncate max-w-[260px]">
        Ref: {track.title} – {track.artist}
      </span>
      <button
        onClick={onClear}
        className="text-white/60 hover:text-white px-1 transition-colors"
        disabled={disabled}
      >
        [x]
      </button>
    </div>
  );
}
