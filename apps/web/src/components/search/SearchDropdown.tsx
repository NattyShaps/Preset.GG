import { Search, Play } from 'lucide-react';

export interface MockSong {
  id: number;
  title: string;
  artist: string;
}

interface SearchDropdownProps {
  searchQuery: string;
  onSearchQueryChange: (value: string) => void;
  songs: MockSong[];
  onSelectSong: (song: MockSong) => void;
}

export default function SearchDropdown({
  searchQuery,
  onSearchQueryChange,
  songs,
  onSelectSong,
}: SearchDropdownProps) {
  const filteredSongs = songs.filter(
    (s) =>
      s.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      s.artist.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="absolute top-full left-0 right-10 mt-3 bg-white/10 backdrop-blur-xl border border-white/20 rounded-2xl p-2 z-20 shadow-[0_8px_32px_rgba(0,0,0,0.2)]">
      <div className="bg-white/10 border border-white/20 rounded-xl flex items-center p-2 mb-2">
        <Search className="w-4 h-4 text-white/60 mr-2 ml-1" />
        <input
          type="text"
          value={searchQuery}
          onChange={(e) => onSearchQueryChange(e.target.value)}
          placeholder="Search Audius for a specific track..."
          className="w-full outline-none text-sm p-1 bg-transparent text-white placeholder-white/50"
          autoFocus
        />
      </div>

      <div className="max-h-48 overflow-y-auto rounded-lg">
        {filteredSongs.map((song) => (
          <div
            key={song.id}
            onClick={() => onSelectSong(song)}
            className="p-2 hover:bg-white/20 cursor-pointer text-sm flex items-center border-b border-white/10 last:border-0 text-white transition-colors rounded-lg mb-1"
          >
            <Play className="w-3 h-3 mr-2 opacity-60" />
            <span className="font-bold mr-1">{song.title}</span>
            <span className="opacity-75">by {song.artist}</span>
          </div>
        ))}
        {filteredSongs.length === 0 && (
          <div className="p-4 text-center text-white/60 text-sm">
            No tracks found.
          </div>
        )}
      </div>
    </div>
  );
}
