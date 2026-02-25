interface SelectedSongBadgeProps {
  songName: string;
  onClear: () => void;
  disabled: boolean;
}

export default function SelectedSongBadge({ songName, onClear, disabled }: SelectedSongBadgeProps) {
  return (
    <div className="mt-4 flex items-center space-x-2 text-white font-pixel text-lg bg-white/10 border border-white/20 px-4 py-1.5 rounded-full backdrop-blur-md shadow-sm">
      <span>Ref: {songName}</span>
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
