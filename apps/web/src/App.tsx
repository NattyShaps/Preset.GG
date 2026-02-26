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
import { getStreamUrl } from './lib/audius';
import type { AudiusTrack } from './types/audius';

export type AppState = 'idle' | 'searching' | 'generating' | 'success';

export default function App() {
  const [appState, setAppState] = useState<AppState>('idle');
  const [prompt, setPrompt] = useState('');
  const [selectedTrack, setSelectedTrack] = useState<AudiusTrack | null>(null);
  const [showHowItWorks, setShowHowItWorks] = useState(false);

  // Confirmed (locked) time range — only set when user hits ✓ / Enter
  const [confirmedRange, setConfirmedRange] = useState<TimeRange | null>(null);

  const audiusSearch = useAudiusSearch();
  const audioRef = useRef<HTMLAudioElement | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  // Track whether the audio src has been set (to avoid re-setting on pause/play)
  const loadedSrcRef = useRef<string | null>(null);

  const isDisabled = appState === 'generating' || appState === 'success';

  // ── Helpers ─────────────────────────────────────────────────────────

  /** Ensure the audio element has the right src loaded */
  const ensureAudioLoaded = (track: AudiusTrack) => {
    const audio = audioRef.current;
    if (!audio) return;

    const url = getStreamUrl(track.id);
    if (loadedSrcRef.current !== url) {
      audio.src = url;
      loadedSrcRef.current = url;
    }
  };

  // ── Event handlers ────────────────────────────────────────────────────

  const handleGenerate = (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    if (!prompt && !selectedTrack) return;
    setAppState('generating');
    setTimeout(() => setAppState('success'), 3000);
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
    setAppState('idle');
    audiusSearch.clear();
    loadedSrcRef.current = null;
    if (audioRef.current) {
      audioRef.current.pause();
      setIsPlaying(false);
    }
  };

  const handleClearTrack = () => {
    setSelectedTrack(null);
    setConfirmedRange(null);
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
      // If currently playing, keep playing from new start
      // If paused, just seek — user will press play
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
          onPromptChange={setPrompt}
          disabled={isDisabled}
          onSearchToggle={handleSearchToggle}
          onSubmit={handleGenerate}
          canSubmit={!!(prompt || selectedTrack)}
        />

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
      {appState === 'success' && <SuccessModal onClose={() => setAppState('idle')} />}
    </div>
  );
}
