# Moonbyte Roadmap

> The [Final Vision](MOONBYTE_FINAL_VISION.md) governs product direction. The [Implementation Map](IMPLEMENTATION.md) is the source of truth for what is live. This document tracks delivery work, not speculative release dates.

## Complete: Discord game foundation and first connected world slice

- [x] Rust/Tokio/Serenity/Poise runtime, validated configuration and PostgreSQL/SQLx persistence.
- [x] Ordered migrations, durable JSONB game sessions, row-locked mutations and restart recovery.
- [x] Shared prefix/slash commands, configurable guild prefixes and optional command-channel restriction.
- [x] Graceful Ctrl-C and Unix SIGTERM shutdown with minimised structured tracing.
- [x] Four playable games and guild-scoped multiplayer lobbies.
- [x] Numbered Discord board controls for Tic-tac-toe, Connect Four and Memory Match, routed through the authoritative GameRuntime.
- [x] Button action-version guard checked inside the session row lock, stale-board refresh, terminal control disabling and text fallback.
- [x] Player profiles/preferences, versioned policy consent, private data inventory, version 10 export and confirmed deletion.
- [x] Completed results, aggregate statistics, five achievements and the earned-only Moonbits ledger.
- [x] Fourteen live Tavern destinations.
- [x] Five-location Atlas, reciprocal routes, direct-neighbour travel and private Chronicle discovery pages.
- [x] Optional Wayfarer objectives, cumulative journey days, cosmetic companion milestones and mutually accepted private rivalries.
- [x] Lantern Road Expedition v1 with solo/group state, opt-in participation, persistent checkpoints, pause/resume/leave, recovery and privacy coverage.
- [x] Provenance-backed account-bound Atlas postcard inventory, existing-player backfill, Satchel and Postcard Wall commands, export/deletion coverage.

## Next engineering slices

1. **Interaction and operational reliability**
   - Add durable interaction receipts only if test/deployment evidence shows the action-version guard is insufficient.
   - Add development-guild smoke tests for stale controls, simultaneous clicks, deleted messages and terminal boards.
   - Automate PostgreSQL backup/restore, define retention and recovery objectives, and rehearse restore in isolation.
   - Add operational health/latency diagnostics and alerting; retain the single-process support contract until concurrency/failover tests justify otherwise.

2. **Known-input crafting**
   - Introduce a small authored material catalogue and a fixed recipe book.
   - Gather only from verified world/game outcomes; never grant materials from random drops.
   - Consume owned inputs and create outputs with immutable provenance in one transaction.
   - Add inventory reservations only when a real multi-step operation needs them.
   - Keep player trading and Bazaar disabled until ownership, moderation, duplicate protection and atomic settlement are complete.

3. **The first Puzzle Vault**
   - Deliver one authored, deterministic challenge with saved checkpoints, clear rules and disclosed fixed rewards.
   - Connect Vault completion to Atlas/Chronicle, Wayfarer and achievement layers through idempotent source events.
   - No paid entry, wagers, random loot or punitive expiry.

4. **World expansion**
   - Add authored Expedition routes and story quests in small complete vertical slices.
   - Add opt-in guild projects and shared collections only after member roles, visibility, moderation and deletion boundaries are tested.
   - Expand the original game library with reviewed word/puzzle packs and polished small games.

## Later proposals

- Fixed-price Bazaar listings and work orders after inventory ownership and atomic settlement are established.
- Mastery, seasonal Almanac, festivals, companion cosmetics and homesteads after their owning systems exist.
- Public player directory and leaderboards only with explicit visibility controls and privacy-complete projections.
- Broader social surfaces only after guild isolation, consent and moderation are complete.

## Release gates

A feature is complete only when its owning Rust module controls state transitions; migrations are ordered; actions are validated and idempotent; concurrency and recovery are tested; Discord UX has accessible labels and text fallback; privacy inventory/export/deletion is complete; and all canonical docs agree with code.
