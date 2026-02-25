import { X, FileAudio } from 'lucide-react';

interface SuccessModalProps {
  onClose: () => void;
}

export default function SuccessModal({ onClose }: SuccessModalProps) {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm">
      <div className="xp-outset w-[420px] shadow-[2px_2px_15px_rgba(0,0,0,0.5)] flex flex-col text-black">
        {/* Title Bar */}
        <div className="xp-titlebar cursor-default select-none">
          <span className="text-sm tracking-wide">File Download</span>
          <button
            onClick={onClose}
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
              onClick={onClose}
              className="px-5 py-1 bg-[#ECE9D8] border-2 border-t-white border-l-white border-b-gray-500 border-r-gray-500 active:border-t-gray-500 active:border-l-gray-500 active:border-b-white active:border-r-white text-sm focus:outline-black focus:outline-1 focus:outline-offset-[-4px]"
            >
              Open
            </button>
            <button
              onClick={onClose}
              className="px-5 py-1 bg-[#ECE9D8] border-2 border-t-white border-l-white border-b-gray-500 border-r-gray-500 active:border-t-gray-500 active:border-l-gray-500 active:border-b-white active:border-r-white text-sm focus:outline-black focus:outline-1 focus:outline-offset-[-4px]"
            >
              Save
            </button>
            <button
              onClick={onClose}
              className="px-5 py-1 bg-[#ECE9D8] border-2 border-t-white border-l-white border-b-gray-500 border-r-gray-500 active:border-t-gray-500 active:border-l-gray-500 active:border-b-white active:border-r-white text-sm focus:outline-black focus:outline-1 focus:outline-offset-[-4px]"
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
