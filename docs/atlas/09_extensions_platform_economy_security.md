# Volume IX — Extensions, Platform, Economy and Security

> Moonbyte World Loom · Expansion II · 40 additional proposals · Design proposal only.

<!-- WORLD_LOOM_GRAPH_DATA {"domains":[{"id":"extensions","title":"Curated Extensions and Safe Creator Interfaces","summary":"Moonbyte grows through declarative content packs and narrow manifests. Arbitrary user code and executable plugins are excluded.","links":[["platform","runs_on","uses the Rust application and Discord Activity"],["creator","authored_through","reuses templates and review"],["trust","constrained_by","enforces schemas and permissions"],["fairness","audited_with","preserves deterministic rules"]],"items":[["extension_manifest","Extension Manifest Standard","A manifest declares identity, author, compatible versions, content types, dependencies and permissions.","Draft it, validate fields, inspect capabilities and submit for review.","A schema-validated manifest."],["extension_capabilities","Capability Permission Catalogue","Extensions request named abilities such as adding a puzzle pack or read-only view.","Inspect a capability, grant minimum scope and revoke when needed.","A capability grant."],["extension_events","Domain Event Subscription","Approved extensions receive filtered, read-only named events.","Choose an event, subscribe in scope, validate payloads and unsubscribe.","A typed subscription."],["extension_content_packs","Declarative Content Packs","Extensions add validated story, puzzle, map or cosmetic definitions through schemas.","Build a pack, validate, review and publish a version.","A content pack."],["extension_compatibility","Compatibility Matrix","The platform records which versions work with each game schema and runtime.","Compare schemas, run fixtures and mark supported versions.","A compatibility record."],["extension_sandbox","Extension Test Sandbox","Authors test content with synthetic fixtures isolated from player records.","Create a sandbox, run tests, inspect safe logs and destroy it.","A disposable test report."],["extension_review","Extension Review Queue","Extensions are reviewed for provenance, permissions, content, privacy and compatibility.","Submit, inspect feedback, revise and receive approval or rejection.","A review record."],["extension_revocation","Extension Revocation Desk","Operators disable compromised content while preserving player records.","Inspect dependencies, disable launches, archive state and notify users.","A revocation event."],["extension_market_index","Community Pack Directory","A directory lists reviewed free packs with authorship and compatibility.","Browse, inspect permissions, enable content and remove later.","A pack selection."],["extension_api_contracts","Application Service Contracts","Public integration points expose typed requests, never direct SQL or trusted outcomes.","Validate input, invoke the permitted service and handle its result.","A versioned service contract."]]},{"id":"platform","title":"Discord Activity and Client Continuity","summary":"Discord remains primary. Rich Activity views improve boards and tools while native commands remain usable if the web surface fails.","links":[["trust","governed_by","inherits privacy and accessibility"],["multiplayer","renders","shows validated lobby state"],["hearth","entered_from","offers Tavern deep links"],["arcade","presents","hosts catalogue and boards"]],"items":[["platform_activity_bootstrap","Activity Bootstrap Contract","Startup validates Discord launch context, obtains authorised data and handles outside-Discord launch.","Validate server-side context and show fallback if invalid.","An authorised Activity context."],["platform_deep_links","World Deep-Link Registry","Stable links route users to games, lobbies, quests, items or map nodes with permission checks.","Open a link, verify audience and target, then load or reject.","A canonical destination."],["platform_session_handoff","Session Handoff","A user moves from a Discord interaction into the same Activity session without duplication.","Open handoff, authenticate the same participant and continue the current phase.","A short-lived session view token."],["platform_reduced_surface","Discord-Only Fallback","Every Activity feature defines native Discord controls for core actions and status.","Use native board after Activity failure and continue at the same state.","A fallback view model."],["platform_mobile_layout","Mobile-First Board Layout","Activity boards adapt to narrow screens with grouped actions and clear labels.","Open a narrow layout, act, inspect state and use the exit control.","Responsive layout rules."],["platform_view_models","Authorised View Model Registry","Rust publishes public, private and moderator views with audience-specific fields.","Request a view, validate scope, serialise allowed fields and reject mismatches.","A typed view model."],["platform_interaction_expiry","Interaction Expiry Recovery","Expired interactions show how to reopen canonical state rather than retry stale controls.","Detect expiry, fetch authorised state and render fresh controls.","A refreshed session view."],["platform_asset_delivery","Pixel Asset Delivery","The frontend loads optimised rights-cleared assets by immutable ID with alt text.","Validate access, load rendition and render text fallback on failure.","An asset reference."],["platform_theme_profiles","World Presentation Profiles","Players choose pixel-art variants, density and motion settings across surfaces.","Preview, apply and retain identical mechanics across themes.","A presentation profile."],["platform_cross_device_resume","Cross-Client Resume","The same saved session can reopen from another authorised client using persisted state.","Authenticate, inspect current phase and resume with fresh controls.","A recovered session view."]]},{"id":"economy_governance","title":"Economy Stewardship and Market Integrity","summary":"The economy remains earned-only and non-redeemable. Governance exposes sources, sinks, eligibility and ledger invariants.","links":[["bazaar","governs","sets market and escrow policies"],["craft","balances_inputs_for","checks recipe inputs and outputs"],["fairness","audits","verifies awards and transactions"],["trust","constrained_by","enforces privacy and anti-abuse"]],"items":[["economy_reward_registry","Reward Source Registry","Every award is tied to a published activity, formula, eligibility rule and idempotency key.","Inspect source, complete an activity and write one canonical event.","A reward definition."],["economy_sink_catalogue","Currency Use Catalogue","Every currency sink lists purpose, cost and gameplay or cosmetic effect.","Inspect full price and confirm a specific service.","A documented sink."],["economy_ledger_audit","Immutable Ledger Audit","A read-only audit recomputes balances from immutable entries and flags inconsistencies.","Recompute totals, inspect reason codes and resolve with an auditable correction.","A reconciliation report."],["economy_trade_eligibility","Trade Eligibility Registry","Each item defines whether it is bound, giftable, fixed-price tradable or barter eligible.","Inspect an item and enforce its rule at listing and transfer.","A versioned eligibility rule."],["economy_fees","Published Fee Schedule","Any market fee is stable and shown before confirmation, with zero as a valid default.","Inspect total, confirm trade and compare receipt to published rule.","A fee schedule."],["economy_refunds","Transaction Reversal Policy","Refunds are compensating ledger entries linked to the original, never silent edits.","Review evidence and issue one linked reversal if approved.","A compensating transaction."],["economy_inflation_watch","Economy Health Snapshot","Aggregate reports show sources, sinks and trade volume without individual profiles.","Inspect aggregate totals and compare cohorts above privacy thresholds.","A privacy-safe snapshot."],["economy_item_sinks","Item Circulation Rules","Eligible items have known crafting, display, repair and trade routes without forced destruction.","Inspect valid uses and choose a transparent conversion.","An item lifecycle graph."],["economy_market_limits","Market Safety Limits","Visible listing and action limits prevent spam without surprise cooldown traps.","Read limits, create permitted listing and manage stock.","A market policy."],["economy_public_policy","Economy Policy Charter","A player-facing charter states Moonbits have no cash value and are never purchased or wagered.","Review earning and spending rules and follow changes.","A versioned disclosure."]]},{"id":"security","title":"Security, Abuse Prevention and Account Safety","summary":"Security is designed into identity, permissions, persistence and operations with narrow, explainable controls.","links":[["trust","owned_by","uses privacy and operational trust"],["fairness","protects","checks outcomes and ledgers"],["multiplayer","secures","validates membership and turns"],["bazaar","protects","guards escrow and transfers"]],"items":[["security_threat_model","Threat Model Register","A maintained model lists assets, trust boundaries, attacker capabilities and mitigations.","Inspect a subsystem, add a risk and track mitigation.","A versioned threat model."],["security_action_nonce","Interaction Nonce Registry","Mutations use one-use keys bound to actor, session and action.","Validate, atomically consume and return the stored result for retries.","A consumed action key."],["security_capability_tokens","Scoped Capability Tokens","Short-lived tokens grant one Activity permission, never general database authority.","Issue scoped token, validate audience and revoke after use.","A short-lived capability."],["security_session_binding","Discord Session Binding","Actions bind to authenticated Discord user and guild, not browser IDs.","Validate interaction context and reject mismatched scope.","An authenticated actor context."],["security_rate_limits","Adaptive Request Limits","Per-user, per-guild and per-feature limits protect service reliability with clear guidance.","Check bounded counters and preserve unrelated progress on rejection.","A rate-limit decision."],["security_secret_rotation","Secret Rotation Procedure","A procedure rotates Discord, database and signing credentials and identifies sessions to revoke.","Prepare, validate, switch, revoke old value and audit.","A rotation record without secret values."],["security_backup_restore","Database Backup and Restore Drill","A tested process backs up SQLite and restores into an isolated environment.","Create backup, verify, restore and compare expected records.","A restore report."],["security_migration_guards","Migration Integrity Guards","Migrations validate schema version, constraints and indexes before accepting play.","Apply migration, run invariant checks and stop on invalid state.","A migration result."],["security_audit_events","Security Audit Events","Sensitive administration creates minimal append-only events without private payloads.","Authorise action, execute, record scope and expose to reviewers.","A redacted audit event."],["security_incident_playbook","Incident Response Playbook","An operator playbook covers token leaks, duplicate trades, spoofing and data incidents.","Contain, preserve safe evidence, notify scope and verify recovery.","An incident checklist."]]}]} -->

