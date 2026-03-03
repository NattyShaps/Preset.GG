import { useState, useRef } from 'react';
import Header from './components/layout/Header';
import Footer from './components/layout/Footer';
import SpinnerLogo from './components/layout/SpinnerLogo';
import PromptInput from './components/prompt/PromptInput';
import SelectedSongBadge from './components/search/SelectedSongBadge';
import SearchDropdown from './components/search/SearchDropdown';
import PlayButton from './components/search/PlayButton';
import TimeRangeSelector, { TIME_RANGE_THRESHOLD, type TimeRange } from './components/search/TimeRangeSelector';
import HowItWorksModal from './components/ui/HowItWorksModal';
import SuccessModal from './components/preset/SuccessModal';
import { useAudiusSearch } from './hooks/useAudiusSearch';
import { usePresetGeneration } from './hooks/usePresetGeneration';
import { getStreamUrl, fetchAudioBlob } from './lib/audius';
import type { AudiusTrack } from './types/audius';

export type AppState = 'idle' | 'searching' | 'generating' | 'success';

export default function App() {
  const [appState, setAppState] = useState<AppState>('idle');
  const [prompt, setPrompt] = useState('');
  const [selectedTrack, setSelectedTrack] = useState<AudiusTrack | null>(null);
  const [showHowItWorks, setShowHowItWorks] = useState(false);

  // Confirmed (locked) time range — only set when user hits ✓ / Enter
  const [confirmedRange, setConfirmedRange] = useState<TimeRange | null>(null);

  // Generation error shown inline below the prompt
  const [generationError, setGenerationError] = useState<string | null>(null);

  // Generation usage counter (updated from API responses)
  const [genUsed, setGenUsed] = useState(0);
  const [genLimit, setGenLimit] = useState(5);

  const audiusSearch = useAudiusSearch();
  const presetGeneration = usePresetGeneration();
  const audioRef = useRef<HTMLAudioElement | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  // Track whether the audio src has been set (to avoid re-setting on pause/play)
  const loadedSrcRef = useRef<string | null>(null);
  // Blob URL for full-file seeking support
  const blobUrlRef = useRef<string | null>(null);
  const fetchControllerRef = useRef<AbortController | null>(null);

  const isDisabled = appState === 'generating' || appState === 'success';

  // ── Helpers ─────────────────────────────────────────────────────────

  /** Ensure the audio element has the right src loaded */
  const ensureAudioLoaded = (track: AudiusTrack) => {
    const audio = audioRef.current;
    if (!audio) return;

    // Prefer blob URL (supports seeking); fall back to stream URL
    const url = blobUrlRef.current ?? getStreamUrl(track.id);
    if (loadedSrcRef.current !== url) {
      audio.src = url;
      loadedSrcRef.current = url;
    }
  };

  /** Kick off a background fetch of the full audio file as a blob */
  const startBlobFetch = (track: AudiusTrack) => {
    // Already have a blob for this track
    if (blobUrlRef.current) return;
    // Already fetching
    if (fetchControllerRef.current) return;

    const controller = new AbortController();
    fetchControllerRef.current = controller;

    fetchAudioBlob(track.id, controller.signal)
      .then((blobUrl) => {
        fetchControllerRef.current = null;
        blobUrlRef.current = blobUrl;

        // Swap the audio src to the blob URL, preserving playback position
        const audio = audioRef.current;
        if (audio) {
          const wasPlaying = !audio.paused;
          const pos = audio.currentTime;
          audio.src = blobUrl;
          loadedSrcRef.current = blobUrl;
          audio.currentTime = pos;
          if (wasPlaying) audio.play();
        }
      })
      .catch((err) => {
        fetchControllerRef.current = null;
        if (err.name !== 'AbortError') {
          console.error('Blob fetch failed (seeking will be limited):', err);
        }
      });
  };

  /** Clean up blob URL and cancel in-flight fetch */
  const cleanupBlob = () => {
    if (fetchControllerRef.current) {
      fetchControllerRef.current.abort();
      fetchControllerRef.current = null;
    }
    if (blobUrlRef.current) {
      URL.revokeObjectURL(blobUrlRef.current);
      blobUrlRef.current = null;
    }
  };

  // ── Event handlers ────────────────────────────────────────────────────

  const handleGenerate = async (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    if (!prompt && !selectedTrack) return;

    // Clear any previous error
    setGenerationError(null);
    setAppState('generating');

    // Pause audio during generation
    if (audioRef.current && isPlaying) {
      audioRef.current.pause();
      setIsPlaying(false);
    }

    await presetGeneration.generate({
      prompt: prompt || undefined,
      trackId: selectedTrack?.id || undefined,
      startTimestamp: confirmedRange?.start,
      endTimestamp: confirmedRange?.end,
    });

    // Check result after generate completes
    // Note: generate() sets its own state, we check it here
  };

  // Sync generation hook state → appState
  // We use the hook's state directly rather than useEffect to keep it simple
  if (presetGeneration.result && appState === 'generating') {
    setGenUsed(presetGeneration.result.generationsUsed);
    setGenLimit(presetGeneration.result.generationsLimit);
    setAppState('success');
  }
  if (presetGeneration.error && appState === 'generating') {
    setGenerationError(presetGeneration.error);
    setAppState('idle');
  }

  const handleSuccessClose = () => {
    presetGeneration.reset();
    setAppState('idle');
  };

  const handleSearchToggle = () => {
    if (appState === 'searching') {
      setAppState('idle');
      audiusSearch.clear();
    } else {
      setAppState('searching');
    }
  };

  const handleSelectTrack = (track: AudiusTrack) => {
    setSelectedTrack(track);
    setConfirmedRange(null);
    setGenerationError(null);
    setAppState('idle');
    audiusSearch.clear();
    cleanupBlob();
    loadedSrcRef.current = null;
    if (audioRef.current) {
      audioRef.current.pause();
      setIsPlaying(false);
    }
  };

  const handleClearTrack = () => {
    setSelectedTrack(null);
    setConfirmedRange(null);
    setGenerationError(null);
    cleanupBlob();
    loadedSrcRef.current = null;
    if (audioRef.current) {
      audioRef.current.pause();
      setIsPlaying(false);
    }
  };

  const handlePlayPause = () => {
    if (!audioRef.current || !selectedTrack) return;

    if (isPlaying) {
      audioRef.current.pause();
      setIsPlaying(false);
    } else {
      ensureAudioLoaded(selectedTrack);
      startBlobFetch(selectedTrack);

      // If a confirmed range exists and we're outside it, seek to start
      if (confirmedRange) {
        const t = audioRef.current.currentTime;
        if (t < confirmedRange.start || t >= confirmedRange.end) {
          audioRef.current.currentTime = confirmedRange.start;
        }
      }

      audioRef.current.play().then(() => {
        setIsPlaying(true);
      }).catch((err) => {
        console.error('Playback failed:', err);
        setIsPlaying(false);
      });
    }
  };

  /** Called by PlayButton when user drags the ring */
  const handleSeek = (time: number) => {
    const audio = audioRef.current;
    if (!audio || !selectedTrack) return;

    ensureAudioLoaded(selectedTrack);

    // If blob isn't loaded yet, clamp to the buffered range
    if (!blobUrlRef.current && audio.buffered.length > 0) {
      const bufferedEnd = audio.buffered.end(audio.buffered.length - 1);
      if (time > bufferedEnd) {
        time = bufferedEnd;
      }
    }

    audio.currentTime = time;
  };

  /** Called when audio timeupdate fires — enforce range end boundary */
  const handleTimeUpdate = () => {
    if (!audioRef.current || !confirmedRange) return;
    if (audioRef.current.currentTime >= confirmedRange.end) {
      audioRef.current.pause();
      setIsPlaying(false);
      // Snap to end so ring shows 100%
      audioRef.current.currentTime = confirmedRange.end;
    }
  };

  /** Called by TimeRangeSelector ONLY on confirm or clear */
  const handleRangeConfirm = (range: TimeRange | null) => {
    setConfirmedRange(range);
    if (audioRef.current && range) {
      audioRef.current.currentTime = range.start;
    }
  };

  // ── Render ────────────────────────────────────────────────────────────

  return (
    <div className="min-h-screen bg-gradient-to-br from-[#CC0FE0] to-[#7E1BCC] flex items-center justify-center relative overflow-hidden font-xp">
      <Header onHowItWorks={() => setShowHowItWorks(true)} />
      <Footer />

      {/* Hidden audio element */}
      <audio
        ref={audioRef}
        onEnded={() => setIsPlaying(false)}
        onTimeUpdate={handleTimeUpdate}
        onError={(e) => {
          console.error('Audio error:', e);
          setIsPlaying(false);
        }}
      />

      {/* Center Stage */}
      <div className="w-full max-w-lg px-4 relative z-10 flex flex-col items-center">
        <SpinnerLogo isGenerating={appState === 'generating'} />

        <PromptInput
          prompt={prompt}
          onPromptChange={(v) => { setPrompt(v); setGenerationError(null); }}
          disabled={isDisabled}
          onSearchToggle={handleSearchToggle}
          onSubmit={handleGenerate}
          canSubmit={!!(prompt || selectedTrack)}
          generationsUsed={genUsed}
          generationsLimit={genLimit}
        />

        {/* Generation error */}
        {generationError && appState === 'idle' && (
          <div className="mt-2 text-red-300 text-xs text-center max-w-sm">
            {generationError}
          </div>
        )}

        {selectedTrack && appState !== 'searching' && (
          <div className="flex flex-col items-center">
            <div className="flex items-center gap-2">
              <SelectedSongBadge
                track={selectedTrack}
                onClear={handleClearTrack}
                disabled={isDisabled}
              />
              <div className="mt-4">
                <PlayButton
                  audioRef={audioRef}
                  isPlaying={isPlaying}
                  onPlayPause={handlePlayPause}
                  onSeek={handleSeek}
                  disabled={isDisabled}
                  trackDuration={selectedTrack.duration}
                  confirmedRange={confirmedRange}
                />
              </div>
            </div>

            {/* Time range selector for long tracks (>3 min) */}
            {selectedTrack.duration > TIME_RANGE_THRESHOLD && (
              <TimeRangeSelector
                duration={selectedTrack.duration}
                confirmedRange={confirmedRange}
                onConfirm={handleRangeConfirm}
                disabled={isDisabled}
              />
            )}
          </div>
        )}

        {appState === 'searching' && (
          <SearchDropdown
            searchQuery={audiusSearch.query}
            onSearchQueryChange={audiusSearch.search}
            results={audiusSearch.results}
            isLoading={audiusSearch.isLoading}
            onSelectTrack={handleSelectTrack}
          />
        )}

        {/* Search error display */}
        {audiusSearch.error && appState === 'searching' && (
          <div className="mt-2 text-red-300 text-xs text-center">
            {audiusSearch.error}
          </div>
        )}
      </div>

      {/* Modals */}
      {showHowItWorks && <HowItWorksModal onClose={() => setShowHowItWorks(false)} />}
      {appState === 'success' && presetGeneration.result && (
        <SuccessModal
          result={presetGeneration.result}
          onClose={handleSuccessClose}
        />
      )}
    </div>
  );
}
