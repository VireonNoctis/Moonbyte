# Moonbyte Roadmap

> The [Final Vision](MOONBYTE_FINAL_VISION.md) governs product direction. The [Implementation Map](IMPLEMENTATION.md) is the source of truth for what is live. This document tracks delivery work, not speculative release dates.

## Complete: foundation and first world slice

- [x] Rust/Tokio/Serenity/Poise runtime, validated environment configuration and PostgreSQL/SQLx.
- [x] Ordered schema migrations, durable JSONB game sessions, row-locked mutations and restart recovery.
- [x] Shared prefix/slash commands, configurable guild prefix and optional command-channel restriction.
- [x] Graceful Ctrl-C and Unix SIGTERM shutdown.
- [x] Four playable games and guild-scoped multiplayer lobbies.
- [x] Player profiles/preferences, versioned policy consent, private data inventory, version 9 export and confirmed deletion.
- [x] Completed results, aggregate statistics, five achievements and the earned-only Moonbits ledger.
- [x] Twelve live Tavern destinations.
- [x] Five-location Atlas, reciprocal routes, direct-neighbour travel and private Chronicle discovery pages.
- [x] Optional Wayfarer objectives, cumulative journey days, cosmetic companion milestones and mutually accepted private rivalries.
- [x] Lantern Road Expedition v1 with solo/group state, opt-in participation, persistent checkpoints, pause/resume/leave, recovery and privacy coverage.
- [x] PostgreSQL integration tests, formatting and Clippy in CI.

## Next slice: reliable Discord play

1. **Discord-native game controls**
   - Add button controls for Tic-tac-toe and Connect Four, then Memory Match.
   - Route component input through the same GameRuntime and validation as command actions.
   - Bind each control to the session/message, enforce guild and participant checks, reject stale/duplicate controls, and disable controls after terminal states.
   - Keep command actions as an accessible fallback.

3. **Interaction reliability**
   - Acknowledge component interactions within Discord's response window.
   - Persist/derive idempotency keys so duplicate delivery cannot apply a move twice.
   - Handle deleted messages, expired interactions, restart recovery and response-edit failure without rolling back committed state.
   - Test concurrent moves and out-of-order controls against PostgreSQL.

4. **Production recovery**
   - Document backup retention and restore objectives.
   - Automate backups and verify restore in an isolated environment.
   - Add database health/latency diagnostics and operational runbooks.
   - Do not claim multi-instance support before load, failover and duplicate-delivery tests pass.

## Following slices

- Expand the original game catalogue in small, polished groups; add reviewed puzzle and word packs.
- Add inventory and provenance before implementing crafting.
- Add one fixed-rule Puzzle Vault with disclosed deterministic rewards; no paid/random loot.
- Add further authored Expedition routes and story quests after the current route's operational acceptance review.
- Add guild projects and collaborative collections with explicit roles, opt-in sharing and deletion boundaries.
- Consider fixed-price Bazaar listings only after inventory ownership, escrow, atomic settlement, duplicate protection and moderation are proven. No betting, wagering or gambling mechanics.
- Add mastery, seasonal Almanac and richer companion cosmetics after their owning systems are implemented.

## Release gates

A feature is complete only when its owning Rust module controls state transitions; migrations are reversible/ordered; actions are validated and idempotent; concurrency and recovery are tested; Discord UX has text fallback and accessible labels; privacy inventory/export/deletion is complete; and docs agree with code.
