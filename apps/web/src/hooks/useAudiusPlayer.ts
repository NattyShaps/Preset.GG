/**
 * Hook for Audius track playback and timestamp selection.
 * TODO: Integrate waveform library (wavesurfer.js) and Audius streaming
 */
import { useState } from 'react';

interface PlayerState {
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  startTimestamp: number | null;
  endTimestamp: number | null;
}

export function useAudiusPlayer() {
  const [state, setState] = useState<PlayerState>({
    isPlaying: false,
    currentTime: 0,
    duration: 0,
    startTimestamp: null,
    endTimestamp: null,
  });

  const play = () => setState((s) => ({ ...s, isPlaying: true }));
  const pause = () => setState((s) => ({ ...s, isPlaying: false }));
  const seek = (time: number) => setState((s) => ({ ...s, currentTime: time }));

  const setTimestampRange = (start: number, end: number) => {
    setState((s) => ({ ...s, startTimestamp: start, endTimestamp: end }));
  };

  const clearTimestampRange = () => {
    setState((s) => ({ ...s, startTimestamp: null, endTimestamp: null }));
  };

  return { ...state, play, pause, seek, setTimestampRange, clearTimestampRange };
}
