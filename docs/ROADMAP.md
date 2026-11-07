# Moonbyte Roadmap

> Document version 1.0.0. All completed checkboxes below mean implemented in source and CI-tested where noted; they do not mean a Discord bot has been registered, invited or deployed.

## Source foundations implemented to date

- [x] Rust/Tokio/Serenity/Poise application, validated environment configuration and PostgreSQL/SQLx persistence.
- [x] Ordered migrations, durable JSONB game sessions, row-locked mutations, restart recovery and 24-hour inactive-session retention.
- [x] Shared prefix/slash command handlers, configurable guild prefixes and optional command-channel restriction.
- [x] Reusable Discord title-panel presentation with optional slash `title` fields and prefix title text across Craft, Tavern, Games, player overviews, inventory/collection, Atlas/Chronicle, Quests and active-session views; Expedition catalogue/status views reuse the standard panel and board embeds retain canonical game titles.
- [x] Slash autocomplete dropdowns for recipes, games, Atlas locations, finite Quest/Rivalry/Expedition actions and profile visibility; recipe labels show their output and ingredients while values remain canonical IDs.
- [x] Conservative typo suggestions for recipes, game IDs, Atlas destinations and finite action verbs; suggested values are never auto-executed.
- [x] Graceful Ctrl-C and Unix SIGTERM shutdown with minimised structured tracing.
- [x] Four playable games and guild-scoped multiplayer lobbies.
- [x] Numbered Discord board controls for Tic-tac-toe, Connect Four and Memory Match, routed through the authoritative GameRuntime.
- [x] Player profiles/preferences, versioned policy consent, private data inventory, version 12 export and confirmed deletion; export uses a repeatable-read snapshot and deletion-race coverage is deterministic.
- [x] Completed results, aggregate statistics, five achievements and the earned-only Moonbits ledger.
- [x] Sixteen Tavern destinations, including the Satchel, Postcard Wall and Workshop.
- [x] Five-location Atlas, direct-neighbour travel, private Chronicle pages and five account-bound postcards.
- [x] Two fixed materials earned through authored Atlas discoveries, with migration backfill.
- [x] One deterministic crafting recipe with atomic input consumption, account-bound output provenance and repeat-craft protection.
- [x] Optional Wayfarer objectives, cumulative journey days, cosmetic companion milestones and mutually accepted private rivalries; seven stable daily objectives now include first-time crafting and completed Expeditions, committed atomically with their source actions.
- [x] Derived per-game and overall Mastery ranks calculated from existing statistics; added `.mastery [game]` and the private Player Card projection without adding a data table.
- [x] Dedicated `.streak` / `.streak milestones` views backed by the canonical companion milestone catalogue, and a private Chronicle Rivalry Log for mutually accepted head-to-head records.
- [x] Lantern Road Expedition v1 with solo/group state, explicit opt-in, persistent checkpoints, pause/resume/leave and privacy coverage.
- [x] Manual PostgreSQL backup and guarded empty-database restore scripts, plus a database readiness check in the ping command.
- [x] Version 1.0.0 initial privacy policy and product blueprint labels.
- [x] Mosslight Resonance solo Puzzle Vault: three authored chambers, explicit private entry, PostgreSQL save stones, resume/pause/leave, fixed one-time keepsake, Chronicle/Wayfarer hand-off, export/deletion and concurrency/replay coverage. CI passed 96 PostgreSQL-backed tests, rustfmt, the PostgreSQL backup/restore round-trip and Clippy; private development-guild interaction verification remains a deployment gate.
- [x] Private Player Card v1: private cross-system projection from the owning game, achievement, Wayfarer, Rivalry, Chronicle, inventory and Vault stores, with command shortcuts. The read-only panel renderer and source build are CI-tested; public/guild profile projections remain deferred.
- [x] First opt-in Guild Project: a server-scoped Lantern Road Survey with manager-authorised setup, explicit join/leave, private aggregate status, idempotent shared-Expedition contributions, anonymous progress, membership export/deletion and actual-uninstall cleanup.
- [x] Seasonal Almanac v1: four published annual UTC chapter windows and a private archive projection over Atlas, Workshop, Vault and completed Expedition evidence; all source routes remain evergreen and no duplicate seasonal reward state is stored.

## Remaining before a first Discord bot deployment

1. **Register and configure the Discord application**
   - Create the Discord application and bot identity; provision its token through a secret manager.
   - Enable only required intents and configure a least-privilege invite for a private development guild.
   - Verify global slash-command registration, prefix behavior, buttons, permissions, guild settings and bot uninstall cleanup in Discord.

2. **Prepare production operations**
   - Provision PostgreSQL with TLS, restricted network access, a dedicated role and documented ownership.
   - Schedule encrypted off-host backups, define retention and recovery objectives, and alert on backup failures.
   - Rehearse restoration into a new empty database; document the measured recovery point and duration.
   - Configure process supervision, log retention, uptime monitoring and an incident contact.

3. **Finish interaction and reliability verification**
   - Add development-guild smoke tests for stale buttons, simultaneous clicks, deleted messages and terminal boards.
   - Add durable interaction receipts only if test/deployment evidence shows the current row-locked action-version guard is insufficient.
   - Review component accessibility and rate-limit/partial-delivery behavior.

4. **Expand the connected game world**
   - Add more authored materials and deterministic recipes only after the first Workshop recipe is fully verified.
   - Add further authored Puzzle Vault blueprints only after Mosslight Resonance has passed its privacy, concurrency, replay and recovery gates. The first implementation is tracked against [ADR 0013](DECISIONS/0013-first-puzzle-vault-vertical-slice.md).
   - Add more Expedition routes, story quests and reviewed original word/puzzle packs.
   - Expand the first Guild Project into reviewed shared collections and milestone hand-offs only after moderation, visibility, reward settlement and abuse boundaries are tested.

## Later proposals

- Fixed-price Bazaar listings and work orders after ownership, moderation, duplicate protection and atomic settlement are complete.
- Festivals with authored seasonal quest/art/story content, additional companion cosmetics and homesteads after their owning systems exist. The private Player Card remains a read-only projection; public and guild-facing cards remain deferred.
- Public player directory and leaderboards only with explicit visibility controls and privacy-complete projections.
- Broader social surfaces only after guild isolation, consent and deletion boundaries are complete. The first Guild Project does not expose member lists or public player profiles.

## Release gates

A feature is complete only when its owning Rust module controls state transitions; migrations are ordered; actions are validated and idempotent; concurrency and recovery are tested; Discord UX has accessible labels and text fallback; privacy inventory/export/deletion is complete; and all canonical docs agree with code.

A source implementation is not a release. The project becomes launch-ready only after the Discord application, deployment secrets, PostgreSQL operations, recovery process and live Discord acceptance checks are configured.
