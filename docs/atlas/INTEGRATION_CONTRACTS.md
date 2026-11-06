# World Loom — Wayfarer Thread and Domain Integration Contract

> Version 1.0 · Normative companion to docs/MOONBYTE_FINAL_VISION.md

This document defines how independently owned Moonbyte domains exchange state without becoming isolated feature islands. The integration layer is an application contract, not a separate runtime service or a new database broker.

## 1. Ownership

Each domain owns its own rules, writes and tables. A consumer never updates another domain's private tables directly.

| Domain owner | Owns | Other domains may consume |
|---|---|---|
| Game runtime | Session membership, legal actions, game result and rules version. | Completion event, result ID, game type, participants and outcome. |
| Atlas | Location catalogue, route validation, current location and discovery history. | Stable location IDs and first-discovery event. |
| Expedition | Expedition definition, run state, checkpoints and completion. | Run ID, definition version, checkpoint and completion event. |
| Wayfarer | Unique journey-day record, daily objective selection/progress and companion milestones. | Verified activity source and milestone projection. |
| Chronicle / Archive | Private lore fragments, collection records and provenance. | Stable discovery/story IDs and visibility-aware collection entries. |
| Mastery | Rank rules, thresholds, personal bests and optional difficulty access. | Verified source results, never client-supplied totals. |
| Character / Companion | Character identity, cosmetic stage and equipped items. | Authorised milestone and inventory references. |
| Guild | Guild-owned projects, scoped contributions and shared Chronicle. | Explicitly shared, guild-authorised events only. |
| Craft / Inventory | Item catalogue, ownership, recipe inputs/outputs and provenance. | Known material sources and item IDs. |
| Bazaar / Economy | Eligible listings, fixed prices, reservations, ledger and settlement. | Provenance checks and atomic trade outcomes. |
| Trust / Security | Permissions, moderation, privacy scope, rate limits and audit policy. | Every domain's access and disclosure decision. |

## 2. Event envelope

Every event that crosses a domain boundary must define:
- **event_id:** stable unique event identity.
- **event_name:** namespaced lower-case name such as atlas.location.discovered.
- **schema_version:** positive integer for payload compatibility.
- **source_id:** the owning aggregate, such as a game session, expedition run or trade.
- **occurred_at:** UTC timestamp from the server.
- **idempotency_key:** stable key unique to the source transition and consumer.
- **player_scope:** authenticated player IDs required by the consumer.
- **guild_scope:** optional guild ID, present only when the event is genuinely guild-owned or explicitly shared.
- **payload:** the minimum typed stable identifiers and values.

Do not include raw message text, tokens, private seeds, full Discord payloads or arbitrary player prose. Do not infer a guild scope from the channel where a personal action happens.

## 3. Canonical events

| Event | Owning source | Primary consumers | Required rule |
|---|---|---|---|
| game.session.completed | Game runtime | Mastery, Wayfarer, Achievement, Profile | Emit once after result commit; result ID is the idempotency source. |
| atlas.location.discovered | Atlas | Chronicle, Quest, Wayfarer, Expedition | First-discovery only; revisits are a separate visit update, not a discovery event. |
| expedition.started | Expedition | Profile, optional party view | Explicitly confirmed start; no automatic party creation. |
| expedition.checkpoint.reached | Expedition | Chronicle, Wayfarer, Guild when shared | Checkpoint and Atlas state must already be durable. |
| expedition.completed | Expedition | Wayfarer, Quest, Mastery, Archive, Guild when shared | One completion event per run; completion and daily objective update are transactional. |
| quest.completed | Quest | Wayfarer, Mastery, Chronicle | Objective and reward eligibility are verified by the owning service. |
| craft.item.created | Craft | Inventory, Archive, Bazaar eligibility | Recipe inputs and output are deterministic and committed together. |
| bazaar.trade.completed | Bazaar | Inventory, provenance, Want Board, optional reputation | Buyer, seller, escrow and ledger settle atomically. |
| mastery.rank.changed | Mastery | Player Card, Atlas access, Arcade, Expedition | Rank threshold is server-derived and versioned. |
| wayfarer.milestone.reached | Wayfarer | Companion, Player Card, Tavern | Unique milestone; absence never removes it. |
| companion.stage.changed | Companion | Player Card, Tavern, party display | Cosmetic stage is permanent and privately owned. |
| guild.milestone.completed | Guild | Guild Chronicle, Guildhall, member profile projection | Guild ID and contributor visibility must be validated. |
| rivalry.match.completed | Rivalry / game runtime | Both private Rivalry Logs, optional Nemesis flavour | Both players must be active consenting participants. |
| festival.set.earned | Festival | Almanac, Archive, Profile | Set ID and reprise schedule are fixed and published. |

## 4. Commit and idempotency rules

1. The source domain validates the player, state version, permissions, phase and ownership.
2. The source writes the state transition and its source-owned records in one PostgreSQL transaction.
3. A consumer applies its own change through its own service and a unique idempotency key.
4. If a consumer fails, retry the same source event; do not recreate the source action or issue a second reward.
5. A Discord response is sent only after the durable write succeeds. A failed response must not reverse a committed result.
6. For a handoff spanning multiple tables in one domain, prefer a single transaction. Cross-domain writes require an explicit application service that documents ordering and retry recovery.
7. Do not hold a transaction across Discord HTTP calls or other slow network operations.

The first release can use in-process Rust application services. A broker, Redis, distributed event stream or standalone event service is not part of the accepted design.

## 5. Continuity Moment contract

A continuity moment is a contextual, dismissible summary rendered after a verified event. It contains what happened, what changed in saved state, which related systems received an update, one optional next action if the user is eligible, and a clear dismiss/return path.

It must not start an activity, accept an invitation, publish a post, list an item, send a DM or spend currency without a separate explicit confirmation. Never create notification spam. Respect per-player preferences, quiet hours and guild notification limits.

## 6. Unified Player Card contract

The Player Card reads projections from domain services; it does not duplicate domain ownership. It is private by default. A public or guild card requires a separate authorised projection that checks profile visibility, statistics preference, each source domain's share setting, guild membership and rivalry participants' shared-display consent.

A missing permission results in omission or a neutral unavailable label, never a fallback to raw private data.

## 7. Integration invariants

- Events are derived from committed state, not client claims.
- First-discovery and completion events are idempotent.
- One player-day record is written per player/date, even if several domains qualify that day.
- A guild projection never changes personal ownership.
- A personal event does not become public simply because it occurred in a server.
- A failed handoff does not roll back an already committed game result.
- Rewards must be deterministic, fixed and visible; consumers cannot invent additional currency.
- Export and deletion include all durable player-owned records added by a consumer.
- Access-control, moderation, localisation and accessibility apply to every domain.
