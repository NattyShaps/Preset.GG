interface HowItWorksModalProps {
  onClose: () => void;
}

export default function HowItWorksModal({ onClose }: HowItWorksModalProps) {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4 font-sans">
      <div className="bg-[#18181b] border border-gray-700 rounded-2xl max-w-md w-full p-6 text-white shadow-2xl">
        <h2 className="text-xl font-bold text-center mb-4">How it works</h2>

        <p className="text-sm text-center text-gray-300 mb-6 leading-relaxed">
          Preset.gg allows <span className="text-[#CC0FE0]">anyone</span> to reverse-engineer synth sounds. This
          tool is only possible because of the open Audius network and on-chain token utility, which let us bypass
          Web2 subscriptions and copyright friction to give you royalty-free presets.
        </p>

        <div className="space-y-3 text-sm text-center text-gray-300 mb-8">
          <p><strong className="text-white">Step 1:</strong> search audius for a track or describe a sound</p>
          <p><strong className="text-white">Step 2:</strong> let the ai listen and extract the exact synth parameters</p>
          <p><strong className="text-white">Step 3:</strong> download the .vital file and drag it into your daw</p>
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
