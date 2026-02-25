import { useState } from 'react';
import Header from './components/layout/Header';
import Footer from './components/layout/Footer';
import SpinnerLogo from './components/layout/SpinnerLogo';
import PromptInput from './components/prompt/PromptInput';
import SelectedSongBadge from './components/search/SelectedSongBadge';
import SearchDropdown, { type MockSong } from './components/search/SearchDropdown';
import HowItWorksModal from './components/ui/HowItWorksModal';
import SuccessModal from './components/preset/SuccessModal';

const MOCK_SONGS: MockSong[] = [
  { id: 1, title: 'Bangarang', artist: 'Skrillex' },
  { id: 2, title: 'Scary Monsters and Nice Sprites', artist: 'Skrillex' },
  { id: 3, title: 'Cinema (Skrillex Remix)', artist: 'Benny Benassi' },
  { id: 4, title: 'First of the Year (Equinox)', artist: 'Skrillex' },
];

export type AppState = 'idle' | 'searching' | 'generating' | 'success';

export default function App() {
  const [appState, setAppState] = useState<AppState>('idle');
  const [prompt, setPrompt] = useState('');
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedSong, setSelectedSong] = useState<string | null>(null);
  const [showHowItWorks, setShowHowItWorks] = useState(false);

  const isDisabled = appState === 'generating' || appState === 'success';

  const handleGenerate = (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    if (!prompt && !selectedSong) return;

    setAppState('generating');
    setTimeout(() => {
      setAppState('success');
    }, 3000);
  };

  const handleSearchToggle = () => {
    setAppState(appState === 'searching' ? 'idle' : 'searching');
  };

  const handleSelectSong = (song: MockSong) => {
    setSelectedSong(`${song.title} - ${song.artist}`);
    setAppState('idle');
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-[#CC0FE0] to-[#7E1BCC] flex items-center justify-center relative overflow-hidden font-xp">
      <Header onHowItWorks={() => setShowHowItWorks(true)} />
      <Footer />

      {/* Center Stage */}
      <div className="w-full max-w-lg px-4 relative z-10 flex flex-col items-center">
        <SpinnerLogo isGenerating={appState === 'generating'} />

        <PromptInput
          prompt={prompt}
          onPromptChange={setPrompt}
          disabled={isDisabled}
          onSearchToggle={handleSearchToggle}
          onSubmit={handleGenerate}
          canSubmit={!!(prompt || selectedSong)}
        />

        {selectedSong && appState !== 'searching' && (
          <SelectedSongBadge
            songName={selectedSong}
            onClear={() => setSelectedSong(null)}
            disabled={isDisabled}
          />
        )}

        {appState === 'searching' && (
          <SearchDropdown
            searchQuery={searchQuery}
            onSearchQueryChange={setSearchQuery}
            songs={MOCK_SONGS}
            onSelectSong={handleSelectSong}
          />
        )}
      </div>

      {/* Modals */}
      {showHowItWorks && <HowItWorksModal onClose={() => setShowHowItWorks(false)} />}
      {appState === 'success' && <SuccessModal onClose={() => setAppState('idle')} />}
    </div>
  );
}
