interface HowItWorksModalProps {
  onClose: () => void;
}

export default function HowItWorksModal({ onClose }: HowItWorksModalProps) {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 font-sans">
      <div className="bg-[#18181b] border border-gray-700 rounded-2xl max-w-md w-full p-6 text-white shadow-2xl">
        <h2 className="text-xl font-bold text-center mb-4">How it works</h2>

        <p className="text-sm text-center text-gray-300 mb-6 leading-relaxed">
          Preset.gg is a decentralized sound design copilot that allows music producers to search for any track
          on Audius, highlight a specific sound, and instantly generate a playable, royalty-free synthesizer preset
          for Vital and Serum (coming soon).
        </p>

        <div className="space-y-3 text-sm text-left text-gray-300 mb-8">
          <div className="flex items-start gap-2">
            <strong className="text-white shrink-0">Step 1:</strong>
            <div className="space-y-1">
              <p>Describe a sound that you would like to recreate.</p>
              <p className="text-xs text-gray-400"><strong className="text-gray-300">Hint:</strong> Using the built in search, find and select a reference song on Audius' open database. Concentrate your search window for even more accurate results.</p>
            </div>
          </div>
          <div className="flex items-start gap-2">
            <strong className="text-white shrink-0">Step 2:</strong>
            <p>Allow Preset.GG to listen and extract the exact synth parameters.</p>
          </div>
          <div className="flex items-start gap-2">
            <strong className="text-white shrink-0">Step 3:</strong>
            <p>Save and Download the preset for use in your DAW.</p>
          </div>
        </div>

        <button
          onClick={onClose}
          className="w-full py-3 bg-gradient-to-r from-[#CC0FE0] to-[#7E1BCC] hover:opacity-90 text-white font-bold rounded-lg transition-opacity"
        >
          I'm ready to generate
        </button>
      </div>
    </div>
  );
}
