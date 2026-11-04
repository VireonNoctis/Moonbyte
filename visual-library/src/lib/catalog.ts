export type GameCategory = "Strategy" | "Memory" | "Wordplay";

export type GameDefinition = {
  id: string;
  name: string;
  description: string;
  category: GameCategory;
  players: string;
  sessionType: "Solo session" | "Two-player lobby";
  duration: string;
  command: string;
  controls: string;
  implementationNote: string | null;
  glyph: string;
  palette: "mint" | "sun" | "lilac" | "coral";
  implemented: true;
};

export const games: GameDefinition[] = [
  {
    id: "tic-tac-toe",
    name: "Tic-tac-toe",
    description: "A compact duel of timing, space, and reading your opponent.",
    category: "Strategy",
    players: "2 players",
    sessionType: "Two-player lobby",
    duration: "2–5 min",
    command: ".play tic-tac-toe",
    controls: "Join the lobby, then submit row,column moves from 1 to 3.",
    implementationNote: null,
    glyph: "╳",
    palette: "coral",
    implemented: true,
  },
  {
    id: "connect-four",
    name: "Connect Four",
    description: "Drop discs, set traps, and connect four before the board closes.",
    category: "Strategy",
    players: "2 players",
    sessionType: "Two-player lobby",
    duration: "3–8 min",
    command: ".play connect-four",
    controls: "Join the lobby, then choose a column from 1 to 7.",
    implementationNote: null,
    glyph: "●",
    palette: "sun",
    implemented: true,
  },
  {
    id: "memory-match",
    name: "Memory Match",
    description: "Find four symbol pairs by building a mental map of the board.",
    category: "Memory",
    players: "1 player",
    sessionType: "Solo session",
    duration: "1–4 min",
    command: ".play memory-match",
    controls: "Reveal positions 1 to 8. The starter board is currently fixed.",
    implementationNote: "The starter build uses one fixed board. A larger board rotation is future game work, not a feature of this release.",
    glyph: "▦",
    palette: "lilac",
    implemented: true,
  },
  {
    id: "word-scramble",
    name: "Word Scramble",
    description: "Rearrange a five-letter word before your attempts run out.",
    category: "Wordplay",
    players: "1 player",
    sessionType: "Solo session",
    duration: "1–2 min",
    command: ".play word-scramble",
    controls: "Submit a word guess. The starter build contains one puzzle.",
    implementationNote: "The current build ships with one starter puzzle. A wider puzzle set is planned and is not presented as live content here.",
    glyph: "Aa",
    palette: "mint",
    implemented: true,
  },
];

export const categories = ["All games", "Strategy", "Memory", "Wordplay"] as const;
export type CategoryFilter = (typeof categories)[number];
