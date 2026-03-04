import { Loader2 } from 'lucide-react';

interface GeneratingPresetWidgetProps {
  visible: boolean;
  isSpinning: boolean;
}

export default function GeneratingPresetWidget({ visible, isSpinning }: GeneratingPresetWidgetProps) {
  return (
    <div
      aria-hidden={!visible}
      className={`absolute left-1/2 -top-10 z-20 -translate-x-1/2 pointer-events-none transition-all duration-300 ease-out ${
        visible ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-2'
      }`}
    >
      <div className="bg-black/20 backdrop-blur-sm px-4 py-1.5 rounded-full border border-white/10 shadow-[0_8px_24px_rgba(0,0,0,0.2)] flex items-center gap-2">
        <Loader2 className={`w-3.5 h-3.5 text-white/70 ${isSpinning ? 'animate-spin' : ''}`} />
        <span className="text-white/70 text-xs font-xp tracking-wide">Generating Preset...</span>
      </div>
    </div>
  );
}
