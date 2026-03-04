interface AudiusLogoProps {
  className?: string;
}

export default function AudiusLogo({ className }: AudiusLogoProps) {
  return (
    <svg viewBox="0 0 100 100" className={className} fill="none" xmlns="http://www.w3.org/2000/svg">
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M18 8 L38 8 L82 33 L38 58 L18 58 Z M38 20 L62 33 L38 46 Z M18 58 L38 58 L42 92 L14 92 Z"
        fill="url(#presetGradient)"
      />
      <defs>
        <linearGradient id="presetGradient" x1="18" y1="8" x2="85" y2="92" gradientUnits="userSpaceOnUse">
          <stop stopColor="#CC0FE0"/>
          <stop offset="1" stopColor="#7E1BCC"/>
        </linearGradient>
      </defs>
    </svg>
  );
}
