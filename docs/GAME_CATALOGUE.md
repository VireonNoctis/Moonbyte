# Moonbyte Game Catalogue

- Status: **Approved product catalogue with a first implemented slice.** The four titles in the implementation snapshot below are runnable; the rest remain approved candidates, not implemented merely by being listed.
- The catalogue contains **39 unique approved new-game titles**. Minesweeper and Mastermind are intentionally cross-listed in two categories but have one canonical implementation each.
- The two standard Lunar baseline games, **Akinator** and **8-ball**, remain inherited baseline experiences and were not part of the new-game approval list.
- Release in polished, tested groups. Approval of the catalogue is not a promise that every title ships in the first release.

## 1. Existing Lunar baseline

The following are inherited baseline experiences, separate from the approved new-game catalogue:

| Game | Moonbyte treatment |
| --- | --- |
| Akinator | Preserve the standard guessing experience after reviewing the implementation, upstream service/API terms, dependencies, and privacy behavior. |
| 8-ball | Preserve the standard casual response experience. Keep it for entertainment and never connect it to wagers, stakes, or prizes. |

Other Lunar features are migration candidates only after source, dependency, privacy, and licence review. Do not assume that every Lunar command is automatically approved for migration.

## 2. Approved game catalogue

### Original mini-games — 12

1. **Memory Match** — flip symbols and find matching pairs.
2. **Word Scramble** — solve scrambled words using curated, age-appropriate word lists.
3. **Sequence Recall** — repeat progressively longer visual or audio-free sequences.
4. **Minesweeper** — reveal safe cells, mark suspected hazards, and solve the board.
5. **2048** — combine matching numbered tiles to reach the target.
6. **Hangman** — solve a word through limited letter guesses; use inclusive, non-graphic presentation.
7. **Mastermind** — deduce a hidden code from structured feedback.
8. **Reaction Challenge** — respond to a prompt with latency-tolerant timing; do not let Discord/network delay decide close matches.
9. **Trivia** — answer curated questions across selectable categories and difficulty.
10. **Pixel Clue** — identify an original pixel-art object or scene from clues.
11. **Snake** — guide a growing snake through a compact board with turn-based or carefully bounded input.
12. **Tetris-style Puzzle** — fit falling or queued shapes into a board; adapt input to Discord's interaction model rather than requiring frame-perfect controls.

### Multiplayer — 10

1. **Tic-tac-toe** — foundational turn-order and state-validation game.
2. **Connect Four** — shared board with strict turn ownership.
3. **Battleship** — private player boards and carefully separated hidden information.
4. **Chess** — legal move validation, clocks only where fair, and optional spectator mode.
5. **Checkers** — full move validation and clear draw rules.
6. **Uno-style Card Game** — original or properly licensed rules/assets; do not copy protected branding or artwork.
7. **Team Quiz** — team lobbies, answer windows, and transparent scoring.
8. **Werewolf / Social Deduction** — private role delivery, phase controls, moderation-safe text, and clear host tools.
9. **Co-op Puzzle** — players combine clues or actions toward a shared objective.
10. **Drawing & Guessing** — drawing prompts and guess turns with reporting, content limits, and moderation tools.

Every multiplayer game must define player limits, public/private lobby mode, invites, host permissions, turn/phase order, timeout rules, reconnect/rejoin behavior, spectator permissions, cancellation, and host-disconnect behavior.

### Adventure / RPG — 9

1. **Persistent Fantasy RPG** — an original world, persistent characters, quests, and story choices.
2. **Dungeon Crawler** — room-by-room exploration with turn-based encounters and discoverable routes.
3. **Monster Collector** — discover, catalogue, befriend, and develop original creatures.
4. **Creature Battler** — strategic turn-based encounters with readable, deterministic rules.
5. **Farming & Life Simulator** — grow crops, improve a home, meet NPCs, and take part in seasonal activities without punitive real-time pressure.
6. **Exploration & Discovery** — reveal maps, locations, lore, collectibles, and environmental puzzles.
7. **Text Adventure** — branching narrative with saved choices and replayable paths.
8. **Guild Adventure** — cooperative guild objectives, shared milestones, and opt-in parties.
9. **Crafting & Survival** — gather, craft, build, and explore in a forgiving turn-based world; no real-world dangerous activity instructions.

### Puzzle / Strategy — 10 catalogue entries, 8 unique titles

1. **Sudoku** — validated puzzles with difficulty tiers.
2. **Nonograms** — picture logic with verified solutions.
3. **Word Search** — themed word-finding boards.
4. **Crossword** — original clue packs with editorial review.
5. **Logic Grid** — deduction puzzles with verified unique or explicitly enumerated solutions.
6. **Minesweeper** — the same canonical game as Original Mini-games.
7. **Mastermind** — the same canonical game as Original Mini-games.
8. **Tower Defense** — strategic placement and wave resolution adapted to turn-based Discord controls.
9. **Chess Puzzles** — curated positions with validated solutions and explanations.
10. **Daily Logic Challenge** — optional rotating puzzle; missing a day never removes progress or causes a punitive reset.

