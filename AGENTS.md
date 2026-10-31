# Moonbyte Engineering Instructions

These instructions apply across this repository. More specific instructions in a nested AGENTS.md may refine them for that directory.

## Product boundary

- Moonbyte is a separate Discord gaming bot built from selected game concepts and systems in Lunar.
- Do not make Moonbyte depend at runtime on Lunar's bot, website, account linking, or database.
- Audit source code, dependencies, data models, and licences before porting anything. Prefer idiomatic Rust game rules over copying entire Discord-specific cogs.
- The default prefix is a period (.). Support slash commands alongside prefix commands where practical.
- The visual identity is original full-colour pixel art. Do not make it monochrome or copy protected console branding, sprites, characters, or assets.
- Build a deep and replayable product, but never optimise for compulsion, inability to stop, or compulsive spending. Respectful engagement is a product requirement.

## Accepted technology direction

- Primary application language: Rust.
- Async runtime: Tokio.
- Discord API: Serenity.
- Command framework: Poise.
- Visual library: Next.js + React + TypeScript.
- Discord Activity integration: official Discord Embedded App SDK.
- Initial persistent database: SQLite, accessed through SQLx configured for SQLite only.
- PostgreSQL and ScyllaDB are explicitly excluded.
- Do not introduce Redis, message brokers, separate services, or distributed storage without measured requirements and a new accepted decision.
- Keep compatible stable dependency versions and commit Cargo.lock when the application workspace exists.

## Before changing code

1. Read this file and the relevant product, privacy, architecture, game, and roadmap documents.
2. Inspect the implementation, project structure, tests, and recent changes.
3. Identify the actual requirement or cause before editing.
4. Check whether an existing module, helper, dependency, or pattern already solves it.
5. Think of useful adjacent improvements, but record unrelated ideas as follow-up work. Do not expand scope silently.

## Implementation standards

- Think before coding. Simplicity first. Make the smallest complete, surgical change.
- Prefer explicit types and clear ownership boundaries over clever abstractions.
- Do not reorganise working code or refactor unrelated areas.
- Keep Discord-specific types at the adapter boundary; game rules must be testable without a live Discord connection.
- Validate every player action against session state, membership, permissions, phase, and turn ownership.
- Do not hold a global lock or database transaction across Discord HTTP calls or slow awaits.
- Use bounded queues, timeouts, cancellation, and backpressure deliberately.
- Do not block Tokio worker threads with CPU-heavy rendering, synchronous I/O, or blocking database calls.
- Do not use per-frame polling. Discord is an event-driven interaction platform, not a 60-FPS game client.
- Version persisted game state and make migrations explicit.
- Handle Discord rate limits, expired interactions, deleted messages, and partial failures without losing durable game state.
- Keep secrets, personal data, and production data out of source control and logs.
- Do not add fake data, placeholder implementations, silent fallbacks, or success messages for unfinished behavior.
- Do not build betting, wagering, staking, paid random rolls, loot-box monetisation, cash-out, or pay-to-win progression.
- Use the SHA-512 commit–reveal protocol in docs/FAIRNESS.md for outcomes that require public verification. SHA-512 is a hash, not an entropy source; use OS-backed cryptographic randomness and unbiased result mapping.
- Keep the Next.js/React frontend presentation-only. Rust owns authoritative game rules, session state, economy ledger, trade execution, fairness transcripts, permissions, and persistence.
- Never expose bot tokens, database credentials, private seeds before reveal, or privileged backend secrets in frontend bundles.
- Use Discord's Embedded App SDK only for Activity integration; ordinary Discord embeds do not host React components.

## Healthy engagement requirements

- Make games compelling through variety, mastery, discovery, cooperation, content quality, and player choice.
- Do not use punitive streak loss, manipulative FOMO, fake scarcity, nagging notifications, infinite reward loops, or mechanics designed to make play difficult to stop.
- Daily challenges are optional and must not punish a player for missing a day.
- Provide clear exits and pause/leave controls. Never obstruct a player from ending a session.
- Respect opt-outs, notification settings, quiet hours, and server-level limits.
- Use transparent, predictable rewards. Do not make progression depend on compulsive repetition.

## Privacy requirements

- Treat privacy as a product requirement, not a marketing badge.
- Collect the minimum fields required for a feature and document each purpose.
- Do not claim end-to-end encryption, encryption at rest, anonymity, or zero logging unless the implementation and deployment prove it.
- Do not persist raw command/message content for analytics.
- Do not log tokens, authorization headers, private payloads, or unnecessary personal data.
- Add an accessible privacy command with overview, data inventory, export, and deletion flows.
- Deletion must require clear confirmation, execute against all relevant user-owned records, and report its actual outcome.
- Test guild isolation, export completeness, deletion completeness, retention cleanup, and log redaction.
- Keep this policy synchronized with the actual schema, logging, deployment, and subprocessors. Implementation must not silently drift from docs/PRIVACY.md.

## Testing and verification

- Unit-test deterministic game rules independently of Discord.
- Test state machines, permissions, turn order, duplicate actions, deadlines, cancellation, and recovery.
- Inject clocks and random-number sources in tests.
- Test SQLite migrations and repository behavior using temporary databases.
- Test privacy export, profile deletion, guild isolation, and log redaction.
- Add adapter tests for command parsing and authorization where practical.
- Run rustfmt, Clippy, tests, and the full build once the Rust workspace exists.
- Review the final diff for secrets, generated artifacts, and unrelated edits.
- State exactly which checks ran. Never report an unexecuted check as passing.

## Agent coordination

When genuine parallel agents are available, use them for independent research, review, test design, or isolated modules. Give each agent a distinct deliverable and file boundary. Never let multiple agents edit the same files concurrently or invent work to increase agent count. One owner integrates and verifies the result.

## Documentation

Update documentation when setup, configuration, public behavior, architecture, privacy, or operations change. Record cross-cutting decisions in docs/DECISIONS/. Clearly distinguish implemented behavior from planned behavior.

## Git hygiene

- Keep commits focused and messages descriptive.
- Do not rewrite shared history or force-push unless explicitly requested.
- Do not commit generated output, local IDE state, credentials, or environment-specific files.
