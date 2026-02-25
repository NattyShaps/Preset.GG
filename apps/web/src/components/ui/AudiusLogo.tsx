interface AudiusLogoProps {
  className?: string;
}

export default function AudiusLogo({ className }: AudiusLogoProps) {
  return (
    <svg viewBox="0 0 100 100" className={className} fill="none" xmlns="http://www.w3.org/2000/svg">
      <path fillRule="evenodd" clipRule="evenodd" d="M49.9999 15L15 75H35L49.9999 49.2857L65 75H85L49.9999 15ZM35 75L25 92.1428H75L65 75H35Z" fill="url(#audiusGradient)"/>
      <defs>
        <linearGradient id="audiusGradient" x1="15" y1="15" x2="85" y2="92.1428" gradientUnits="userSpaceOnUse">
          <stop stopColor="#CC0FE0"/>
          <stop offset="1" stopColor="#7E1BCC"/>
        </linearGradient>
      </defs>
    </svg>
  );
}
