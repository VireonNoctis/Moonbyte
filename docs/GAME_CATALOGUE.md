# Moonbyte Game Catalogue

- Status: Candidate catalogue; no listed game is implemented by this specification alone.
- The catalogue should grow through polished releases, not a target number of games.

## 1. Required categories

### Existing Lunar games

Migration candidates from [Lunar](https://github.com/VireonNoctis/Lunar), subject to source, dependency, and licence review.

| Candidate | Likely Moonbyte role | Migration notes |
| --- | --- | --- |
| Akinator-style guessing | Guided yes/no guessing session. | Audit external service/API dependency and terms. |
| Coin flip | Short solo or party game. | Non-wagering only; test outcome generation and session handling. |
| Dice | Quick randomised mini-game. | Define supported formats and validate inputs. No stakes or betting. |
| 8-ball | Casual party interaction. | Keep age-appropriate and clearly for fun. |
| Rating interaction | Lightweight social prompt. | Review abuse, targeting, and privacy risks before porting. |
| Card collection | Collection and progression. | Audit data model, odds, rewards, and dependencies. No paid pulls or wagering. |

### Original mini-games

- **Memory Match:** flip a board of symbols and find pairs.
- **Word Scramble:** solve a short word puzzle with validated word lists.
- **Sequence Recall:** remember and repeat a growing sequence.
- **Pattern Finder:** identify a rule in a visual or numeric sequence.
- **Quick Quiz:** answer a small set of themed questions.
- **Pixel Clue:** identify an original object or scene from a gradually revealed pixel-art clue.

Keep rounds short, rules visible, and scoring deterministic. Do not use reaction windows so tight that normal Discord latency decides the winner.

### Multiplayer

Candidate order:
1. **Tic-tac-toe:** reference game for validation and turn order.
2. **Connect Four:** board state and win detection.
3. **Team Quiz:** lobby, answer windows, scoring, and tie handling.
4. **Co-op Puzzle:** players contribute to a shared objective.
5. **Turn-based strategy:** add after session reliability is proven.

Every multiplayer game must specify player limits, lobby rules, host permissions, turn order, timeout behavior, leave/rejoin, spectator rules, and host-disconnect behavior.

### Adventure / RPG

Longer-running experiences can include an original world map, explorable locations, quests, branching choices, NPC dialogue, inventory, collectible items, crafting, deterministic upgrades, party progression, cooperative objectives, and turn-based encounters.

The first RPG should be a compact vertical slice: one area, a few NPCs, a short quest chain, a small inventory, and a clear save/resume loop. Do not begin with a giant open world.

### Puzzle / strategy

Candidate experiences:
- Sudoku and compact logic grids.
- Nonograms and picture logic.
- Word and number puzzles.
- Mastermind-style deduction.
- Chess and checkers where a robust rules implementation is available.
- Daily and weekly puzzle variants with clear reset rules.

Puzzle generation must validate solvability and solution quality before publication.

## 2. Standard game definition

Every game should declare:
- Stable identifier that does not change with display name.
- Name, description, category, and tags.
- Player limits and session mode.
- Rules and help shown before play.
- State version and persistence requirements.
- Legal actions for each game phase.
- Public board and private per-player view models.
- End conditions: completion, failure, draw, timeout, or cancellation.
- Result schema and eligible progression events.
- Expected session length and supported pause/resume behavior.

## 3. Shared lifecycle

Games use the platform lifecycle: Created, Waiting for players, Active, Paused (only when supported), Completed, Aborted, and Expired.

Every action must be validated against state, membership, actor permissions, and turn ownership. Duplicate interactions must not award duplicate results or rewards.

## 4. Visual standards

- Original full-colour pixel art is the default visual language.
- Each game has its own icon and consistent board presentation.
- Reuse sprites, palette tokens, layout components, and render caches.
- Use text labels, shape differences, or symbols as well as colour to communicate status.
- Keep board images suitable for Discord.
- Provide text rules and results if images fail to load.
- Record asset provenance and licences.

## 5. Healthy replayability

Give players reasons to return through new content, learning, mastery, social play, and personal choice. Daily challenges are optional. Missing a day must not break a streak, remove rewards, or place a player behind. Avoid artificial scarcity, endless pressure loops, or punitive timers.

## 6. Game quality gate

A game enters the public catalogue only when rules and legal actions are documented; tests cover wins, losses, draws, invalid moves, and edge cases; multiplayer tests cover turn order, membership, deadlines, retries, and reconnect; persisted state is versioned; UI states are accessible; rewards are non-wagering; and failure, cancellation, and cleanup paths are tested.
