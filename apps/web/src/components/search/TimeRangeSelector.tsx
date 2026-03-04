import { useState, useEffect } from 'react';
import { formatDuration } from '@/lib/audius';
import { Clock, Check, X, Pencil } from 'lucide-react';
import WaveformRange from './WaveformRange';

/** Threshold in seconds — show the time range selector above this */
export const TIME_RANGE_THRESHOLD = 180; // 3 minutes

export interface TimeRange {
  start: number; // seconds
  end: number;   // seconds
}

interface TimeRangeSelectorProps {
  trackId: string;
  duration: number; // track duration in seconds
  /** The currently confirmed (locked) range, or null */
  confirmedRange: TimeRange | null;
  /** Called ONLY on confirm or clear — not on every keystroke */
  onConfirm: (range: TimeRange | null) => void;
  /** External audio element — waveform progress syncs to its currentTime */
  audioRef: React.RefObject<HTMLAudioElement | null>;
  /** Pre-fetched blob URL for instant waveform loading */
  blobUrl?: string | null;
  disabled?: boolean;
}

/**
 * Optional time range selector for long tracks / mixes.
 *
 * Lifecycle:
 *   collapsed → user clicks toggle → draft mode (waveform + drag handles)
 *   → user clicks ✓ → confirmed (locked waveform, ring rescales)
 *   → user clicks ✎ → back to draft → user clicks ✕ → cleared
 */
export default function TimeRangeSelector({
  trackId,
  duration,
  confirmedRange,
  onConfirm,
  audioRef,
  blobUrl,
  disabled = false,
}: TimeRangeSelectorProps) {
  // State machine: 'collapsed' | 'draft' | 'confirmed'
  type Phase = 'collapsed' | 'draft' | 'confirmed';
  const [phase, setPhase] = useState<Phase>(confirmedRange ? 'confirmed' : 'collapsed');

  // Draft values (local until confirmed)
  const [draftStart, setDraftStart] = useState(confirmedRange?.start ?? 0);
  const [draftEnd, setDraftEnd] = useState(confirmedRange?.end ?? Math.min(duration, 180));

  // Sync when track changes
  useEffect(() => {
    if (!confirmedRange) {
      setDraftStart(0);
      setDraftEnd(Math.min(duration, 180));
      setPhase('collapsed');
    }
  }, [duration]);

  // ── Handlers ──────────────────────────────────────────────────────────

  const handleToggleOpen = () => {
    setDraftStart(confirmedRange?.start ?? 0);
    setDraftEnd(confirmedRange?.end ?? Math.min(duration, 180));
    setPhase('draft');
  };

  const handleConfirm = () => {
    const range: TimeRange = { start: draftStart, end: draftEnd };
    onConfirm(range);
    setPhase('confirmed');
  };

  const handleEdit = () => {
    setDraftStart(confirmedRange?.start ?? draftStart);
    setDraftEnd(confirmedRange?.end ?? draftEnd);
    setPhase('draft');
  };

  const handleClear = () => {
    onConfirm(null);
    setPhase('collapsed');
  };

  const handleRangeChange = (start: number, end: number) => {
    setDraftStart(Math.max(0, Math.min(start, end - 1)));
    setDraftEnd(Math.max(start + 1, Math.min(end, duration)));
  };

  const toMMSS = (s: number) => formatDuration(s);

  // ── Render ────────────────────────────────────────────────────────────

  // Collapsed: just a toggle link
  if (phase === 'collapsed') {
    return (
      <div className="mt-3 w-full max-w-sm">
        <button
          type="button"
          onClick={handleToggleOpen}
          disabled={disabled}
          className="flex items-center gap-1.5 text-xs font-pixel text-white/50 hover:text-white/70 transition-colors"
        >
          <Clock className="w-3.5 h-3.5" />
          <span>Set focus window</span>
          <span className="text-white/40 ml-1">(track is {toMMSS(duration)})</span>
        </button>
      </div>
    );
  }

  // Confirmed: show locked waveform + edit / clear buttons
  if (phase === 'confirmed' && confirmedRange) {
    return (
      <div className="mt-3 w-full max-w-sm">
        <div className="flex items-center gap-1.5 text-xs font-pixel text-white/60 mb-1.5">
          <Clock className="w-3.5 h-3.5 text-purple-300" />
          <span className="text-purple-200">
            Focus: {toMMSS(confirmedRange.start)} – {toMMSS(confirmedRange.end)}
          </span>
          <span className="text-purple-300/50 text-[10px]">
            ({toMMSS(confirmedRange.end - confirmedRange.start)})
          </span>
          <div className="flex-1" />
          <button
            type="button"
            onClick={handleEdit}
            disabled={disabled}
            className="text-white/40 hover:text-white/70 transition-colors p-0.5"
            title="Edit focus window"
          >
            <Pencil className="w-3 h-3" />
          </button>
          <button
            type="button"
            onClick={handleClear}
            disabled={disabled}
            className="text-white/40 hover:text-red-300 transition-colors p-0.5"
            title="Remove focus window"
          >
            <X className="w-3.5 h-3.5" />
          </button>
        </div>
        {/* Locked waveform (non-interactive) */}
        <div className="bg-white/5 border border-purple-400/20 rounded-xl px-3 py-2">
          <WaveformRange
            trackId={trackId}
            duration={duration}
            startTime={confirmedRange.start}
            endTime={confirmedRange.end}
            onRangeChange={() => {}}
            audioRef={audioRef}
            blobUrl={blobUrl}
            disabled
          />
        </div>
      </div>
    );
  }

  // Draft: waveform with draggable handles + confirm / cancel buttons
  return (
    <div className="mt-3 w-full max-w-sm">
      <div className="flex items-center gap-1.5 text-xs font-pixel text-white mb-1.5">
        <Clock className="w-3.5 h-3.5" />
        <span>Set focus window</span>
        <span className="text-white/40 ml-1">(track is {toMMSS(duration)})</span>
      </div>
      <div className="bg-white/5 border border-white/20 rounded-xl px-3 py-2">
        <WaveformRange
          trackId={trackId}
          duration={duration}
          startTime={draftStart}
          endTime={draftEnd}
          onRangeChange={handleRangeChange}
          audioRef={audioRef}
          blobUrl={blobUrl}
          disabled={disabled}
        />
        {/* Action buttons */}
        <div className="flex items-center justify-end gap-2 mt-2">
          {/* Confirm */}
          <button
            type="button"
            onClick={handleConfirm}
            disabled={disabled}
            className="flex items-center justify-center w-6 h-6 rounded-md bg-white/10 hover:bg-green-500/30 border border-white/20 hover:border-green-400/40 text-white/60 hover:text-green-300 transition-all"
            title="Confirm focus window"
          >
            <Check className="w-3.5 h-3.5" />
          </button>
          {/* Cancel / clear */}
          <button
            type="button"
            onClick={handleClear}
            disabled={disabled}
            className="flex items-center justify-center w-6 h-6 rounded-md bg-white/10 hover:bg-red-500/20 border border-white/20 hover:border-red-400/30 text-white/60 hover:text-red-300 transition-all"
            title="Cancel"
          >
            <X className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>
    </div>
  );
}
