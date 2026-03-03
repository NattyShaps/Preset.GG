interface HeaderProps {
  onHowItWorks: () => void;
}

export default function Header({ onHowItWorks }: HeaderProps) {
  return (
    <>
      <div className="absolute top-4 left-4 flex items-center space-x-6">
        <div className="flex items-baseline">
          <span className="text-white font-pixel text-2xl tracking-wider drop-shadow-md">Preset.GG</span>
          <span className="text-white/50 font-pixel text-[10px] ml-1.5 tracking-wider">(beta)</span>
        </div>
        <button
          onClick={onHowItWorks}
          className="text-white font-pixel text-sm px-2 py-1 xp-button"
        >
          [how it works]
        </button>
      </div>
      <div className="absolute top-4 right-4">
        <button
          disabled
          className="text-white/40 font-pixel text-sm px-2 py-1 xp-button cursor-not-allowed opacity-50"
          title="Coming soon"
        >
          [connect wallet]
        </button>
      </div>
    </>
  );
}
