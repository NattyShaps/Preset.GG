import { useState, useEffect, useCallback, useRef } from 'react';
import type { TimeRange } from './TimeRangeSelector';

interface PlayButtonProps {
  audioRef: React.RefObject<HTMLAudioElement | null>;
  isPlaying: boolean;
  onPlayPause: () => void;
  onSeek: (time: number) => void;
  disabled?: boolean;
  /** Total duration in seconds (from Audius metadata) — used as fallback */
  trackDuration: number;
  /** When set, the ring represents ONLY this window */
  confirmedRange: TimeRange | null;
  size?: number;
}

/**
 * Circular play/pause button with a draggable progress ring.
 *
 * When `confirmedRange` is null, the ring represents the full track.
 * When set, the ring rescales so 0% = range.start and 100% = range.end.
 *
 * Click or drag on the ring to seek. The center button toggles play/pause.
 */
export default function PlayButton({
  audioRef,
  isPlaying,
  onPlayPause,
  onSeek,
  disabled = false,
  trackDuration,
  confirmedRange,
  size = 44,
}: PlayButtonProps) {
  // ── SVG ring geometry (declared early — used by pointer handlers) ────
  const strokeWidth = 2.5;
  const hitStrokeWidth = 24;      // Invisible fat ring for easier grabbing
  const radius = (size - strokeWidth) / 2;
  const circumference = 2 * Math.PI * radius;
  const center = size / 2;

  const [progress, setProgress] = useState(0);
  const [isSeeking, setIsSeeking] = useState(false);
  const svgRef = useRef<SVGSVGElement | null>(null);

  // ── Effective time boundaries ──────────────────────────────────────────
  const effectiveStart = confirmedRange?.start ?? 0;
  const effectiveEnd = confirmedRange?.end ?? trackDuration;
  const effectiveDuration = effectiveEnd - effectiveStart;

  // ── Progress sync ─────────────────────────────────────────────────────
  const updateProgress = useCallback(() => {
    if (isSeeking) return;           // Don't fight the drag
    const audio = audioRef.current;
    if (!audio) return;

    if (effectiveDuration > 0) {
      const p = (audio.currentTime - effectiveStart) / effectiveDuration;
      setProgress(Math.max(0, Math.min(1, p)));
    }
  }, [audioRef, effectiveStart, effectiveDuration, isSeeking]);

  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;
    audio.addEventListener('timeupdate', updateProgress);
    audio.addEventListener('seeked', updateProgress);
    return () => {
      audio.removeEventListener('timeupdate', updateProgress);
      audio.removeEventListener('seeked', updateProgress);
    };
  }, [audioRef, updateProgress]);

  // Reset when track or range changes
  useEffect(() => { setProgress(0); }, [trackDuration, confirmedRange]);

  // ── Angle ↔ time helpers ──────────────────────────────────────────────
  const angleToFraction = (clientX: number, clientY: number): number => {
    const svg = svgRef.current;
    if (!svg) return 0;
    const rect = svg.getBoundingClientRect();
    const cx = rect.left + rect.width / 2;
    const cy = rect.top + rect.height / 2;
    // atan2 gives angle from positive-x axis; we want 0 at top, clockwise
    let angle = Math.atan2(clientX - cx, -(clientY - cy));   // 0 = top
    if (angle < 0) angle += 2 * Math.PI;
    return angle / (2 * Math.PI);
  };

  const fractionToTime = (frac: number): number => {
    return effectiveStart + frac * effectiveDuration;
  };

  // ── Pointer interaction (mouse + touch) ───────────────────────────────
  const handlePointerDown = (e: React.PointerEvent<SVGSVGElement>) => {
    if (disabled) return;

    // Only start drag if the click is near the ring, not in the center
    const svg = svgRef.current;
    if (!svg) return;
    const rect = svg.getBoundingClientRect();
    const cx = rect.width / 2;
    const cy = rect.height / 2;
    const dx = e.clientX - (rect.left + cx);
    const dy = e.clientY - (rect.top + cy);
    const dist = Math.sqrt(dx * dx + dy * dy);
    const ringRadius = (size - strokeWidth) / 2;
    // Hit zone: within ±hitSlop of the ring radius
    const hitSlop = 14;
    if (dist < ringRadius - hitSlop) return;   // Too close to center → let play/pause handle it

    e.preventDefault();
    e.stopPropagation();
    (e.target as Element).setPointerCapture(e.pointerId);

    setIsSeeking(true);
    const frac = angleToFraction(e.clientX, e.clientY);
    setProgress(Math.max(0, Math.min(1, frac)));
  };

  const handlePointerMove = (e: React.PointerEvent<SVGSVGElement>) => {
    if (!isSeeking) return;
    const frac = angleToFraction(e.clientX, e.clientY);
    setProgress(Math.max(0, Math.min(1, frac)));
  };

  const handlePointerUp = (e: React.PointerEvent<SVGSVGElement>) => {
    if (!isSeeking) return;
    (e.target as Element).releasePointerCapture(e.pointerId);
    setIsSeeking(false);
    const frac = angleToFraction(e.clientX, e.clientY);
    const time = fractionToTime(Math.max(0, Math.min(1, frac)));
    onSeek(time);
  };

  // ── SVG derived values ─────────────────────────────────────────────
  const dashOffset = circumference * (1 - progress);

  // Ring color: white normally, accent when windowed
  const ringColor = confirmedRange
    ? 'rgba(168,130,255,0.9)'     // Soft purple accent when focus window active
    : 'rgba(255,255,255,0.85)';

  return (
    <div
      className="relative flex items-center justify-center"
      style={{ width: size, height: size }}
    >
      {/* SVG with drag interaction */}
      <svg
        ref={svgRef}
        width={size}
        height={size}
        className="absolute inset-0 -rotate-90"
        style={{ cursor: disabled ? 'default' : 'pointer', touchAction: 'none' }}
        onPointerDown={handlePointerDown}
        onPointerMove={handlePointerMove}
        onPointerUp={handlePointerUp}
        onPointerCancel={handlePointerUp}
      >
        {/* Invisible fat hit-area ring */}
        <circle
          cx={center}
          cy={center}
          r={radius}
          fill="none"
          stroke="transparent"
          strokeWidth={hitStrokeWidth}
        />
        {/* Visible background ring */}
        <circle
          cx={center}
          cy={center}
          r={radius}
          fill="none"
          stroke="rgba(255,255,255,0.15)"
          strokeWidth={strokeWidth}
        />
        {/* Visible progress ring */}
        <circle
          cx={center}
          cy={center}
          r={radius}
          fill="none"
          stroke={ringColor}
          strokeWidth={strokeWidth}
          strokeLinecap="round"
          strokeDasharray={circumference}
          strokeDashoffset={dashOffset}
          className={isSeeking ? '' : 'transition-[stroke-dashoffset] duration-200 ease-linear'}
        />
        {/* Drag handle dot at the progress tip */}
        {progress > 0.005 && (
          <circle
            cx={center + radius * Math.cos(progress * 2 * Math.PI)}
            cy={center + radius * Math.sin(progress * 2 * Math.PI)}
            r={isSeeking ? 4.5 : 3}
            fill="white"
            className={isSeeking ? '' : 'transition-all duration-200'}
          />
        )}
      </svg>

      {/* Play / Pause button (center area) */}
      <button
        onClick={onPlayPause}
        disabled={disabled}
        className="relative z-10 flex items-center justify-center w-full h-full rounded-full transition-colors group"
        title={isPlaying ? 'Pause' : 'Play'}
      >
        <span className="text-white/80 group-hover:text-white text-sm leading-none select-none">
          {isPlaying ? '⏸' : '▶'}
        </span>
      </button>
    </div>
  );
}