Puzzle generation must validate solvability, solution count, clue quality, and difficulty before a puzzle is published.

## 3. Catalogue identifiers and cross-listing

Every title receives one stable game ID independent of its display name. Category membership is metadata, not a second implementation.

Suggested IDs:

| Game | Canonical ID |
| --- | --- |
| Minesweeper | `minesweeper` |
| Mastermind | `mastermind` |
| 2048 | `2048` |
| Tetris-style Puzzle | `tetris_puzzle` |
| Werewolf / Social Deduction | `social_deduction` |
| Uno-style Card Game | `card_party` |
| Persistent Fantasy RPG | `fantasy_rpg` |
| Farming & Life Simulator | `life_sim` |

IDs for the remaining games should follow the same stable snake_case convention. Display names may change without breaking saved records.

## 4. Standard game definition

Every game should declare:

- Stable ID, display name, description, category, tags, and version.
- Player limits, solo/multiplayer mode, and expected session length.
- Rules and help shown before play.
- State version and persistence requirements.
- Legal actions for every phase and actor.
- Public board/view model and private per-player view model.
- End conditions: completion, failure, draw, timeout, or cancellation.
- Result schema and eligible progression events.
- Pause, leave, save, resume, reconnect, and expiry behavior.
- Accessibility labels and a text fallback when visual rendering fails.
- Randomness requirements and whether a verifiable fairness transcript is needed.

## 5. Shared lifecycle

Games use the platform lifecycle: Created, Waiting for players, Active, Paused (only when supported), Completed, Aborted, and Expired.

Every action is validated against current state, membership, actor permissions, guild scope, and turn/phase ownership. Duplicate interactions must not award duplicate results or progression.

## 6. Visual and interaction standards

- Original full-colour pixel art is the default art direction.
- The visual library is a Next.js + React + TypeScript frontend.
- Discord-native messages, embeds, buttons, select menus, and modals remain the command bot's baseline interaction surface.
- A richer embedded experience may be delivered as a Discord Activity using the official Embedded App SDK; the same frontend should retain a browser fallback.
- Do not assume React components can be rendered inside an ordinary Discord embed.
- Give each game its own icon, board presentation, readable status, and accessible text alternative.
- Use symbols and labels as well as colour to communicate state.
- Keep image dimensions and payload sizes suitable for Discord.
- Record asset provenance and licences. Art must be original or properly licensed.

## 7. Fair randomness

For random outcomes where fairness needs to be independently checked, use the SHA-512 commit–reveal protocol described in [Fairness and Randomness](FAIRNESS.md).

SHA-512 is a cryptographic hash, not a random-number generator. The implementation must use operating-system-backed cryptographic entropy, publish a commitment before an outcome can be influenced, use an unbiased mapping from random bytes to game outcomes, and reveal enough transcript data to verify the result afterward. Never claim that “SHA-512 makes it fair” by itself.

No fairness protocol enables betting or wagering. Random games are for play and scores only; no stakes, cash-out, prize pools, or currency/item transfers based on chance.

## 8. Healthy replayability

Give players reasons to return through content variety, skill mastery, discovery, cooperation, and personal choice. Daily challenges are optional. Missing a day never destroys progress, removes earned rewards, or puts a player behind. Avoid artificial scarcity, punitive timers, endless pressure loops, and mechanics designed to make stopping difficult.

## 9. Game quality gate

A game enters the public catalogue only when:

- Rules and legal actions are documented.
- Tests cover wins, losses, draws, invalid moves, retries, and edge cases.
- Multiplayer tests cover turn order, membership, deadlines, reconnect, and cancellation.
- Persisted state is versioned and recoverable.
- Visual and text fallback states are accessible.
- Rewards are transparent, deterministic where possible, and non-wagering.
- Failure, cancellation, timeout, cleanup, and privacy paths are tested.


## 10. Current implementation snapshot

The Rust runtime currently implements these four catalogue entries:

| Stable ID | Catalogue entry | Current status |
| --- | --- | --- |
| tic-tac-toe | Multiplayer: Tic-tac-toe | Implemented: two-player guild lobby, legal moves, turn order, wins and draws. |
| connect-four | Multiplayer: Connect Four | Implemented: two-player guild lobby, column gravity, full-column checks, four-in-a-row and draws. |
| memory-match | Original mini-games: Memory Match | Implemented starter: solo eight-position fixed layout with four pairs. |
| word-scramble | Original mini-games: Word Scramble | Implemented starter: one original five-letter puzzle with three attempts. |

All four sessions are process-local and disappear on restart. They have no persistent profile, leaderboard or economy integration. Memory Match's starter layout is fixed; the randomised/seeded content pipeline described in FAIRNESS.md is not implemented. Word Scramble currently contains one puzzle rather than a curated word pack.

The remaining catalogue entries keep their existing proposed status until each has code, rule tests, command/interaction adapters, accessibility review and the appropriate persistence/privacy coverage.
