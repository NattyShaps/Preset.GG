import presetgglogo from '../../assets/presetgglogo.png';

interface SpinnerLogoProps {
  isGenerating: boolean;
}

export default function SpinnerLogo({ isGenerating }: SpinnerLogoProps) {
  return (
    <div
      className={`w-28 h-28 rounded-full bg-white shadow-[0_0_30px_rgba(0,0,0,0.2)] overflow-hidden flex items-center justify-center mb-8 transition-transform duration-1000 ${
        isGenerating ? 'animate-[spin_2s_linear_infinite]' : ''
      }`}
    >
      <img
        src={presetgglogo}
        alt="Preset.GG logo"
        className="w-24 h-24 object-contain object-center shrink-0 translate-x-1.5"
      />
    </div>
  );
}
