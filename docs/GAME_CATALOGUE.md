# Moonbyte Game Catalogue

> An approved design catalogue, not a claim that every listed game is already playable. The current live games are shown at the end.

## Original mini-games — 12

1. **Memory Match** — reveal symbols and find matching pairs.
2. **Word Scramble** — solve curated, age-appropriate scrambled words.
3. **Sequence Recall** — repeat increasingly long visual sequences.
4. **Minesweeper** — reveal safe cells and solve the board.
5. **2048** — combine matching numbered tiles.
6. **Hangman** — solve a word through bounded letter guesses with non-graphic presentation.
7. **Mastermind** — deduce a hidden code from structured feedback.
8. **Reaction Challenge** — respond to prompts using latency-tolerant timing.
9. **Trivia** — answer curated questions by category and difficulty.
10. **Pixel Clue** — identify an original pixel-art object or scene from clues.
11. **Snake** — guide a growing snake through a compact turn-based board.
12. **Falling-Block Puzzle** — fit queued shapes using Discord-friendly turns.

## Multiplayer — 10

1. **Tic-tac-toe** — turn order and state validation.
2. **Connect Four** — shared board and strict turn ownership.
3. **Battleship** — private boards and separated hidden information.
4. **Chess** — legal move validation and optional spectator view.
5. **Checkers** — complete move validation and explicit draw rules.
6. **Original Shedding Card Game** — an original or properly licensed ruleset and assets.
7. **Team Quiz** — team lobbies, answer windows and transparent scoring.
8. **Social Deduction** — private role delivery, phase controls and moderation-safe text.
9. **Co-op Puzzle** — combine clues or actions toward a shared objective.
10. **Drawing and Guessing** — moderated prompts, bounded turns and report controls.

## Adventure and RPG — 9

1. **Persistent Fantasy RPG** — original characters, quests and story choices.
2. **Dungeon Crawler** — room-by-room exploration with turn-based encounters.
3. **Monster Collector** — discover and catalogue original creatures.
4. **Creature Battler** — readable, turn-based encounters with documented rules.
5. **Farming and Life Simulator** — grow crops, improve a home and meet NPCs without punitive timers.
6. **Exploration and Discovery** — maps, lore, collectibles and environmental puzzles.
7. **Text Adventure** — branching narrative with saved choices.
8. **Guild Adventure** — cooperative server objectives and opt-in parties.
9. **Crafting and Survival** — a forgiving turn-based fictional world with safe, age-appropriate content.

## Puzzle and strategy — 10 category entries, 8 unique titles

1. **Sudoku** — validated puzzles with difficulty tiers.
2. **Nonograms** — picture logic with verified solutions.
3. **Word Search** — themed word-finding boards.
4. **Crossword** — original clue packs with editorial review.
5. **Logic Grid** — deduction puzzles with verified solutions.
6. **Minesweeper** — the same canonical game as the mini-game entry.
7. **Mastermind** — the same canonical game as the mini-game entry.
8. **Tower Defense** — strategic placement and turn-based wave resolution.
9. **Chess Puzzles** — curated positions with validated solutions.
10. **Daily Logic Challenge** — optional puzzles; missed days never remove progress.

## Shared game contract

Every game has a stable ID independent of its display name and declares:

- Category, description, tags, player limits and expected session length.
- Rules and legal actions for each phase/actor.
- Versioned state and persistence requirements.
- Public board and private per-player projections where needed.
- Win, loss, draw, completion, timeout and cancellation rules.
- Pause, leave, resume, reconnect and expiry behavior.
- Accessibility labels and a text fallback.
- Randomness/fairness requirements and any eligible fixed progression event.

A game is not released merely because a command exists. Its domain rules, concurrency, persistence, recovery, interaction authorization, privacy coverage and accessibility must pass the release gate.

## Current implementation

| Stable ID | Players | Status |
|---|---:|---|
| `tic-tac-toe` | 2 | Implemented: guild lobby, legal moves, turn order, wins and draws. |
| `connect-four` | 2 | Implemented: guild lobby, legal drops, turn order, wins and draws. |
| `memory-match` | 1 | Implemented starter: eight fixed positions and four pairs. |
| `word-scramble` | 1 | Implemented starter: one five-letter puzzle and bounded attempts. |

The remaining catalogue entries are approved concepts only. They require authored content, rule tests, Discord interaction design, persistence/recovery, accessibility review and privacy coverage before being advertised as playable.