Each proposal defines its mechanic, player loop, cross-domain connections, Rust ownership, persistence, privacy, Discord and Activity behaviour, accessibility, testing and release gates.

## Curated Extensions and Safe Creator Interfaces (`extensions`)

Moonbyte grows through declarative content packs and narrow manifests. Arbitrary user code and executable plugins are excluded.

**Domain connections:** platform via runs_on — uses the Rust application and Discord Activity; creator via authored_through — reuses templates and review; trust via constrained_by — enforces schemas and permissions; fairness via audited_with — preserves deterministic rules.

# extension_manifest — Extension Manifest Standard
> Expansion II · Curated Extensions and Safe Creator Interfaces · Proposal 1 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A manifest declares identity, author, compatible versions, content types, dependencies and permissions.
- Player need: Draft it, validate fields, inspect capabilities and submit for review.
- Why distinct: Extension Manifest Standard owns a specific new mechanic and does not replace an existing domain.
- Successful output: A schema-validated manifest.
### 02 · Product boundary
- Moonbyte grows through declarative content packs and narrow manifests. Arbitrary user code and executable plugins are excluded.
- Owner: Curated Extensions and Safe Creator Interfaces (`extensions`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `extension_manifest`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Draft it, validate fields, inspect capabilities and submit for review.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A manifest declares identity, author, compatible versions, content types, dependencies and permissions.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to platform: uses the Rust application and Discord Activity.
- Relationship type: `runs_on`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reuses templates and review.
- Relationship type: `authored_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to trust: enforces schemas and permissions.
- Relationship type: `constrained_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to fairness: preserves deterministic rules.
- Relationship type: `audited_with`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A schema-validated manifest.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# extension_capabilities — Capability Permission Catalogue
> Expansion II · Curated Extensions and Safe Creator Interfaces · Proposal 2 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Extensions request named abilities such as adding a puzzle pack or read-only view.
- Player need: Inspect a capability, grant minimum scope and revoke when needed.
- Why distinct: Capability Permission Catalogue owns a specific new mechanic and does not replace an existing domain.
- Successful output: A capability grant.
### 02 · Product boundary
- Moonbyte grows through declarative content packs and narrow manifests. Arbitrary user code and executable plugins are excluded.
- Owner: Curated Extensions and Safe Creator Interfaces (`extensions`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `extension_capabilities`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect a capability, grant minimum scope and revoke when needed.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Extensions request named abilities such as adding a puzzle pack or read-only view.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to platform: uses the Rust application and Discord Activity.
- Relationship type: `runs_on`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reuses templates and review.
- Relationship type: `authored_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to trust: enforces schemas and permissions.
- Relationship type: `constrained_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to fairness: preserves deterministic rules.
- Relationship type: `audited_with`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A capability grant.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# extension_events — Domain Event Subscription
> Expansion II · Curated Extensions and Safe Creator Interfaces · Proposal 3 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Approved extensions receive filtered, read-only named events.
- Player need: Choose an event, subscribe in scope, validate payloads and unsubscribe.
- Why distinct: Domain Event Subscription owns a specific new mechanic and does not replace an existing domain.
- Successful output: A typed subscription.
### 02 · Product boundary
- Moonbyte grows through declarative content packs and narrow manifests. Arbitrary user code and executable plugins are excluded.
- Owner: Curated Extensions and Safe Creator Interfaces (`extensions`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `extension_events`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Choose an event, subscribe in scope, validate payloads and unsubscribe.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Approved extensions receive filtered, read-only named events.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to platform: uses the Rust application and Discord Activity.
- Relationship type: `runs_on`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reuses templates and review.
- Relationship type: `authored_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to trust: enforces schemas and permissions.
- Relationship type: `constrained_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to fairness: preserves deterministic rules.
- Relationship type: `audited_with`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A typed subscription.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# extension_content_packs — Declarative Content Packs
> Expansion II · Curated Extensions and Safe Creator Interfaces · Proposal 4 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Extensions add validated story, puzzle, map or cosmetic definitions through schemas.
- Player need: Build a pack, validate, review and publish a version.
- Why distinct: Declarative Content Packs owns a specific new mechanic and does not replace an existing domain.
- Successful output: A content pack.
### 02 · Product boundary
- Moonbyte grows through declarative content packs and narrow manifests. Arbitrary user code and executable plugins are excluded.
- Owner: Curated Extensions and Safe Creator Interfaces (`extensions`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `extension_content_packs`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Build a pack, validate, review and publish a version.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Extensions add validated story, puzzle, map or cosmetic definitions through schemas.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to platform: uses the Rust application and Discord Activity.
- Relationship type: `runs_on`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reuses templates and review.
- Relationship type: `authored_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to trust: enforces schemas and permissions.
- Relationship type: `constrained_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to fairness: preserves deterministic rules.
- Relationship type: `audited_with`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A content pack.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# extension_compatibility — Compatibility Matrix
> Expansion II · Curated Extensions and Safe Creator Interfaces · Proposal 5 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- The platform records which versions work with each game schema and runtime.
- Player need: Compare schemas, run fixtures and mark supported versions.
- Why distinct: Compatibility Matrix owns a specific new mechanic and does not replace an existing domain.
- Successful output: A compatibility record.
### 02 · Product boundary
- Moonbyte grows through declarative content packs and narrow manifests. Arbitrary user code and executable plugins are excluded.
- Owner: Curated Extensions and Safe Creator Interfaces (`extensions`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `extension_compatibility`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Compare schemas, run fixtures and mark supported versions.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- The platform records which versions work with each game schema and runtime.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to platform: uses the Rust application and Discord Activity.
- Relationship type: `runs_on`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reuses templates and review.
- Relationship type: `authored_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to trust: enforces schemas and permissions.
- Relationship type: `constrained_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to fairness: preserves deterministic rules.
- Relationship type: `audited_with`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A compatibility record.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# extension_sandbox — Extension Test Sandbox
> Expansion II · Curated Extensions and Safe Creator Interfaces · Proposal 6 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Authors test content with synthetic fixtures isolated from player records.
- Player need: Create a sandbox, run tests, inspect safe logs and destroy it.
- Why distinct: Extension Test Sandbox owns a specific new mechanic and does not replace an existing domain.
- Successful output: A disposable test report.
### 02 · Product boundary
- Moonbyte grows through declarative content packs and narrow manifests. Arbitrary user code and executable plugins are excluded.
- Owner: Curated Extensions and Safe Creator Interfaces (`extensions`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `extension_sandbox`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Create a sandbox, run tests, inspect safe logs and destroy it.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Authors test content with synthetic fixtures isolated from player records.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to platform: uses the Rust application and Discord Activity.
- Relationship type: `runs_on`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reuses templates and review.
- Relationship type: `authored_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to trust: enforces schemas and permissions.
- Relationship type: `constrained_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to fairness: preserves deterministic rules.
- Relationship type: `audited_with`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A disposable test report.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# extension_review — Extension Review Queue
> Expansion II · Curated Extensions and Safe Creator Interfaces · Proposal 7 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Extensions are reviewed for provenance, permissions, content, privacy and compatibility.
- Player need: Submit, inspect feedback, revise and receive approval or rejection.
- Why distinct: Extension Review Queue owns a specific new mechanic and does not replace an existing domain.
- Successful output: A review record.
### 02 · Product boundary
- Moonbyte grows through declarative content packs and narrow manifests. Arbitrary user code and executable plugins are excluded.
- Owner: Curated Extensions and Safe Creator Interfaces (`extensions`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `extension_review`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Submit, inspect feedback, revise and receive approval or rejection.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Extensions are reviewed for provenance, permissions, content, privacy and compatibility.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to platform: uses the Rust application and Discord Activity.
- Relationship type: `runs_on`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reuses templates and review.
- Relationship type: `authored_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to trust: enforces schemas and permissions.
- Relationship type: `constrained_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to fairness: preserves deterministic rules.
- Relationship type: `audited_with`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A review record.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# extension_revocation — Extension Revocation Desk
> Expansion II · Curated Extensions and Safe Creator Interfaces · Proposal 8 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Operators disable compromised content while preserving player records.
- Player need: Inspect dependencies, disable launches, archive state and notify users.
- Why distinct: Extension Revocation Desk owns a specific new mechanic and does not replace an existing domain.
- Successful output: A revocation event.
### 02 · Product boundary
- Moonbyte grows through declarative content packs and narrow manifests. Arbitrary user code and executable plugins are excluded.
- Owner: Curated Extensions and Safe Creator Interfaces (`extensions`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `extension_revocation`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect dependencies, disable launches, archive state and notify users.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Operators disable compromised content while preserving player records.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to platform: uses the Rust application and Discord Activity.
- Relationship type: `runs_on`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reuses templates and review.
- Relationship type: `authored_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to trust: enforces schemas and permissions.
- Relationship type: `constrained_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to fairness: preserves deterministic rules.
- Relationship type: `audited_with`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A revocation event.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# extension_market_index — Community Pack Directory
> Expansion II · Curated Extensions and Safe Creator Interfaces · Proposal 9 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A directory lists reviewed free packs with authorship and compatibility.
- Player need: Browse, inspect permissions, enable content and remove later.
- Why distinct: Community Pack Directory owns a specific new mechanic and does not replace an existing domain.
- Successful output: A pack selection.
### 02 · Product boundary
- Moonbyte grows through declarative content packs and narrow manifests. Arbitrary user code and executable plugins are excluded.
- Owner: Curated Extensions and Safe Creator Interfaces (`extensions`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `extension_market_index`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Browse, inspect permissions, enable content and remove later.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A directory lists reviewed free packs with authorship and compatibility.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to platform: uses the Rust application and Discord Activity.
- Relationship type: `runs_on`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reuses templates and review.
- Relationship type: `authored_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to trust: enforces schemas and permissions.
- Relationship type: `constrained_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to fairness: preserves deterministic rules.
- Relationship type: `audited_with`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A pack selection.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# extension_api_contracts — Application Service Contracts
> Expansion II · Curated Extensions and Safe Creator Interfaces · Proposal 10 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Public integration points expose typed requests, never direct SQL or trusted outcomes.
- Player need: Validate input, invoke the permitted service and handle its result.
- Why distinct: Application Service Contracts owns a specific new mechanic and does not replace an existing domain.
- Successful output: A versioned service contract.
### 02 · Product boundary
- Moonbyte grows through declarative content packs and narrow manifests. Arbitrary user code and executable plugins are excluded.
- Owner: Curated Extensions and Safe Creator Interfaces (`extensions`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `extension_api_contracts`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Validate input, invoke the permitted service and handle its result.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Public integration points expose typed requests, never direct SQL or trusted outcomes.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to platform: uses the Rust application and Discord Activity.
- Relationship type: `runs_on`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reuses templates and review.
- Relationship type: `authored_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to trust: enforces schemas and permissions.
- Relationship type: `constrained_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to fairness: preserves deterministic rules.
- Relationship type: `audited_with`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A versioned service contract.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---
## Discord Activity and Client Continuity (`platform`)

Discord remains primary. Rich Activity views improve boards and tools while native commands remain usable if the web surface fails.

**Domain connections:** trust via governed_by — inherits privacy and accessibility; multiplayer via renders — shows validated lobby state; hearth via entered_from — offers Tavern deep links; arcade via presents — hosts catalogue and boards.

# platform_activity_bootstrap — Activity Bootstrap Contract
> Expansion II · Discord Activity and Client Continuity · Proposal 1 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Startup validates Discord launch context, obtains authorised data and handles outside-Discord launch.
- Player need: Validate server-side context and show fallback if invalid.
- Why distinct: Activity Bootstrap Contract owns a specific new mechanic and does not replace an existing domain.
- Successful output: An authorised Activity context.
### 02 · Product boundary
- Discord remains primary. Rich Activity views improve boards and tools while native commands remain usable if the web surface fails.
- Owner: Discord Activity and Client Continuity (`platform`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `platform_activity_bootstrap`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Validate server-side context and show fallback if invalid.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Startup validates Discord launch context, obtains authorised data and handles outside-Discord launch.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits privacy and accessibility.
- Relationship type: `governed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to multiplayer: shows validated lobby state.
- Relationship type: `renders`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to hearth: offers Tavern deep links.
- Relationship type: `entered_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: hosts catalogue and boards.
- Relationship type: `presents`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- An authorised Activity context.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# platform_deep_links — World Deep-Link Registry
> Expansion II · Discord Activity and Client Continuity · Proposal 2 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Stable links route users to games, lobbies, quests, items or map nodes with permission checks.
- Player need: Open a link, verify audience and target, then load or reject.
- Why distinct: World Deep-Link Registry owns a specific new mechanic and does not replace an existing domain.
- Successful output: A canonical destination.
### 02 · Product boundary
- Discord remains primary. Rich Activity views improve boards and tools while native commands remain usable if the web surface fails.
- Owner: Discord Activity and Client Continuity (`platform`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `platform_deep_links`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Open a link, verify audience and target, then load or reject.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Stable links route users to games, lobbies, quests, items or map nodes with permission checks.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits privacy and accessibility.
- Relationship type: `governed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to multiplayer: shows validated lobby state.
- Relationship type: `renders`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to hearth: offers Tavern deep links.
- Relationship type: `entered_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: hosts catalogue and boards.
- Relationship type: `presents`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A canonical destination.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# platform_session_handoff — Session Handoff
> Expansion II · Discord Activity and Client Continuity · Proposal 3 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A user moves from a Discord interaction into the same Activity session without duplication.
- Player need: Open handoff, authenticate the same participant and continue the current phase.
- Why distinct: Session Handoff owns a specific new mechanic and does not replace an existing domain.
- Successful output: A short-lived session view token.
### 02 · Product boundary
- Discord remains primary. Rich Activity views improve boards and tools while native commands remain usable if the web surface fails.
- Owner: Discord Activity and Client Continuity (`platform`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `platform_session_handoff`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Open handoff, authenticate the same participant and continue the current phase.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A user moves from a Discord interaction into the same Activity session without duplication.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits privacy and accessibility.
- Relationship type: `governed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to multiplayer: shows validated lobby state.
- Relationship type: `renders`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to hearth: offers Tavern deep links.
- Relationship type: `entered_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: hosts catalogue and boards.
- Relationship type: `presents`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A short-lived session view token.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# platform_reduced_surface — Discord-Only Fallback
> Expansion II · Discord Activity and Client Continuity · Proposal 4 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Every Activity feature defines native Discord controls for core actions and status.
- Player need: Use native board after Activity failure and continue at the same state.
- Why distinct: Discord-Only Fallback owns a specific new mechanic and does not replace an existing domain.
- Successful output: A fallback view model.
### 02 · Product boundary
- Discord remains primary. Rich Activity views improve boards and tools while native commands remain usable if the web surface fails.
- Owner: Discord Activity and Client Continuity (`platform`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `platform_reduced_surface`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Use native board after Activity failure and continue at the same state.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Every Activity feature defines native Discord controls for core actions and status.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits privacy and accessibility.
- Relationship type: `governed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to multiplayer: shows validated lobby state.
- Relationship type: `renders`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to hearth: offers Tavern deep links.
- Relationship type: `entered_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: hosts catalogue and boards.
- Relationship type: `presents`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A fallback view model.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# platform_mobile_layout — Mobile-First Board Layout
> Expansion II · Discord Activity and Client Continuity · Proposal 5 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Activity boards adapt to narrow screens with grouped actions and clear labels.
- Player need: Open a narrow layout, act, inspect state and use the exit control.
- Why distinct: Mobile-First Board Layout owns a specific new mechanic and does not replace an existing domain.
- Successful output: Responsive layout rules.
### 02 · Product boundary
- Discord remains primary. Rich Activity views improve boards and tools while native commands remain usable if the web surface fails.
- Owner: Discord Activity and Client Continuity (`platform`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `platform_mobile_layout`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Open a narrow layout, act, inspect state and use the exit control.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Activity boards adapt to narrow screens with grouped actions and clear labels.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits privacy and accessibility.
- Relationship type: `governed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to multiplayer: shows validated lobby state.
- Relationship type: `renders`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to hearth: offers Tavern deep links.
- Relationship type: `entered_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: hosts catalogue and boards.
- Relationship type: `presents`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- Responsive layout rules.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# platform_view_models — Authorised View Model Registry
> Expansion II · Discord Activity and Client Continuity · Proposal 6 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Rust publishes public, private and moderator views with audience-specific fields.
- Player need: Request a view, validate scope, serialise allowed fields and reject mismatches.
- Why distinct: Authorised View Model Registry owns a specific new mechanic and does not replace an existing domain.
- Successful output: A typed view model.
### 02 · Product boundary
- Discord remains primary. Rich Activity views improve boards and tools while native commands remain usable if the web surface fails.
- Owner: Discord Activity and Client Continuity (`platform`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `platform_view_models`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Request a view, validate scope, serialise allowed fields and reject mismatches.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Rust publishes public, private and moderator views with audience-specific fields.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits privacy and accessibility.
- Relationship type: `governed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to multiplayer: shows validated lobby state.
- Relationship type: `renders`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to hearth: offers Tavern deep links.
- Relationship type: `entered_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: hosts catalogue and boards.
- Relationship type: `presents`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A typed view model.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# platform_interaction_expiry — Interaction Expiry Recovery
> Expansion II · Discord Activity and Client Continuity · Proposal 7 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Expired interactions show how to reopen canonical state rather than retry stale controls.
- Player need: Detect expiry, fetch authorised state and render fresh controls.
- Why distinct: Interaction Expiry Recovery owns a specific new mechanic and does not replace an existing domain.
- Successful output: A refreshed session view.
### 02 · Product boundary
- Discord remains primary. Rich Activity views improve boards and tools while native commands remain usable if the web surface fails.
- Owner: Discord Activity and Client Continuity (`platform`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `platform_interaction_expiry`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Detect expiry, fetch authorised state and render fresh controls.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Expired interactions show how to reopen canonical state rather than retry stale controls.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits privacy and accessibility.
- Relationship type: `governed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to multiplayer: shows validated lobby state.
- Relationship type: `renders`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to hearth: offers Tavern deep links.
- Relationship type: `entered_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: hosts catalogue and boards.
- Relationship type: `presents`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A refreshed session view.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# platform_asset_delivery — Pixel Asset Delivery
> Expansion II · Discord Activity and Client Continuity · Proposal 8 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- The frontend loads optimised rights-cleared assets by immutable ID with alt text.
- Player need: Validate access, load rendition and render text fallback on failure.
- Why distinct: Pixel Asset Delivery owns a specific new mechanic and does not replace an existing domain.
- Successful output: An asset reference.
### 02 · Product boundary
- Discord remains primary. Rich Activity views improve boards and tools while native commands remain usable if the web surface fails.
- Owner: Discord Activity and Client Continuity (`platform`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `platform_asset_delivery`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Validate access, load rendition and render text fallback on failure.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- The frontend loads optimised rights-cleared assets by immutable ID with alt text.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits privacy and accessibility.
- Relationship type: `governed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to multiplayer: shows validated lobby state.
- Relationship type: `renders`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to hearth: offers Tavern deep links.
- Relationship type: `entered_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: hosts catalogue and boards.
- Relationship type: `presents`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- An asset reference.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# platform_theme_profiles — World Presentation Profiles
> Expansion II · Discord Activity and Client Continuity · Proposal 9 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Players choose pixel-art variants, density and motion settings across surfaces.
- Player need: Preview, apply and retain identical mechanics across themes.
- Why distinct: World Presentation Profiles owns a specific new mechanic and does not replace an existing domain.
- Successful output: A presentation profile.
### 02 · Product boundary
- Discord remains primary. Rich Activity views improve boards and tools while native commands remain usable if the web surface fails.
- Owner: Discord Activity and Client Continuity (`platform`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `platform_theme_profiles`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Preview, apply and retain identical mechanics across themes.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Players choose pixel-art variants, density and motion settings across surfaces.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits privacy and accessibility.
- Relationship type: `governed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to multiplayer: shows validated lobby state.
- Relationship type: `renders`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to hearth: offers Tavern deep links.
- Relationship type: `entered_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: hosts catalogue and boards.
- Relationship type: `presents`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A presentation profile.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# platform_cross_device_resume — Cross-Client Resume
> Expansion II · Discord Activity and Client Continuity · Proposal 10 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- The same saved session can reopen from another authorised client using persisted state.
- Player need: Authenticate, inspect current phase and resume with fresh controls.
- Why distinct: Cross-Client Resume owns a specific new mechanic and does not replace an existing domain.
- Successful output: A recovered session view.
### 02 · Product boundary
- Discord remains primary. Rich Activity views improve boards and tools while native commands remain usable if the web surface fails.
- Owner: Discord Activity and Client Continuity (`platform`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `platform_cross_device_resume`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Authenticate, inspect current phase and resume with fresh controls.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- The same saved session can reopen from another authorised client using persisted state.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits privacy and accessibility.
- Relationship type: `governed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to multiplayer: shows validated lobby state.
- Relationship type: `renders`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to hearth: offers Tavern deep links.
- Relationship type: `entered_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: hosts catalogue and boards.
- Relationship type: `presents`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A recovered session view.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---
## Economy Stewardship and Market Integrity (`economy_governance`)

The economy remains earned-only and non-redeemable. Governance exposes sources, sinks, eligibility and ledger invariants.

**Domain connections:** bazaar via governs — sets market and escrow policies; craft via balances_inputs_for — checks recipe inputs and outputs; fairness via audits — verifies awards and transactions; trust via constrained_by — enforces privacy and anti-abuse.

# economy_reward_registry — Reward Source Registry
> Expansion II · Economy Stewardship and Market Integrity · Proposal 1 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Every award is tied to a published activity, formula, eligibility rule and idempotency key.
- Player need: Inspect source, complete an activity and write one canonical event.
- Why distinct: Reward Source Registry owns a specific new mechanic and does not replace an existing domain.
- Successful output: A reward definition.
### 02 · Product boundary
- The economy remains earned-only and non-redeemable. Governance exposes sources, sinks, eligibility and ledger invariants.
- Owner: Economy Stewardship and Market Integrity (`economy_governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `economy_reward_registry`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect source, complete an activity and write one canonical event.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Every award is tied to a published activity, formula, eligibility rule and idempotency key.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to bazaar: sets market and escrow policies.
- Relationship type: `governs`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to craft: checks recipe inputs and outputs.
- Relationship type: `balances_inputs_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: verifies awards and transactions.
- Relationship type: `audits`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: enforces privacy and anti-abuse.
- Relationship type: `constrained_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A reward definition.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# economy_sink_catalogue — Currency Use Catalogue
> Expansion II · Economy Stewardship and Market Integrity · Proposal 2 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Every currency sink lists purpose, cost and gameplay or cosmetic effect.
- Player need: Inspect full price and confirm a specific service.
- Why distinct: Currency Use Catalogue owns a specific new mechanic and does not replace an existing domain.
- Successful output: A documented sink.
### 02 · Product boundary
- The economy remains earned-only and non-redeemable. Governance exposes sources, sinks, eligibility and ledger invariants.
- Owner: Economy Stewardship and Market Integrity (`economy_governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `economy_sink_catalogue`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect full price and confirm a specific service.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Every currency sink lists purpose, cost and gameplay or cosmetic effect.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to bazaar: sets market and escrow policies.
- Relationship type: `governs`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to craft: checks recipe inputs and outputs.
- Relationship type: `balances_inputs_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: verifies awards and transactions.
- Relationship type: `audits`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: enforces privacy and anti-abuse.
- Relationship type: `constrained_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A documented sink.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# economy_ledger_audit — Immutable Ledger Audit
> Expansion II · Economy Stewardship and Market Integrity · Proposal 3 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A read-only audit recomputes balances from immutable entries and flags inconsistencies.
- Player need: Recompute totals, inspect reason codes and resolve with an auditable correction.
- Why distinct: Immutable Ledger Audit owns a specific new mechanic and does not replace an existing domain.
- Successful output: A reconciliation report.
### 02 · Product boundary
- The economy remains earned-only and non-redeemable. Governance exposes sources, sinks, eligibility and ledger invariants.
- Owner: Economy Stewardship and Market Integrity (`economy_governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `economy_ledger_audit`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Recompute totals, inspect reason codes and resolve with an auditable correction.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A read-only audit recomputes balances from immutable entries and flags inconsistencies.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to bazaar: sets market and escrow policies.
- Relationship type: `governs`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to craft: checks recipe inputs and outputs.
- Relationship type: `balances_inputs_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: verifies awards and transactions.
- Relationship type: `audits`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: enforces privacy and anti-abuse.
- Relationship type: `constrained_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A reconciliation report.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# economy_trade_eligibility — Trade Eligibility Registry
> Expansion II · Economy Stewardship and Market Integrity · Proposal 4 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Each item defines whether it is bound, giftable, fixed-price tradable or barter eligible.
- Player need: Inspect an item and enforce its rule at listing and transfer.
- Why distinct: Trade Eligibility Registry owns a specific new mechanic and does not replace an existing domain.
- Successful output: A versioned eligibility rule.
### 02 · Product boundary
- The economy remains earned-only and non-redeemable. Governance exposes sources, sinks, eligibility and ledger invariants.
- Owner: Economy Stewardship and Market Integrity (`economy_governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `economy_trade_eligibility`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect an item and enforce its rule at listing and transfer.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Each item defines whether it is bound, giftable, fixed-price tradable or barter eligible.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to bazaar: sets market and escrow policies.
- Relationship type: `governs`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to craft: checks recipe inputs and outputs.
- Relationship type: `balances_inputs_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: verifies awards and transactions.
- Relationship type: `audits`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: enforces privacy and anti-abuse.
- Relationship type: `constrained_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A versioned eligibility rule.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# economy_fees — Published Fee Schedule
> Expansion II · Economy Stewardship and Market Integrity · Proposal 5 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Any market fee is stable and shown before confirmation, with zero as a valid default.
- Player need: Inspect total, confirm trade and compare receipt to published rule.
- Why distinct: Published Fee Schedule owns a specific new mechanic and does not replace an existing domain.
- Successful output: A fee schedule.
### 02 · Product boundary
- The economy remains earned-only and non-redeemable. Governance exposes sources, sinks, eligibility and ledger invariants.
- Owner: Economy Stewardship and Market Integrity (`economy_governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `economy_fees`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect total, confirm trade and compare receipt to published rule.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Any market fee is stable and shown before confirmation, with zero as a valid default.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to bazaar: sets market and escrow policies.
- Relationship type: `governs`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to craft: checks recipe inputs and outputs.
- Relationship type: `balances_inputs_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: verifies awards and transactions.
- Relationship type: `audits`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: enforces privacy and anti-abuse.
- Relationship type: `constrained_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A fee schedule.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# economy_refunds — Transaction Reversal Policy
> Expansion II · Economy Stewardship and Market Integrity · Proposal 6 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Refunds are compensating ledger entries linked to the original, never silent edits.
- Player need: Review evidence and issue one linked reversal if approved.
- Why distinct: Transaction Reversal Policy owns a specific new mechanic and does not replace an existing domain.
- Successful output: A compensating transaction.
### 02 · Product boundary
- The economy remains earned-only and non-redeemable. Governance exposes sources, sinks, eligibility and ledger invariants.
- Owner: Economy Stewardship and Market Integrity (`economy_governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `economy_refunds`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Review evidence and issue one linked reversal if approved.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Refunds are compensating ledger entries linked to the original, never silent edits.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to bazaar: sets market and escrow policies.
- Relationship type: `governs`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to craft: checks recipe inputs and outputs.
- Relationship type: `balances_inputs_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: verifies awards and transactions.
- Relationship type: `audits`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: enforces privacy and anti-abuse.
- Relationship type: `constrained_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A compensating transaction.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# economy_inflation_watch — Economy Health Snapshot
> Expansion II · Economy Stewardship and Market Integrity · Proposal 7 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Aggregate reports show sources, sinks and trade volume without individual profiles.
- Player need: Inspect aggregate totals and compare cohorts above privacy thresholds.
- Why distinct: Economy Health Snapshot owns a specific new mechanic and does not replace an existing domain.
- Successful output: A privacy-safe snapshot.
### 02 · Product boundary
- The economy remains earned-only and non-redeemable. Governance exposes sources, sinks, eligibility and ledger invariants.
- Owner: Economy Stewardship and Market Integrity (`economy_governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `economy_inflation_watch`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect aggregate totals and compare cohorts above privacy thresholds.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Aggregate reports show sources, sinks and trade volume without individual profiles.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to bazaar: sets market and escrow policies.
- Relationship type: `governs`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to craft: checks recipe inputs and outputs.
- Relationship type: `balances_inputs_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: verifies awards and transactions.
- Relationship type: `audits`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: enforces privacy and anti-abuse.
- Relationship type: `constrained_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A privacy-safe snapshot.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# economy_item_sinks — Item Circulation Rules
> Expansion II · Economy Stewardship and Market Integrity · Proposal 8 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Eligible items have known crafting, display, repair and trade routes without forced destruction.
- Player need: Inspect valid uses and choose a transparent conversion.
- Why distinct: Item Circulation Rules owns a specific new mechanic and does not replace an existing domain.
- Successful output: An item lifecycle graph.
### 02 · Product boundary
- The economy remains earned-only and non-redeemable. Governance exposes sources, sinks, eligibility and ledger invariants.
- Owner: Economy Stewardship and Market Integrity (`economy_governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `economy_item_sinks`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect valid uses and choose a transparent conversion.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Eligible items have known crafting, display, repair and trade routes without forced destruction.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to bazaar: sets market and escrow policies.
- Relationship type: `governs`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to craft: checks recipe inputs and outputs.
- Relationship type: `balances_inputs_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: verifies awards and transactions.
- Relationship type: `audits`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: enforces privacy and anti-abuse.
- Relationship type: `constrained_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- An item lifecycle graph.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# economy_market_limits — Market Safety Limits
> Expansion II · Economy Stewardship and Market Integrity · Proposal 9 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Visible listing and action limits prevent spam without surprise cooldown traps.
- Player need: Read limits, create permitted listing and manage stock.
- Why distinct: Market Safety Limits owns a specific new mechanic and does not replace an existing domain.
- Successful output: A market policy.
### 02 · Product boundary
- The economy remains earned-only and non-redeemable. Governance exposes sources, sinks, eligibility and ledger invariants.
- Owner: Economy Stewardship and Market Integrity (`economy_governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `economy_market_limits`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Read limits, create permitted listing and manage stock.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Visible listing and action limits prevent spam without surprise cooldown traps.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to bazaar: sets market and escrow policies.
- Relationship type: `governs`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to craft: checks recipe inputs and outputs.
- Relationship type: `balances_inputs_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: verifies awards and transactions.
- Relationship type: `audits`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: enforces privacy and anti-abuse.
- Relationship type: `constrained_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A market policy.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# economy_public_policy — Economy Policy Charter
> Expansion II · Economy Stewardship and Market Integrity · Proposal 10 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A player-facing charter states Moonbits have no cash value and are never purchased or wagered.
- Player need: Review earning and spending rules and follow changes.
- Why distinct: Economy Policy Charter owns a specific new mechanic and does not replace an existing domain.
- Successful output: A versioned disclosure.
### 02 · Product boundary
- The economy remains earned-only and non-redeemable. Governance exposes sources, sinks, eligibility and ledger invariants.
- Owner: Economy Stewardship and Market Integrity (`economy_governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `economy_public_policy`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Review earning and spending rules and follow changes.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A player-facing charter states Moonbits have no cash value and are never purchased or wagered.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to bazaar: sets market and escrow policies.
- Relationship type: `governs`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to craft: checks recipe inputs and outputs.
- Relationship type: `balances_inputs_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: verifies awards and transactions.
- Relationship type: `audits`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: enforces privacy and anti-abuse.
- Relationship type: `constrained_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A versioned disclosure.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---
## Security, Abuse Prevention and Account Safety (`security`)

Security is designed into identity, permissions, persistence and operations with narrow, explainable controls.

**Domain connections:** trust via owned_by — uses privacy and operational trust; fairness via protects — checks outcomes and ledgers; multiplayer via secures — validates membership and turns; bazaar via protects — guards escrow and transfers.

# security_threat_model — Threat Model Register
> Expansion II · Security, Abuse Prevention and Account Safety · Proposal 1 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A maintained model lists assets, trust boundaries, attacker capabilities and mitigations.
- Player need: Inspect a subsystem, add a risk and track mitigation.
- Why distinct: Threat Model Register owns a specific new mechanic and does not replace an existing domain.
- Successful output: A versioned threat model.
### 02 · Product boundary
- Security is designed into identity, permissions, persistence and operations with narrow, explainable controls.
- Owner: Security, Abuse Prevention and Account Safety (`security`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `security_threat_model`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect a subsystem, add a risk and track mitigation.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A maintained model lists assets, trust boundaries, attacker capabilities and mitigations.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses privacy and operational trust.
- Relationship type: `owned_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to fairness: checks outcomes and ledgers.
- Relationship type: `protects`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to multiplayer: validates membership and turns.
- Relationship type: `secures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to bazaar: guards escrow and transfers.
- Relationship type: `protects`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A versioned threat model.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# security_action_nonce — Interaction Nonce Registry
> Expansion II · Security, Abuse Prevention and Account Safety · Proposal 2 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Mutations use one-use keys bound to actor, session and action.
- Player need: Validate, atomically consume and return the stored result for retries.
- Why distinct: Interaction Nonce Registry owns a specific new mechanic and does not replace an existing domain.
- Successful output: A consumed action key.
### 02 · Product boundary
- Security is designed into identity, permissions, persistence and operations with narrow, explainable controls.
- Owner: Security, Abuse Prevention and Account Safety (`security`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `security_action_nonce`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Validate, atomically consume and return the stored result for retries.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Mutations use one-use keys bound to actor, session and action.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses privacy and operational trust.
- Relationship type: `owned_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to fairness: checks outcomes and ledgers.
- Relationship type: `protects`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to multiplayer: validates membership and turns.
- Relationship type: `secures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to bazaar: guards escrow and transfers.
- Relationship type: `protects`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A consumed action key.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# security_capability_tokens — Scoped Capability Tokens
> Expansion II · Security, Abuse Prevention and Account Safety · Proposal 3 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Short-lived tokens grant one Activity permission, never general database authority.
- Player need: Issue scoped token, validate audience and revoke after use.
- Why distinct: Scoped Capability Tokens owns a specific new mechanic and does not replace an existing domain.
- Successful output: A short-lived capability.
### 02 · Product boundary
- Security is designed into identity, permissions, persistence and operations with narrow, explainable controls.
- Owner: Security, Abuse Prevention and Account Safety (`security`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `security_capability_tokens`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Issue scoped token, validate audience and revoke after use.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Short-lived tokens grant one Activity permission, never general database authority.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses privacy and operational trust.
- Relationship type: `owned_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to fairness: checks outcomes and ledgers.
- Relationship type: `protects`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to multiplayer: validates membership and turns.
- Relationship type: `secures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to bazaar: guards escrow and transfers.
- Relationship type: `protects`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A short-lived capability.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# security_session_binding — Discord Session Binding
> Expansion II · Security, Abuse Prevention and Account Safety · Proposal 4 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Actions bind to authenticated Discord user and guild, not browser IDs.
- Player need: Validate interaction context and reject mismatched scope.
- Why distinct: Discord Session Binding owns a specific new mechanic and does not replace an existing domain.
- Successful output: An authenticated actor context.
### 02 · Product boundary
- Security is designed into identity, permissions, persistence and operations with narrow, explainable controls.
- Owner: Security, Abuse Prevention and Account Safety (`security`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `security_session_binding`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Validate interaction context and reject mismatched scope.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Actions bind to authenticated Discord user and guild, not browser IDs.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses privacy and operational trust.
- Relationship type: `owned_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to fairness: checks outcomes and ledgers.
- Relationship type: `protects`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to multiplayer: validates membership and turns.
- Relationship type: `secures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to bazaar: guards escrow and transfers.
- Relationship type: `protects`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- An authenticated actor context.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# security_rate_limits — Adaptive Request Limits
> Expansion II · Security, Abuse Prevention and Account Safety · Proposal 5 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Per-user, per-guild and per-feature limits protect service reliability with clear guidance.
- Player need: Check bounded counters and preserve unrelated progress on rejection.
- Why distinct: Adaptive Request Limits owns a specific new mechanic and does not replace an existing domain.
- Successful output: A rate-limit decision.
### 02 · Product boundary
- Security is designed into identity, permissions, persistence and operations with narrow, explainable controls.
- Owner: Security, Abuse Prevention and Account Safety (`security`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `security_rate_limits`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Check bounded counters and preserve unrelated progress on rejection.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Per-user, per-guild and per-feature limits protect service reliability with clear guidance.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses privacy and operational trust.
- Relationship type: `owned_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to fairness: checks outcomes and ledgers.
- Relationship type: `protects`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to multiplayer: validates membership and turns.
- Relationship type: `secures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to bazaar: guards escrow and transfers.
- Relationship type: `protects`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A rate-limit decision.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# security_secret_rotation — Secret Rotation Procedure
> Expansion II · Security, Abuse Prevention and Account Safety · Proposal 6 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A procedure rotates Discord, database and signing credentials and identifies sessions to revoke.
- Player need: Prepare, validate, switch, revoke old value and audit.
- Why distinct: Secret Rotation Procedure owns a specific new mechanic and does not replace an existing domain.
- Successful output: A rotation record without secret values.
### 02 · Product boundary
- Security is designed into identity, permissions, persistence and operations with narrow, explainable controls.
- Owner: Security, Abuse Prevention and Account Safety (`security`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `security_secret_rotation`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Prepare, validate, switch, revoke old value and audit.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A procedure rotates Discord, database and signing credentials and identifies sessions to revoke.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses privacy and operational trust.
- Relationship type: `owned_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to fairness: checks outcomes and ledgers.
- Relationship type: `protects`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to multiplayer: validates membership and turns.
- Relationship type: `secures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to bazaar: guards escrow and transfers.
- Relationship type: `protects`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A rotation record without secret values.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# security_backup_restore — Database Backup and Restore Drill
> Expansion II · Security, Abuse Prevention and Account Safety · Proposal 7 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A tested process backs up SQLite and restores into an isolated environment.
- Player need: Create backup, verify, restore and compare expected records.
- Why distinct: Database Backup and Restore Drill owns a specific new mechanic and does not replace an existing domain.
- Successful output: A restore report.
### 02 · Product boundary
- Security is designed into identity, permissions, persistence and operations with narrow, explainable controls.
- Owner: Security, Abuse Prevention and Account Safety (`security`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `security_backup_restore`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Create backup, verify, restore and compare expected records.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A tested process backs up SQLite and restores into an isolated environment.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses privacy and operational trust.
- Relationship type: `owned_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to fairness: checks outcomes and ledgers.
- Relationship type: `protects`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to multiplayer: validates membership and turns.
- Relationship type: `secures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to bazaar: guards escrow and transfers.
- Relationship type: `protects`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A restore report.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# security_migration_guards — Migration Integrity Guards
> Expansion II · Security, Abuse Prevention and Account Safety · Proposal 8 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Migrations validate schema version, constraints and indexes before accepting play.
- Player need: Apply migration, run invariant checks and stop on invalid state.
- Why distinct: Migration Integrity Guards owns a specific new mechanic and does not replace an existing domain.
- Successful output: A migration result.
### 02 · Product boundary
- Security is designed into identity, permissions, persistence and operations with narrow, explainable controls.
- Owner: Security, Abuse Prevention and Account Safety (`security`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `security_migration_guards`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Apply migration, run invariant checks and stop on invalid state.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Migrations validate schema version, constraints and indexes before accepting play.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses privacy and operational trust.
- Relationship type: `owned_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to fairness: checks outcomes and ledgers.
- Relationship type: `protects`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to multiplayer: validates membership and turns.
- Relationship type: `secures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to bazaar: guards escrow and transfers.
- Relationship type: `protects`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A migration result.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# security_audit_events — Security Audit Events
> Expansion II · Security, Abuse Prevention and Account Safety · Proposal 9 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Sensitive administration creates minimal append-only events without private payloads.
- Player need: Authorise action, execute, record scope and expose to reviewers.
- Why distinct: Security Audit Events owns a specific new mechanic and does not replace an existing domain.
- Successful output: A redacted audit event.
### 02 · Product boundary
- Security is designed into identity, permissions, persistence and operations with narrow, explainable controls.
- Owner: Security, Abuse Prevention and Account Safety (`security`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `security_audit_events`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Authorise action, execute, record scope and expose to reviewers.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Sensitive administration creates minimal append-only events without private payloads.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses privacy and operational trust.
- Relationship type: `owned_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to fairness: checks outcomes and ledgers.
- Relationship type: `protects`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to multiplayer: validates membership and turns.
- Relationship type: `secures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to bazaar: guards escrow and transfers.
- Relationship type: `protects`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A redacted audit event.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---

# security_incident_playbook — Incident Response Playbook
> Expansion II · Security, Abuse Prevention and Account Safety · Proposal 10 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- An operator playbook covers token leaks, duplicate trades, spoofing and data incidents.
- Player need: Contain, preserve safe evidence, notify scope and verify recovery.
- Why distinct: Incident Response Playbook owns a specific new mechanic and does not replace an existing domain.
- Successful output: An incident checklist.
### 02 · Product boundary
- Security is designed into identity, permissions, persistence and operations with narrow, explainable controls.
- Owner: Security, Abuse Prevention and Account Safety (`security`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `security_incident_playbook`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Contain, preserve safe evidence, notify scope and verify recovery.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- An operator playbook covers token leaks, duplicate trades, spoofing and data incidents.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses privacy and operational trust.
- Relationship type: `owned_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to fairness: checks outcomes and ledgers.
- Relationship type: `protects`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to multiplayer: validates membership and turns.
- Relationship type: `secures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to bazaar: guards escrow and transfers.
- Relationship type: `protects`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- An incident checklist.
- Every durable output has a stable ID, definition version, owner scope and source event reference.
- Public fields are separated from private data for Discord and Activity views.
- Consumers receive explicit events or read models, not direct access to other tables.
### 13 · Rust implementation
- Rust owns rules, state transitions, permissions, output validation and persistence.
- Core mechanics are pure or narrowly stateful functions testable without Discord.
- Poise and Serenity adapt commands, buttons, menus and modals; they do not own game logic.
- Tokio work is bounded and event-driven, without per-frame loops or unbounded task fan-out.
### 14 · State machine
- Lifecycle: Available → Prepared → Active → Completed, with Paused, Cancelled and Expired exits where relevant.
- Each accepted action advances a monotonic state version and records actor and transition.
- Duplicate actions return the canonical result; stale actions cannot rewind state.
- Migrations define how earlier versions upgrade or archive safely.
### 15 · Persistence
- Persist only durable facts required by this feature; do not store raw command text or message history.
- Use stable foreign keys, owner scopes, unique idempotency keys and constraints for one-time claims.
- Related balance, inventory and ledger writes share one short SQLite transaction.
- No database transaction is held across Discord HTTP, rendering or slow awaits.
### 16 · Privacy
- Public, party, guild, private and operational data are separate categories with distinct read policies.
- Document every stored field's purpose, retention, export and deletion route before release.
- No behavioural profile or unnecessary personal attribute is required by this design.
- Guild-owned records remain isolated unless a user explicitly shares a permitted result.
### 17 · Economy
- Moonbits remain earned-only, non-redeemable, non-purchasable and never staked.
- Prices, fees, inputs and rewards are disclosed before confirmation and deterministic for the content version.
- Randomness may support score-only entertainment under FAIRNESS.md, never tradable asset allocation.
- No gambling, wager, cash-out or paid chance mechanic can be derived from this feature.
### 18 · Discord surface
- Prefix and slash commands call the same Rust application service and expose legal next actions only.
- Messages, embeds, buttons, select menus and modals preserve a useful core flow without Activity.
- Private records and offers are delivered privately where supported; public boards show public state only.
- Deleted messages and expired interactions are recovered from durable state, never duplicated.
### 19 · Activity surface
- Next.js, React and TypeScript provide rich maps, boards, editors or service views where useful.
- Discord's official Embedded App SDK supplies Activity context; browser fallback may support compatible views.
- Frontend renders authorised view models and submits intent; it never decides trusted outcomes or ownership.
- Important state retains a Discord-native or text alternative and a clear asset-failure state.
### 20 · Accessibility
- Critical information uses labels and symbols as well as colour.
- Controls have explicit names and predictable focus order; mobile keeps the main action and exit visible.
- Reduced motion, readable density, contrast and text-first alternatives use the same rules.
- Instructions explain objective, legal actions and completion before entry.
### 21 · Moderation and safety
- User-authored material is length-limited, validated and reviewed for its visibility level.
- Only authorised moderators see moderation evidence; ordinary roles do not access private inventories.
- Content must be original or rights-cleared and age-appropriate.
- Reports and decisions are reasoned and auditable; popularity is not evidence.
### 22 · Healthy engagement
- Engagement comes from content quality, discovery, cooperation and player agency.
- No punitive streaks, forced attendance, fake scarcity, nagging reminders or hidden countdowns.
- Rotating content remains archived or evergreen; absence never destroys progress.
- Pause, leave, mute and notification opt-out are always available.
### 23 · Failure and tests
- Test duplicate clicks, stale interactions, reconnects, deleted messages, partial writes, rate limits and concurrent actions.
- Failed reservations release exactly once; successful retries return the original receipt.
- Unit tests cover legal actions, versions, deterministic outputs and edge cases.
- Integration tests cover SQLite rollback, restart recovery, export/deletion and guild isolation where relevant.
### 24 · Release gate
- Release as one genuine workflow with persistence, Discord interaction and Activity only where useful.
- Validate content references, accessibility, localisation and asset rights before publication.
- Use staged previews and a reversible release; preserve completed player state.
- Do not add microservices, Redis, another database or executable third-party plugins without an accepted architecture decision.

---
