# Moonbyte Engineering Instructions

These instructions apply across this repository. More-specific nested instructions may refine them.

## Product boundary

- Moonbyte is a standalone Discord gaming bot. Its identity, source, rules, storage, commands and release lifecycle belong to this repository.
- Discord is the required player interface: commands, messages, embeds, buttons, select menus and modals.
- Do not add a website, React/Next.js frontend, browser game client or Discord Activity. They are not part of the product requirement.
- The visual identity is original full-colour pixel art. Do not copy protected console branding, sprites, characters or third-party assets.
- Build a deep, replayable world through quality, variety, mastery, discovery and cooperation—not compulsive use.

## Accepted technology

- Primary language: Rust.
- Async runtime: Tokio.
- Discord API: Serenity.
- Command framework: Poise.
- Database: PostgreSQL through SQLx with the PostgreSQL driver only.
- SQLite and ScyllaDB are excluded. Do not add Redis, brokers, separate services or distributed storage without measured need and a new accepted decision.
- The pinned toolchain in `rust-toolchain.toml` is authoritative for local development. CI must report the exact compiler version it uses. Change the pin deliberately, not speculatively.
- Keep `Cargo.lock` committed and use `--locked` for reproducible checks.

## Before changing code

1. Read this file and the relevant canonical documents: Final Vision, Implementation Map, Architecture, Privacy, Game Catalogue and Roadmap.
2. Inspect the current implementation, migrations, tests and recent branch changes.
3. Identify the actual requirement and existing ownership boundary before editing.
4. Reuse an existing module, helper, dependency or pattern when it solves the problem.
5. Think about adjacent improvements, but record unrelated ideas as follow-up work instead of silently expanding scope.

## Engineering standards

- Think before coding. Simplicity first. Make the smallest complete, surgical change.
- Keep domain game rules independent of Serenity, Poise and SQLx.
- Keep Discord-specific types at the adapter boundary.
- Validate every action against session state, participant membership, permissions, phase and turn ownership.
- Do not hold a database transaction or shared lock across Discord HTTP requests or slow awaits.
- Use bounded queues, timeouts, cancellation and backpressure deliberately.
- Do not block Tokio workers with synchronous I/O or CPU-heavy work.
- Discord is event-driven; do not build frame polling or 60-FPS assumptions into game design.
- Version persisted state and make migrations explicit and ordered.
- Handle rate limits, expired interactions, deleted messages and partial delivery failures without losing committed state.
- Never add fake data, placeholder implementations, silent fallbacks or success messages for unfinished behavior.
- Keep secrets, private player data and production values out of source control and logs.

## Fairness, economy and healthy engagement

- No betting, gambling, wagering, staking, pooled stakes, paid random rolls, paid loot boxes, cash-out, chance-based tradable rewards or pay-to-win progression.
- Moonbits remain earned-only, non-redeemable and ledger-backed.
- SHA-512 is a hash, not an entropy source. Use OS-backed cryptographic randomness and unbiased mapping where randomness is required.
- Daily challenges are optional. Missing a day never removes progress or rewards.
- Do not use punitive streaks, fake scarcity, FOMO, nagging, forced presence or mechanics designed to make stopping difficult.
- Provide clear pause, leave, decline, mute and account-deletion paths.

## Privacy requirements

- Collect only fields required by an enabled feature and document purpose, visibility, retention, export and deletion.
- Keep personal, party, guild and server scopes separate.
- Do not retain raw command/message content for analytics.
- Do not log tokens, authorization headers, private payloads, player messages or unrevealed fairness seeds.
- Do not claim end-to-end encryption, encryption at rest, anonymity or zero logging without deployment evidence.
- Extend privacy inventory, export, deletion and tests whenever a new player-owned table is added.
- Keep [Privacy Policy](docs/PRIVACY.md) aligned with the schema, logs, deployment and providers.

## Testing and verification

- Unit-test game rules without Discord.
- Test state transitions, permissions, turn order, duplicate actions, deadlines, cancellation and recovery.
- Inject clocks and random sources into deterministic tests.
- Test PostgreSQL migrations, transaction rollback, concurrent mutations, expiry and recovery against an isolated database.
- Test privacy export/deletion, guild isolation and log redaction.
- Run repository validation, rustfmt, workspace tests, Clippy and the full build as applicable.
- State exactly which checks ran; never report an unexecuted check as passing.

## Documentation

Use a single canonical document for each concern. Update the Implementation Map for live behavior, the Roadmap for status/next work, Architecture for technical contracts, and Privacy for data practices. Keep the ten World Loom volumes for proposals; do not copy their idea catalogues into feature documents.

## Agent coordination

Use genuine parallel agents only for independent research, review, test design or isolated modules. Give each a distinct deliverable and file boundary. One owner integrates and verifies the result.

## Git hygiene

Keep commits focused and descriptive. Do not rewrite shared history or force-push. Do not commit generated output, local IDE state, credentials or environment-specific files.
