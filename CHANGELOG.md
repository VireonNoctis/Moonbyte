# Changelog

## 2026-11-03 — Wayfarer progression slice

- Add a daily quest board with three stable optional objectives selected from a fixed catalogue per player/day.
- Generate rotation seeds with PostgreSQL UUID randomness and derive selections using SHA-512 with domain separation and unbiased rejection sampling; expose only the fingerprint, never the seed.
- Connect game completions and first-time Atlas discoveries to quest progress and cumulative active-day history inside their source transactions.
- Add cosmetic companion milestones that never reset and do not grant gameplay advantages.
- Add private, mutually accepted rivalries that count only later completed two-player sessions.
- Credit fixed quest rewards through immutable, idempotent Moonbits ledger entries; no wagering, random reward pools, purchases or cash-out are introduced.
- Extend the PostgreSQL schema, privacy inventory, version 7 player export, deletion receipt, Tavern destinations, command help, tests and docs.
- Add a responsive React catalogue view for the real Wayfarer objectives without fabricating player progress.

## Unreleased

- Add a first-use agreement gate for versioned Terms of Service and Privacy Policy documents, with legal and privacy commands available before acceptance.
- Add the versioned user-consent record to data inventory, version 6 export and confirmed account deletion.
- Add the Terms of Service document and serve both full policies as private slash-command attachments.
- Add prefix-only shortcuts for selected gameplay and navigation commands; keep help, Terms, consent and privacy commands full-length.
- Synchronize guild command-setting caches with PostgreSQL LISTEN/NOTIFY and clean guild-owned sessions/settings after actual bot removal.

- Add PostgreSQL-backed per-server command prefixes with a safe process-default fallback.
- Add a Manage Server-only prefix command for viewing, setting, resetting and routing bot commands to one server channel.
- Apply command-channel restrictions consistently to prefix and slash commands while keeping configuration available from any channel.
- Cache guild routing settings so ordinary messages do not cause PostgreSQL lookups.
- Correct a literal escaped newline in the README documentation index.

- Add the five-entry achievement gallery with server-authoritative milestone checks and idempotent unlock rows.
- Add the initial five-location Atlas with stable location IDs, reciprocal route edges, personal discovery/visit records and direct-neighbour travel.
- Add PostgreSQL migration 0006 for player world state and personal Atlas discoveries, both owned by player profiles through foreign keys.
- Add private `.atlas` and `.travel` prefix/slash commands and expose The Atlas as a live Tavern Wayfinder destination.
- Extend player inventory, version 5 JSON export and confirmed deletion to include current Atlas location and discovery history.
- Add route catalogue tests for unique IDs, reciprocal connectivity, full reachability and stable lookup, plus PostgreSQL travel/revisit/privacy lifecycle coverage.

- Add PostgreSQL-backed player profiles with validated display names, short bios, private-by-default visibility and saved preferences.
- Add private profile commands, a current-schema data inventory, on-demand JSON export and explicit two-step profile deletion.
- Add atomic player deletion that removes the profile and every game-session snapshot containing that player ID.
- Add a PostgreSQL migration for player profiles and extend the ordered migration runner.
- Add PostgreSQL integration coverage for profile creation, editing, preferences, export and deletion.
- Add migration 0003 for completed-game result records and per-player/per-game aggregate statistics.
- Persist results and update every participant's counters atomically with the completed session snapshot, using the session ID to prevent duplicate awards.
- Include result metadata and aggregate statistics in privacy inventory/export/deletion.
- Add the initial Tavern home and typed Wayfinder, linking only to implemented games, player profile, game records and wallet destinations.
- Add a central custom-emoji registry with five upload-name slots, validated numeric IDs and Unicode fallbacks.
- Add versioned PostgreSQL migration 0004 for player wallets and immutable wallet ledger entries.
- Award a fixed 10 Moonbits on each player's first completion of each game type, once per player/game type, in the same transaction as the completed game result and statistics.
- Add a private `.wallet` command and disclose the reward rule in the game library.
- Extend data inventory, version 3 JSON export and transactional deletion to wallet and ledger records.
- Add integration coverage for repeated-completion reward protection and wallet export/deletion.

- Switch the persistence target from the superseded SQLite decision to PostgreSQL through SQLx (PostgreSQL driver only).
- Add a versioned PostgreSQL migration runner, durable JSONB game-session snapshots, transactional row-locked mutations, restart recovery, 24-hour inactivity expiry and scheduled cleanup.
- Add PostgreSQL-backed runtime integration tests and a CI PostgreSQL service.

- Add transport-independent Rust rules for Tic-tac-toe, Connect Four, Memory Match and Word Scramble.
- Add an in-process session runtime with guild-scoped lobbies, participant-only reads, turn validation, action validation, completion and leave controls.
- Add prefix and slash commands for games, play, join, action, session and leave.
- Add unit coverage for game outcomes, invalid actions, turn order, participant boundaries and guild isolation.
- Document the process-local persistence boundary and remaining runtime work.

