import { HelpCircle, Plus, ArrowRight } from 'lucide-react';

interface PromptInputProps {
  prompt: string;
  onPromptChange: (value: string) => void;
  disabled: boolean;
  onSearchToggle: () => void;
  onSubmit: (e?: React.FormEvent) => void;
  canSubmit: boolean;
  generationsUsed: number;
  generationsLimit: number;
}

export default function PromptInput({
  prompt,
  onPromptChange,
  disabled,
  onSearchToggle,
  onSubmit,
  canSubmit,
  generationsUsed,
  generationsLimit,
}: PromptInputProps) {
  return (
    <div className="flex items-center w-full relative">
      <form onSubmit={onSubmit} className="relative flex items-center w-full">
        <input
          type="text"
          value={prompt}
          onChange={(e) => onPromptChange(e.target.value)}
          disabled={disabled}
          placeholder="Describe a sound (e.g., heavy dubstep bass)..."
          className="w-full py-4 pl-6 pr-40 text-base bg-white/10 backdrop-blur-xl border border-white/20 rounded-full focus:outline-none focus:bg-white/20 focus:border-white/30 disabled:bg-black/10 disabled:text-white/40 placeholder-white/60 shadow-[0_8px_32px_rgba(0,0,0,0.15)] transition-all text-white"
        />

        <div className="absolute right-3 z-10 flex items-center space-x-2">
          <span className="text-[10px] text-white/70 font-pixel uppercase tracking-wider mt-0.5 mr-1">
            Gen: {generationsUsed}/{generationsLimit}
          </span>
          <button
            type="button"
            onClick={onSearchToggle}
            disabled={disabled}
            className="w-8 h-8 flex items-center justify-center rounded-full bg-white/10 hover:bg-white/20 border border-white/20 transition-all disabled:opacity-50"
            title="Search Audius"
          >
            <Plus className="w-4 h-4 text-white" />
          </button>
          <button
            type="submit"
            disabled={disabled || !canSubmit}
            className="w-8 h-8 flex items-center justify-center rounded-full bg-white/20 hover:bg-white/30 border border-white/30 transition-all disabled:opacity-50"
            title="Generate Preset"
          >
            <ArrowRight className="w-4 h-4 text-white" />
          </button>
        </div>
      </form>

      <div className="ml-4 relative group cursor-help flex items-center shrink-0">
        <HelpCircle className="w-6 h-6 text-white/80 hover:text-white transition-colors" />
        <div className="absolute bottom-full right-0 mb-3 w-64 p-3 bg-white/10 backdrop-blur-xl border border-white/20 rounded-xl text-xs hidden group-hover:block shadow-[0_8px_32px_rgba(0,0,0,0.15)] z-50 text-white font-sans">
          Selecting a reference track and focus window will greatly improve the accuracy of your requested preset!
        </div>
      </div>
    </div>
  );
}
