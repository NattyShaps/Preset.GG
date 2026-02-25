interface HeaderProps {
  onHowItWorks: () => void;
}

export default function Header({ onHowItWorks }: HeaderProps) {
  return (
    <>
      <div className="absolute top-4 left-4 flex items-center space-x-6">
        <span className="text-white font-pixel text-2xl tracking-wider drop-shadow-md">Preset.GG</span>
        <button
          onClick={onHowItWorks}
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
    </>
  );
}
