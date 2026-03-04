import { useRef, useEffect, useState } from 'react';
import WaveSurfer from 'wavesurfer.js';
import RegionsPlugin, { type Region } from 'wavesurfer.js/dist/plugins/regions.js';
import { getStreamUrl, formatDuration } from '@/lib/audius';

interface WaveformRangeProps {
  trackId: string;
  duration: number;
  startTime: number;
  endTime: number;
  onRangeChange: (start: number, end: number) => void;
  /** External audio element — waveform progress syncs to its currentTime */
  audioRef: React.RefObject<HTMLAudioElement | null>;
  /** Pre-fetched blob URL — avoids re-downloading the audio */
  blobUrl?: string | null;
  disabled?: boolean;
}

/**
 * Waveform bar with two draggable edge handles for setting a time range.
 *
 * Uses wavesurfer.js (visual only — no audio playback) and its Regions
 * plugin to render a draggable/resizable region on top of the waveform.
 *
 * Playback progress is synced from an external <audio> element so the
 * waveform visually tracks the same position as the circular play button.
 */
export default function WaveformRange({
  trackId,
  duration,
  startTime,
  endTime,
  onRangeChange,
  audioRef,
  blobUrl,
  disabled = false,
}: WaveformRangeProps) {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const wsRef = useRef<WaveSurfer | null>(null);
  const regionRef = useRef<Region | null>(null);
  const [isReady, setIsReady] = useState(false);

  // Keep latest callback in a ref so the region listener never goes stale
  const onRangeChangeRef = useRef(onRangeChange);
  onRangeChangeRef.current = onRangeChange;

  // ── Create WaveSurfer + region on mount ────────────────────────────────
  // Track the URL we actually loaded so we can skip re-creation when the
  // waveform is already rendered from the stream and the blob arrives later.
  const loadedUrlRef = useRef<string | null>(null);

  const audioUrl = blobUrl || getStreamUrl(trackId);

  useEffect(() => {
    if (!containerRef.current) return;

    // If waveform is already rendered and the only change is that blob
    // arrived (stream → blob for the same track), skip re-creation.
    if (isReady && loadedUrlRef.current && loadedUrlRef.current !== audioUrl) {
      // Same track, just a different URL variant — keep the existing waveform
      return;
    }

    // If already loaded with this exact URL, skip
    if (loadedUrlRef.current === audioUrl && wsRef.current) return;

    setIsReady(false);

    const regions = RegionsPlugin.create();

    const ws = WaveSurfer.create({
      container: containerRef.current,
      url: audioUrl,
      height: 48,
      waveColor: 'rgba(255, 255, 255, 0.75)',
      progressColor: 'rgba(255, 180, 255, 1)',
      cursorWidth: 0,
      interact: false,
      barWidth: 2,
      barGap: 1,
      barRadius: 1,
      normalize: true,
      plugins: [regions],
    });

    wsRef.current = ws;
    loadedUrlRef.current = audioUrl;

    ws.on('ready', () => {
      // Mute WaveSurfer's internal audio — we only use it for visual sync
      ws.setVolume(0);

      setIsReady(true);

      const region = regions.addRegion({
        start: startTime,
        end: endTime,
        color: 'rgba(168, 130, 255, 0.55)',
        drag: !disabled,
        resize: !disabled,
        minLength: 10,  // at least 10 seconds
      });

      regionRef.current = region;

      // Fire on every drag frame so time labels update live
      region.on('update', () => {
        onRangeChangeRef.current(region.start, region.end);
      });

      // Also fire when drag ends (final position)
      region.on('update-end', () => {
        onRangeChangeRef.current(region.start, region.end);
      });
    });

    return () => {
      regionRef.current = null;
      loadedUrlRef.current = null;
      ws.destroy();
      wsRef.current = null;
    };
    // Recreate when trackId or audioUrl changes
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [trackId, audioUrl]);

  // ── Sync external audio progress → waveform visual ────────────────────
  useEffect(() => {
    const audio = audioRef.current;
    if (!audio) return;

    const handleTimeUpdate = () => {
      const ws = wsRef.current;
      if (!ws || !isReady) return;
      const wsDuration = ws.getDuration();
      if (wsDuration > 0) {
        const fraction = audio.currentTime / wsDuration;
        ws.seekTo(Math.max(0, Math.min(1, fraction)));
      }
    };

    audio.addEventListener('timeupdate', handleTimeUpdate);
    return () => audio.removeEventListener('timeupdate', handleTimeUpdate);
  }, [audioRef, isReady]);

  // ── Sync disabled prop to region ──────────────────────────────────────
  useEffect(() => {
    const region = regionRef.current;
    if (!region) return;
    region.setOptions({ drag: !disabled, resize: !disabled });
  }, [disabled]);

  // ── Render ────────────────────────────────────────────────────────────

  return (
    <div className="relative w-full">
      {/* Loading skeleton */}
      {!isReady && (
        <div className="absolute inset-0 flex items-center justify-center rounded-lg bg-white/5">
          <div className="h-8 w-full mx-4 rounded bg-white/10 animate-pulse" />
        </div>
      )}

      {/* WaveSurfer container */}
      <div
        ref={containerRef}
        className="w-full rounded-lg overflow-hidden"
        style={{ minHeight: 48, opacity: isReady ? 1 : 0 }}
      />

      {/* Time labels */}
      {isReady && (
        <div className="flex justify-between mt-1 px-0.5">
          <span className="text-[10px] font-mono text-white/50">
            {formatDuration(startTime)}
          </span>
          <span className="text-[10px] font-mono text-white/40">
            {formatDuration(endTime - startTime)}
          </span>
          <span className="text-[10px] font-mono text-white/50">
            {formatDuration(endTime)}
          </span>
        </div>
      )}
    </div>
  );
}
