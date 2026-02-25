import AudiusLogo from '../ui/AudiusLogo';

interface SpinnerLogoProps {
  isGenerating: boolean;
}

export default function SpinnerLogo({ isGenerating }: SpinnerLogoProps) {
  return (
    <div
      className={`w-28 h-28 rounded-full bg-white shadow-[0_0_30px_rgba(0,0,0,0.2)] flex items-center justify-center mb-8 transition-transform duration-1000 ${
        isGenerating ? 'animate-[spin_2s_linear_infinite]' : ''
      }`}
    >
      <AudiusLogo className="w-16 h-16" />
    </div>
  );
}
