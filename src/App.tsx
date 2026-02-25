import { useState } from 'react';
import { HelpCircle, Plus, Search, X, FileAudio, Play, ArrowRight } from 'lucide-react';

const AudiusLogo = ({ className }: { className?: string }) => (
  <svg viewBox="0 0 100 100" className={className} fill="none" xmlns="http://www.w3.org/2000/svg">
    <path fillRule="evenodd" clipRule="evenodd" d="M49.9999 15L15 75H35L49.9999 49.2857L65 75H85L49.9999 15ZM35 75L25 92.1428H75L65 75H35Z" fill="url(#audiusGradient)"/>
    <defs>
      <linearGradient id="audiusGradient" x1="15" y1="15" x2="85" y2="92.1428" gradientUnits="userSpaceOnUse">
        <stop stopColor="#CC0FE0"/>
        <stop offset="1" stopColor="#7E1BCC"/>
      </linearGradient>
    </defs>
  </svg>
);

export default function App() {
  const [appState, setAppState] = useState<'idle' | 'searching' | 'generating' | 'success'>('idle');
  const [prompt, setPrompt] = useState('');
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedSong, setSelectedSong] = useState<string | null>(null);
  const [showHowItWorks, setShowHowItWorks] = useState(false);

  const handleGenerate = (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    if (!prompt && !selectedSong) return;
    
    setAppState('generating');
    setTimeout(() => {
      setAppState('success');
    }, 3000);
  };

  const handleSearchToggle = () => {
    if (appState === 'searching') {
      setAppState('idle');
    } else {
      setAppState('searching');
    }
  };

  const mockSongs = [
    { id: 1, title: 'Bangarang', artist: 'Skrillex' },
    { id: 2, title: 'Scary Monsters and Nice Sprites', artist: 'Skrillex' },
    { id: 3, title: 'Cinema (Skrillex Remix)', artist: 'Benny Benassi' },
    { id: 4, title: 'First of the Year (Equinox)', artist: 'Skrillex' },
  ];

  return (
    <div className="min-h-screen bg-gradient-to-br from-[#CC0FE0] to-[#7E1BCC] flex items-center justify-center relative overflow-hidden font-xp">
      
      {/* Absolute Elements */}
      <div className="absolute top-4 left-4 flex items-center space-x-6">
        <span className="text-white font-pixel text-2xl tracking-wider drop-shadow-md">Preset.GG</span>
        <button 
          onClick={() => setShowHowItWorks(true)}
          className="text-white font-pixel text-sm px-2 py-1 xp-button"
        >
          [how it works]
        </button>
      </div>
      <div className="absolute top-4 right-4">
        <button className="text-white font-pixel text-sm px-2 py-1 xp-button">
          [connect wallet]
        </button>
      </div>
      
      {/* Bottom Widget */}
      <div className="absolute bottom-6 left-1/2 -translate-x-1/2 bg-black/20 backdrop-blur-sm px-4 py-1.5 rounded-full border border-white/10">
        <span className="text-white/70 text-xs font-xp tracking-wide">Powered by Audius</span>
      </div>

      {/* Center Stage */}
      <div className="w-full max-w-lg px-4 relative z-10 flex flex-col items-center">
        
        {/* Circular Shazam-style Logo */}
        <div className={`w-28 h-28 rounded-full bg-white shadow-[0_0_30px_rgba(0,0,0,0.2)] flex items-center justify-center mb-8 transition-transform duration-1000 ${appState === 'generating' ? 'animate-[spin_2s_linear_infinite]' : ''}`}>
          <AudiusLogo className="w-16 h-16" />
        </div>

        <div className="flex items-center w-full relative">
          <form onSubmit={handleGenerate} className="relative flex items-center w-full">
            {/* Main Input */}
            <input
              type="text"
              value={prompt}
              onChange={(e) => setPrompt(e.target.value)}
              disabled={appState === 'generating' || appState === 'success'}
              placeholder="Describe a sound (e.g., heavy dubstep bass)..."
              className="w-full py-4 pl-6 pr-40 text-base bg-white/10 backdrop-blur-xl border border-white/20 rounded-full focus:outline-none focus:bg-white/20 focus:border-white/30 disabled:bg-black/10 disabled:text-white/40 placeholder-white/60 shadow-[0_8px_32px_rgba(0,0,0,0.15)] transition-all text-white"
            />

            {/* Inside Right Icons */}
            <div className="absolute right-3 z-10 flex items-center space-x-2">
              <span className="text-[10px] text-white/70 font-pixel uppercase tracking-wider mt-0.5 mr-1">
                Gen: 1/1
              </span>
              <button 
                type="button"
                onClick={handleSearchToggle}
                disabled={appState === 'generating' || appState === 'success'}
                className="w-8 h-8 flex items-center justify-center rounded-full bg-white/10 hover:bg-white/20 border border-white/20 transition-all disabled:opacity-50"
                title="Search Audius"
              >
                <Plus className="w-4 h-4 text-white" />
              </button>
              <button 
                type="submit"
                disabled={appState === 'generating' || appState === 'success' || (!prompt && !selectedSong)}
                className="w-8 h-8 flex items-center justify-center rounded-full bg-white/20 hover:bg-white/30 border border-white/30 transition-all disabled:opacity-50"
                title="Generate Preset"
              >
                <ArrowRight className="w-4 h-4 text-white" />
              </button>
            </div>
          </form>

          {/* Outside Right Info Button */}
          <div className="ml-4 relative group cursor-help flex items-center shrink-0">
            <HelpCircle className="w-6 h-6 text-white/80 hover:text-white transition-colors" />
            <div className="absolute bottom-full right-0 mb-3 w-64 p-3 bg-white/10 backdrop-blur-xl border border-white/20 rounded-xl text-xs hidden group-hover:block shadow-[0_8px_32px_rgba(0,0,0,0.15)] z-50 text-white font-sans">
              Your request will be much better if there is a specific song to search.
            </div>
          </div>
        </div>

        {/* Selected Song Indicator */}
        {selectedSong && appState !== 'searching' && (
          <div className="mt-4 flex items-center space-x-2 text-white font-pixel text-lg bg-white/10 border border-white/20 px-4 py-1.5 rounded-full backdrop-blur-md shadow-sm">
            <span>Ref: {selectedSong}</span>
            <button 
              onClick={() => setSelectedSong(null)}
              className="text-white/60 hover:text-white px-1 transition-colors"
              disabled={appState === 'generating' || appState === 'success'}
            >
              [x]
            </button>
          </div>
        )}

        {/* Audius Search Dropdown */}
        {appState === 'searching' && (
          <div className="absolute top-full left-0 right-10 mt-3 bg-white/10 backdrop-blur-xl border border-white/20 rounded-2xl p-2 z-20 shadow-[0_8px_32px_rgba(0,0,0,0.2)]">
            <div className="bg-white/10 border border-white/20 rounded-xl flex items-center p-2 mb-2">
              <Search className="w-4 h-4 text-white/60 mr-2 ml-1" />
              <input 
                type="text" 
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                placeholder="Search Audius for a specific track..."
                className="w-full outline-none text-sm p-1 bg-transparent text-white placeholder-white/50"
                autoFocus
              />
            </div>
            
            <div className="max-h-48 overflow-y-auto rounded-lg">
              {mockSongs.filter(s => s.title.toLowerCase().includes(searchQuery.toLowerCase()) || s.artist.toLowerCase().includes(searchQuery.toLowerCase())).map(song => (
                <div 
                  key={song.id}
                  onClick={() => {
                    setSelectedSong(`${song.title} - ${song.artist}`);
                    setAppState('idle');
                  }}
                  className="p-2 hover:bg-white/20 cursor-pointer text-sm flex items-center border-b border-white/10 last:border-0 text-white transition-colors rounded-lg mb-1"
                >
                  <Play className="w-3 h-3 mr-2 opacity-60" />
                  <span className="font-bold mr-1">{song.title}</span>
                  <span className="opacity-75">by {song.artist}</span>
                </div>
              ))}
              {mockSongs.filter(s => s.title.toLowerCase().includes(searchQuery.toLowerCase()) || s.artist.toLowerCase().includes(searchQuery.toLowerCase())).length === 0 && (
                <div className="p-4 text-center text-white/60 text-sm">
                  No tracks found.
                </div>
              )}
            </div>
          </div>
        )}

      </div>

      {/* How It Works Modal (Pump.fun style) */}
      {showHowItWorks && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 font-sans">
          <div className="bg-[#18181b] border border-gray-700 rounded-2xl max-w-md w-full p-6 text-white shadow-2xl">
            <h2 className="text-xl font-bold text-center mb-4">How it works</h2>
            
            <p className="text-sm text-center text-gray-300 mb-6 leading-relaxed">
              Preset.gg allows <span className="text-[#CC0FE0]">anyone</span> to reverse-engineer synth sounds. This tool is only possible because of the open Audius network and on-chain token utility, which let us bypass Web2 subscriptions and copyright friction to give you royalty-free presets.
            </p>
            
            <div className="space-y-3 text-sm text-center text-gray-300 mb-8">
              <p><strong className="text-white">Step 1:</strong> search audius for a track or describe a sound</p>
              <p><strong className="text-white">Step 2:</strong> let the ai listen and extract the exact synth parameters</p>
              <p><strong className="text-white">Step 3:</strong> download the .vital file and drag it into your daw</p>
            </div>
            
            <button 
              onClick={() => setShowHowItWorks(false)}
              className="w-full py-3 bg-gradient-to-r from-[#CC0FE0] to-[#7E1BCC] hover:opacity-90 text-white font-bold rounded-lg transition-opacity"
            >
              I'm ready to generate
            </button>
          </div>
        </div>
      )}

      {/* Success State (XP Modal) */}
      {appState === 'success' && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm">
          <div className="xp-outset w-[420px] shadow-[2px_2px_15px_rgba(0,0,0,0.5)] flex flex-col text-black">
            {/* Title Bar */}
            <div className="xp-titlebar cursor-default select-none">
              <span className="text-sm tracking-wide">File Download</span>
              <button 
                onClick={() => setAppState('idle')}
                className="w-[21px] h-[21px] bg-[#E95F4A] border border-white flex items-center justify-center hover:bg-[#f0705c] active:bg-[#d0402b] rounded-sm ml-2 transition-colors"
              >
                <X className="w-4 h-4 text-white" strokeWidth={3} />
              </button>
            </div>

            {/* Content */}
            <div className="p-4 flex flex-col">
              <div className="flex items-start mb-6">
                <FileAudio className="w-10 h-10 text-blue-600 mr-4 shrink-0" strokeWidth={1.5} />
                <div className="text-sm">
                  <p className="mb-3">Do you want to open or save this file?</p>
                  <p className="mb-1">Name: <strong>preset_generated.vital</strong></p>
                  <p>Type: Vital Synth Preset</p>
                </div>
              </div>

              {/* Buttons */}
              <div className="flex justify-end space-x-2 mt-auto">
                <button 
                  onClick={() => setAppState('idle')}
                  className="px-5 py-1 bg-[#ECE9D8] border-2 border-t-white border-l-white border-b-gray-500 border-r-gray-500 active:border-t-gray-500 active:border-l-gray-500 active:border-b-white active:border-r-white text-sm focus:outline-black focus:outline-1 focus:outline-offset-[-4px]"
                >
                  Open
                </button>
                <button 
                  onClick={() => setAppState('idle')}
                  className="px-5 py-1 bg-[#ECE9D8] border-2 border-t-white border-l-white border-b-gray-500 border-r-gray-500 active:border-t-gray-500 active:border-l-gray-500 active:border-b-white active:border-r-white text-sm focus:outline-black focus:outline-1 focus:outline-offset-[-4px]"
                >
                  Save
                </button>
                <button 
                  onClick={() => setAppState('idle')}
                  className="px-5 py-1 bg-[#ECE9D8] border-2 border-t-white border-l-white border-b-gray-500 border-r-gray-500 active:border-t-gray-500 active:border-l-gray-500 active:border-b-white active:border-r-white text-sm focus:outline-black focus:outline-1 focus:outline-offset-[-4px]"
                >
                  Cancel
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

    </div>
  );
}
