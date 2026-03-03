import { useState } from 'react';
import { X, FileAudio } from 'lucide-react';
import type { GenerationResult } from '@/hooks/usePresetGeneration';

interface SuccessModalProps {
  result: GenerationResult;
  onClose: () => void;
}

/**
 * Decode base64 preset data and trigger a browser file download.
 */
function downloadPreset(result: GenerationResult) {
  try {
    // Decode base64 to raw bytes.
    const binaryString = atob(result.presetData);
    const bytes = new Uint8Array(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i);
    }

    // Create a Blob and trigger download.
    const blob = new Blob([bytes], { type: 'application/octet-stream' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = result.fileName;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  } catch (err) {
    console.error('Download failed:', err);
  }
}

export default function SuccessModal({ result, onClose }: SuccessModalProps) {
  const [hasDownloaded, setHasDownloaded] = useState(false);

  const handleDownload = () => {
    if (hasDownloaded) return;
    setHasDownloaded(true);
    downloadPreset(result);
    onClose();
  };

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
              <p className="mb-1">Name: <strong>{result.fileName}</strong></p>
              <p>Type: {result.format === 'vital' ? 'Vital Synth Preset' : 'Serum Preset'}</p>
            </div>
          </div>

          {/* Buttons */}
          <div className="flex justify-end space-x-2 mt-auto">
            <button
              onClick={handleDownload}
              disabled={hasDownloaded}
              className="px-5 py-1 bg-[#ECE9D8] border-2 border-t-white border-l-white border-b-gray-500 border-r-gray-500 active:border-t-gray-500 active:border-l-gray-500 active:border-b-white active:border-r-white text-sm focus:outline-black focus:outline-1 focus:outline-offset-[-4px] disabled:opacity-50 disabled:cursor-default disabled:active:border-t-white disabled:active:border-l-white disabled:active:border-b-gray-500 disabled:active:border-r-gray-500"
            >
              Open
            </button>
            <button
              onClick={handleDownload}
              disabled={hasDownloaded}
              className="px-5 py-1 bg-[#ECE9D8] border-2 border-t-white border-l-white border-b-gray-500 border-r-gray-500 active:border-t-gray-500 active:border-l-gray-500 active:border-b-white active:border-r-white text-sm focus:outline-black focus:outline-1 focus:outline-offset-[-4px] disabled:opacity-50 disabled:cursor-default disabled:active:border-t-white disabled:active:border-l-white disabled:active:border-b-gray-500 disabled:active:border-r-gray-500"
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
