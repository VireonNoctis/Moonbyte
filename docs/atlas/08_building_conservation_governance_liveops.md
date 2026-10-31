# Volume VIII — Building, Conservation, Governance and Live Operations

> Moonbyte World Loom · Expansion II · 40 additional proposals · Design proposal only.

<!-- WORLD_LOOM_GRAPH_DATA {"domains":[{"id":"building","title":"Architecture, Construction and World Design","summary":"Building tools let players design places from approved pieces and deterministic recipes, without arbitrary scripts.","links":[["homestead","extends","reuses personal layout controls"],["guild","builds_for","creates shared spaces"],["craft","uses_materials_from","consumes known inputs"],["atlas","registered_in","adds structures to the world map"]],"items":[["building_blueprint_library","Blueprint Library","Validated house, hall, bridge and workshop designs include bounds, inputs and supported functions.","Browse a blueprint, inspect footprint and recipe, preview and save a plan.","A versioned blueprint."],["building_grid_editor","World Grid Editor","A constrained editor places approved tiles and structures on a grid with collision feedback.","Choose a plot, place objects, inspect collisions, undo and save a draft.","A layout state."],["building_public_architecture","Public Architecture Projects","Guilds build public landmarks and utility buildings with documented functions.","Choose a site, blueprint and known materials, build in stages and publish.","A public building record."],["building_functional_rooms","Functional Room Contracts","Rooms reference existing services such as a library or workshop rather than duplicate rules.","Place a room marker, select its function, validate permissions and link its service.","A room configuration."],["building_palette_studio","Architect Palette Studio","A palette editor defines original pixel-art combinations with contrast and licence checks.","Choose a palette, test contrast, preview structures and save it.","A validated palette."],["building_structural_rules","Construction Validation Rules","Validation checks plot bounds, connections, accessible routes and material requirements.","Run checks, inspect errors, correct the layout and submit it.","A validation report."],["building_shared_plots","Guild Shared Plots","A guild designates shared plots with editor roles, proposals and rollback history.","Propose a change, review its diff, approve and publish.","A guild layout."],["building_building_history","Construction Chronicle","Public structure updates include snapshots, material summaries and contributor records.","Inspect revisions, compare snapshots and restore a version.","A construction history."],["building_wayfinding","Architectural Wayfinding","Buildings expose labelled entrances, room names and route anchors to Atlas navigation.","Add labels, connect room IDs, test and publish the overlay.","Validated room nodes."],["building_template_exchange","Blueprint Exchange","Creators publish rights-cleared templates with material requirements and compatibility.","Prepare a template, attach sources, validate, review and publish.","A moderated blueprint pack."]]},{"id":"conservation","title":"Resource Stewardship and Restoration","summary":"Stewardship turns fictional resource use into positive world-building. Resources remain available through repeatable routes.","links":[["climate","informed_by","uses regional observations"],["atlas","mapped_in","records restoration areas"],["festival","celebrated_by","recognises projects permanently"],["guild","organised_by","coordinates restoration teams"]],"items":[["conservation_habitat_restoration","Habitat Restoration Plans","A plan combines observations, planting recipes and puzzles into a visible regional improvement.","Inspect a site, review criteria, contribute an approved task and verify it.","A regional restoration state."],["conservation_resource_budget","Resource Stewardship Ledger","A budget records material use and shows recipes that reduce consumption.","Inspect a budget, compare recipes, choose a plan and record materials.","A project resource summary."],["conservation_reuse","Repair and Reuse Routes","Eligible items convert through known recipes instead of being discarded.","Inspect exact output, confirm conversion and preserve provenance.","A deterministic transformation."],["conservation_biodiversity","Biodiversity Observation Goals","Players record verified species and habitats to support a regional picture.","Choose an observation route, document a species and compare records.","A habitat coverage map."],["conservation_river_projects","Riverway Restoration","A project uses watershed puzzles and settlement coordination to reconnect fictional routes.","Solve flow constraints, coordinate work and validate the river route.","A restored map edge."],["conservation_garden_network","Community Garden Network","Players contribute seed recipes and layouts to shared decorative gardens.","Choose a garden, contribute a design, review it and publish.","A shared garden record."],["conservation_material_origins","Material Origin Labels","Craft inputs show a fictional region and source activity.","Inspect an input, follow its source and use it in an eligible recipe.","An origin label."],["conservation_project_audit","Restoration Audit Board","A board verifies whether public restoration milestones meet published criteria.","Inspect evidence, validate each criterion and publish an audit.","A completion record."],["conservation_learning_scenarios","Stewardship Simulations","Fictional scenarios ask players to balance habitat, route and settlement needs.","Review constraints, select a plan, simulate and compare trade-offs.","A deterministic scenario result."],["conservation_legacy_map","Restoration Legacy Map","Completed projects remain visible as permanent changes to the world.","View works, follow sources and contribute to an optional plan.","A restoration map layer."]]},{"id":"governance","title":"Civic Governance and Community Decisions","summary":"Governance provides transparent fictional town decisions and server administration. Votes are scoped and private ballots stay private.","links":[["server","configured_by","uses Discord administrator boundaries"],["guild","participates_through","lets members review projects"],["fairness","audited_by","records eligibility and tally rules"],["trust","protected_by","uses privacy and permissions"]],"items":[["governance_town_charter","Town Charter","A fictional charter defines proposals, public services, budgets and project approvals.","Read the charter, inspect version and follow its history.","A versioned charter."],["governance_proposal_board","Civic Proposal Board","Members submit proposals for town decorations, map updates or services using a template.","Draft scope and resource needs, preview visibility and submit.","A structured proposal."],["governance_deliberation","Deliberation Notes","A proposal page collects arguments, evidence and impact notes before a decision opens.","Read, add a note, cite world records and review summary.","A deliberation record."],["governance_ballots","Transparent Community Ballots","A ballot defines eligibility, options, close conditions and counting rules.","Check eligibility, submit one private choice and verify the aggregate result.","A ballot event and tally."],["governance_quorum","Decision Threshold Rules","Each decision defines whether it needs a simple count, quorum or review.","Read the threshold, check participation and resolve under the declared condition.","A decision rule."],["governance_appeals","Civic Decision Appeal","A participant may challenge procedural error using an evidence reference.","Select a decision, state the procedural issue and request review.","An appeal case."],["governance_budget","Public Project Budget","A budget displays materials, contributions, remaining requirements and fixed fees.","Inspect cost, contribute after preview and review ledger.","A public project budget."],["governance_moderator_roles","Civic Steward Roles","Moderators get narrow permissions without control of personal inventories.","Review permissions, assign a steward and audit their actions.","A least-privilege role."],["governance_rule_versions","Charter Revision History","Rule changes require a proposal, visible diff and migration policy.","Compare revisions, approve under charter and archive the prior version.","A revision history."],["governance_public_minutes","Public Decision Minutes","An archive records decisions and affected world objects without exposing ballots.","Inspect summary, follow project IDs and request correction.","A public-safe decision record."]]},{"id":"liveops","title":"Content Studio and Release Operations","summary":"A reviewed content pipeline keeps a large world coherent and reversible. Releases are evergreen, not scarcity traps.","links":[["creator","receives_content_from","takes creator drafts"],["trust","reviewed_by","applies safety and rights checks"],["arcade","publishes_to","makes games discoverable"],["quest","versions_story_for","keeps narrative dependencies aligned"]],"items":[["liveops_content_registry","Content Registry","A registry tracks stable IDs, authorship, versions, dependencies and review status.","Register a package, validate manifest and publish an approved version.","A content registry entry."],["liveops_editorial_calendar","Editorial Calendar","A calendar plans chapters, puzzle packs and themes with evergreen access.","Schedule a package, preview dependencies, publish and archive it.","A release plan."],["liveops_release_notes","Player-Facing Release Notes","Changes explain new content, rule changes, fixes and saved-state migrations.","Read a note, inspect changed systems and follow help links.","A versioned release note."],["liveops_hotfix_registry","Hotfix and Rollback Registry","A hotfix records affected versions, mitigation and rollback route.","Register, compatibility-check, deploy and verify or roll back.","A reversible deployment record."],["liveops_qa_matrix","Content Quality Matrix","A matrix tests solvability, permissions, accessibility, privacy, economy and localisation.","Run tests, inspect failures and require sign-off.","A QA report."],["liveops_dependency_graph","Content Dependency Graph","Packages declare references so broken links are caught before release.","Inspect dependencies, resolve missing IDs and validate compatibility.","A dependency graph."],["liveops_asset_pipeline","Pixel Asset Pipeline","Art passes dimension, palette, frame, contrast, compression and licence checks.","Record rights, run checks, preview at Discord size and publish.","A validated asset package."],["liveops_story_migrations","Story State Migrations","Migrations preserve completed choices and old checkpoints when stories change.","Define a deterministic mapping, run historical fixtures and publish.","A story migration map."],["liveops_preview_world","Staging World Preview","Reviewers play staged packages with test identities and isolated state.","Create a preview, test views, report issues, reset and approve.","A disposable preview record."],["liveops_archive_retention","Evergreen Content Archive","Retired themes and event packages remain available unless removal is legally or safety required.","Archive, retain compatible links and preserve completed progress.","An archive entry."]]}]} -->

Each proposal defines its mechanic, player loop, cross-domain connections, Rust ownership, persistence, privacy, Discord and Activity behaviour, accessibility, testing and release gates.

## Architecture, Construction and World Design (`building`)

Building tools let players design places from approved pieces and deterministic recipes, without arbitrary scripts.

**Domain connections:** homestead via extends — reuses personal layout controls; guild via builds_for — creates shared spaces; craft via uses_materials_from — consumes known inputs; atlas via registered_in — adds structures to the world map.

# building_blueprint_library — Blueprint Library
> Expansion II · Architecture, Construction and World Design · Proposal 1 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Validated house, hall, bridge and workshop designs include bounds, inputs and supported functions.
- Player need: Browse a blueprint, inspect footprint and recipe, preview and save a plan.
- Why distinct: Blueprint Library owns a specific new mechanic and does not replace an existing domain.
- Successful output: A versioned blueprint.
### 02 · Product boundary
- Building tools let players design places from approved pieces and deterministic recipes, without arbitrary scripts.
- Owner: Architecture, Construction and World Design (`building`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `building_blueprint_library`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Browse a blueprint, inspect footprint and recipe, preview and save a plan.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Validated house, hall, bridge and workshop designs include bounds, inputs and supported functions.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to homestead: reuses personal layout controls.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: creates shared spaces.
- Relationship type: `builds_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to craft: consumes known inputs.
- Relationship type: `uses_materials_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to atlas: adds structures to the world map.
- Relationship type: `registered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A versioned blueprint.
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

# building_grid_editor — World Grid Editor
> Expansion II · Architecture, Construction and World Design · Proposal 2 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A constrained editor places approved tiles and structures on a grid with collision feedback.
- Player need: Choose a plot, place objects, inspect collisions, undo and save a draft.
- Why distinct: World Grid Editor owns a specific new mechanic and does not replace an existing domain.
- Successful output: A layout state.
### 02 · Product boundary
- Building tools let players design places from approved pieces and deterministic recipes, without arbitrary scripts.
- Owner: Architecture, Construction and World Design (`building`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `building_grid_editor`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Choose a plot, place objects, inspect collisions, undo and save a draft.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A constrained editor places approved tiles and structures on a grid with collision feedback.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to homestead: reuses personal layout controls.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: creates shared spaces.
- Relationship type: `builds_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to craft: consumes known inputs.
- Relationship type: `uses_materials_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to atlas: adds structures to the world map.
- Relationship type: `registered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A layout state.
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

# building_public_architecture — Public Architecture Projects
> Expansion II · Architecture, Construction and World Design · Proposal 3 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Guilds build public landmarks and utility buildings with documented functions.
- Player need: Choose a site, blueprint and known materials, build in stages and publish.
- Why distinct: Public Architecture Projects owns a specific new mechanic and does not replace an existing domain.
- Successful output: A public building record.
### 02 · Product boundary
- Building tools let players design places from approved pieces and deterministic recipes, without arbitrary scripts.
- Owner: Architecture, Construction and World Design (`building`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `building_public_architecture`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Choose a site, blueprint and known materials, build in stages and publish.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Guilds build public landmarks and utility buildings with documented functions.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to homestead: reuses personal layout controls.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: creates shared spaces.
- Relationship type: `builds_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to craft: consumes known inputs.
- Relationship type: `uses_materials_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to atlas: adds structures to the world map.
- Relationship type: `registered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A public building record.
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

# building_functional_rooms — Functional Room Contracts
> Expansion II · Architecture, Construction and World Design · Proposal 4 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Rooms reference existing services such as a library or workshop rather than duplicate rules.
- Player need: Place a room marker, select its function, validate permissions and link its service.
- Why distinct: Functional Room Contracts owns a specific new mechanic and does not replace an existing domain.
- Successful output: A room configuration.
### 02 · Product boundary
- Building tools let players design places from approved pieces and deterministic recipes, without arbitrary scripts.
- Owner: Architecture, Construction and World Design (`building`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `building_functional_rooms`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Place a room marker, select its function, validate permissions and link its service.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Rooms reference existing services such as a library or workshop rather than duplicate rules.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to homestead: reuses personal layout controls.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: creates shared spaces.
- Relationship type: `builds_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to craft: consumes known inputs.
- Relationship type: `uses_materials_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to atlas: adds structures to the world map.
- Relationship type: `registered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A room configuration.
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

# building_palette_studio — Architect Palette Studio
> Expansion II · Architecture, Construction and World Design · Proposal 5 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A palette editor defines original pixel-art combinations with contrast and licence checks.
- Player need: Choose a palette, test contrast, preview structures and save it.
- Why distinct: Architect Palette Studio owns a specific new mechanic and does not replace an existing domain.
- Successful output: A validated palette.
### 02 · Product boundary
- Building tools let players design places from approved pieces and deterministic recipes, without arbitrary scripts.
- Owner: Architecture, Construction and World Design (`building`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `building_palette_studio`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Choose a palette, test contrast, preview structures and save it.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A palette editor defines original pixel-art combinations with contrast and licence checks.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to homestead: reuses personal layout controls.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: creates shared spaces.
- Relationship type: `builds_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to craft: consumes known inputs.
- Relationship type: `uses_materials_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to atlas: adds structures to the world map.
- Relationship type: `registered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A validated palette.
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

# building_structural_rules — Construction Validation Rules
> Expansion II · Architecture, Construction and World Design · Proposal 6 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Validation checks plot bounds, connections, accessible routes and material requirements.
- Player need: Run checks, inspect errors, correct the layout and submit it.
- Why distinct: Construction Validation Rules owns a specific new mechanic and does not replace an existing domain.
- Successful output: A validation report.
### 02 · Product boundary
- Building tools let players design places from approved pieces and deterministic recipes, without arbitrary scripts.
- Owner: Architecture, Construction and World Design (`building`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `building_structural_rules`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Run checks, inspect errors, correct the layout and submit it.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Validation checks plot bounds, connections, accessible routes and material requirements.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to homestead: reuses personal layout controls.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: creates shared spaces.
- Relationship type: `builds_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to craft: consumes known inputs.
- Relationship type: `uses_materials_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to atlas: adds structures to the world map.
- Relationship type: `registered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A validation report.
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

# building_shared_plots — Guild Shared Plots
> Expansion II · Architecture, Construction and World Design · Proposal 7 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A guild designates shared plots with editor roles, proposals and rollback history.
- Player need: Propose a change, review its diff, approve and publish.
- Why distinct: Guild Shared Plots owns a specific new mechanic and does not replace an existing domain.
- Successful output: A guild layout.
### 02 · Product boundary
- Building tools let players design places from approved pieces and deterministic recipes, without arbitrary scripts.
- Owner: Architecture, Construction and World Design (`building`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `building_shared_plots`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Propose a change, review its diff, approve and publish.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A guild designates shared plots with editor roles, proposals and rollback history.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to homestead: reuses personal layout controls.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: creates shared spaces.
- Relationship type: `builds_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to craft: consumes known inputs.
- Relationship type: `uses_materials_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to atlas: adds structures to the world map.
- Relationship type: `registered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A guild layout.
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

# building_building_history — Construction Chronicle
> Expansion II · Architecture, Construction and World Design · Proposal 8 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Public structure updates include snapshots, material summaries and contributor records.
- Player need: Inspect revisions, compare snapshots and restore a version.
- Why distinct: Construction Chronicle owns a specific new mechanic and does not replace an existing domain.
- Successful output: A construction history.
### 02 · Product boundary
- Building tools let players design places from approved pieces and deterministic recipes, without arbitrary scripts.
- Owner: Architecture, Construction and World Design (`building`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `building_building_history`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect revisions, compare snapshots and restore a version.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Public structure updates include snapshots, material summaries and contributor records.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to homestead: reuses personal layout controls.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: creates shared spaces.
- Relationship type: `builds_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to craft: consumes known inputs.
- Relationship type: `uses_materials_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to atlas: adds structures to the world map.
- Relationship type: `registered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A construction history.
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

# building_wayfinding — Architectural Wayfinding
> Expansion II · Architecture, Construction and World Design · Proposal 9 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Buildings expose labelled entrances, room names and route anchors to Atlas navigation.
- Player need: Add labels, connect room IDs, test and publish the overlay.
- Why distinct: Architectural Wayfinding owns a specific new mechanic and does not replace an existing domain.
- Successful output: Validated room nodes.
### 02 · Product boundary
- Building tools let players design places from approved pieces and deterministic recipes, without arbitrary scripts.
- Owner: Architecture, Construction and World Design (`building`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `building_wayfinding`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Add labels, connect room IDs, test and publish the overlay.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Buildings expose labelled entrances, room names and route anchors to Atlas navigation.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to homestead: reuses personal layout controls.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: creates shared spaces.
- Relationship type: `builds_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to craft: consumes known inputs.
- Relationship type: `uses_materials_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to atlas: adds structures to the world map.
- Relationship type: `registered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- Validated room nodes.
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

# building_template_exchange — Blueprint Exchange
> Expansion II · Architecture, Construction and World Design · Proposal 10 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Creators publish rights-cleared templates with material requirements and compatibility.
- Player need: Prepare a template, attach sources, validate, review and publish.
- Why distinct: Blueprint Exchange owns a specific new mechanic and does not replace an existing domain.
- Successful output: A moderated blueprint pack.
### 02 · Product boundary
- Building tools let players design places from approved pieces and deterministic recipes, without arbitrary scripts.
- Owner: Architecture, Construction and World Design (`building`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `building_template_exchange`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Prepare a template, attach sources, validate, review and publish.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Creators publish rights-cleared templates with material requirements and compatibility.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to homestead: reuses personal layout controls.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: creates shared spaces.
- Relationship type: `builds_for`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to craft: consumes known inputs.
- Relationship type: `uses_materials_from`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to atlas: adds structures to the world map.
- Relationship type: `registered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A moderated blueprint pack.
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
## Resource Stewardship and Restoration (`conservation`)

Stewardship turns fictional resource use into positive world-building. Resources remain available through repeatable routes.

**Domain connections:** climate via informed_by — uses regional observations; atlas via mapped_in — records restoration areas; festival via celebrated_by — recognises projects permanently; guild via organised_by — coordinates restoration teams.

# conservation_habitat_restoration — Habitat Restoration Plans
> Expansion II · Resource Stewardship and Restoration · Proposal 1 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A plan combines observations, planting recipes and puzzles into a visible regional improvement.
- Player need: Inspect a site, review criteria, contribute an approved task and verify it.
- Why distinct: Habitat Restoration Plans owns a specific new mechanic and does not replace an existing domain.
- Successful output: A regional restoration state.
### 02 · Product boundary
- Stewardship turns fictional resource use into positive world-building. Resources remain available through repeatable routes.
- Owner: Resource Stewardship and Restoration (`conservation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `conservation_habitat_restoration`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect a site, review criteria, contribute an approved task and verify it.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A plan combines observations, planting recipes and puzzles into a visible regional improvement.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to climate: uses regional observations.
- Relationship type: `informed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to atlas: records restoration areas.
- Relationship type: `mapped_in`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to festival: recognises projects permanently.
- Relationship type: `celebrated_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to guild: coordinates restoration teams.
- Relationship type: `organised_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A regional restoration state.
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

# conservation_resource_budget — Resource Stewardship Ledger
> Expansion II · Resource Stewardship and Restoration · Proposal 2 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A budget records material use and shows recipes that reduce consumption.
- Player need: Inspect a budget, compare recipes, choose a plan and record materials.
- Why distinct: Resource Stewardship Ledger owns a specific new mechanic and does not replace an existing domain.
- Successful output: A project resource summary.
### 02 · Product boundary
- Stewardship turns fictional resource use into positive world-building. Resources remain available through repeatable routes.
- Owner: Resource Stewardship and Restoration (`conservation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `conservation_resource_budget`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect a budget, compare recipes, choose a plan and record materials.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A budget records material use and shows recipes that reduce consumption.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to climate: uses regional observations.
- Relationship type: `informed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to atlas: records restoration areas.
- Relationship type: `mapped_in`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to festival: recognises projects permanently.
- Relationship type: `celebrated_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to guild: coordinates restoration teams.
- Relationship type: `organised_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A project resource summary.
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

# conservation_reuse — Repair and Reuse Routes
> Expansion II · Resource Stewardship and Restoration · Proposal 3 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Eligible items convert through known recipes instead of being discarded.
- Player need: Inspect exact output, confirm conversion and preserve provenance.
- Why distinct: Repair and Reuse Routes owns a specific new mechanic and does not replace an existing domain.
- Successful output: A deterministic transformation.
### 02 · Product boundary
- Stewardship turns fictional resource use into positive world-building. Resources remain available through repeatable routes.
- Owner: Resource Stewardship and Restoration (`conservation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `conservation_reuse`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect exact output, confirm conversion and preserve provenance.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Eligible items convert through known recipes instead of being discarded.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to climate: uses regional observations.
- Relationship type: `informed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to atlas: records restoration areas.
- Relationship type: `mapped_in`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to festival: recognises projects permanently.
- Relationship type: `celebrated_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to guild: coordinates restoration teams.
- Relationship type: `organised_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A deterministic transformation.
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

# conservation_biodiversity — Biodiversity Observation Goals
> Expansion II · Resource Stewardship and Restoration · Proposal 4 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Players record verified species and habitats to support a regional picture.
- Player need: Choose an observation route, document a species and compare records.
- Why distinct: Biodiversity Observation Goals owns a specific new mechanic and does not replace an existing domain.
- Successful output: A habitat coverage map.
### 02 · Product boundary
- Stewardship turns fictional resource use into positive world-building. Resources remain available through repeatable routes.
- Owner: Resource Stewardship and Restoration (`conservation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `conservation_biodiversity`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Choose an observation route, document a species and compare records.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Players record verified species and habitats to support a regional picture.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to climate: uses regional observations.
- Relationship type: `informed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to atlas: records restoration areas.
- Relationship type: `mapped_in`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to festival: recognises projects permanently.
- Relationship type: `celebrated_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to guild: coordinates restoration teams.
- Relationship type: `organised_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A habitat coverage map.
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

# conservation_river_projects — Riverway Restoration
> Expansion II · Resource Stewardship and Restoration · Proposal 5 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A project uses watershed puzzles and settlement coordination to reconnect fictional routes.
- Player need: Solve flow constraints, coordinate work and validate the river route.
- Why distinct: Riverway Restoration owns a specific new mechanic and does not replace an existing domain.
- Successful output: A restored map edge.
### 02 · Product boundary
- Stewardship turns fictional resource use into positive world-building. Resources remain available through repeatable routes.
- Owner: Resource Stewardship and Restoration (`conservation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `conservation_river_projects`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Solve flow constraints, coordinate work and validate the river route.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A project uses watershed puzzles and settlement coordination to reconnect fictional routes.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to climate: uses regional observations.
- Relationship type: `informed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to atlas: records restoration areas.
- Relationship type: `mapped_in`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to festival: recognises projects permanently.
- Relationship type: `celebrated_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to guild: coordinates restoration teams.
- Relationship type: `organised_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A restored map edge.
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

# conservation_garden_network — Community Garden Network
> Expansion II · Resource Stewardship and Restoration · Proposal 6 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Players contribute seed recipes and layouts to shared decorative gardens.
- Player need: Choose a garden, contribute a design, review it and publish.
- Why distinct: Community Garden Network owns a specific new mechanic and does not replace an existing domain.
- Successful output: A shared garden record.
### 02 · Product boundary
- Stewardship turns fictional resource use into positive world-building. Resources remain available through repeatable routes.
- Owner: Resource Stewardship and Restoration (`conservation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `conservation_garden_network`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Choose a garden, contribute a design, review it and publish.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Players contribute seed recipes and layouts to shared decorative gardens.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to climate: uses regional observations.
- Relationship type: `informed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to atlas: records restoration areas.
- Relationship type: `mapped_in`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to festival: recognises projects permanently.
- Relationship type: `celebrated_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to guild: coordinates restoration teams.
- Relationship type: `organised_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A shared garden record.
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

# conservation_material_origins — Material Origin Labels
> Expansion II · Resource Stewardship and Restoration · Proposal 7 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Craft inputs show a fictional region and source activity.
- Player need: Inspect an input, follow its source and use it in an eligible recipe.
- Why distinct: Material Origin Labels owns a specific new mechanic and does not replace an existing domain.
- Successful output: An origin label.
### 02 · Product boundary
- Stewardship turns fictional resource use into positive world-building. Resources remain available through repeatable routes.
- Owner: Resource Stewardship and Restoration (`conservation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `conservation_material_origins`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect an input, follow its source and use it in an eligible recipe.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Craft inputs show a fictional region and source activity.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to climate: uses regional observations.
- Relationship type: `informed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to atlas: records restoration areas.
- Relationship type: `mapped_in`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to festival: recognises projects permanently.
- Relationship type: `celebrated_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to guild: coordinates restoration teams.
- Relationship type: `organised_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- An origin label.
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

# conservation_project_audit — Restoration Audit Board
> Expansion II · Resource Stewardship and Restoration · Proposal 8 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A board verifies whether public restoration milestones meet published criteria.
- Player need: Inspect evidence, validate each criterion and publish an audit.
- Why distinct: Restoration Audit Board owns a specific new mechanic and does not replace an existing domain.
- Successful output: A completion record.
### 02 · Product boundary
- Stewardship turns fictional resource use into positive world-building. Resources remain available through repeatable routes.
- Owner: Resource Stewardship and Restoration (`conservation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `conservation_project_audit`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect evidence, validate each criterion and publish an audit.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A board verifies whether public restoration milestones meet published criteria.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to climate: uses regional observations.
- Relationship type: `informed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to atlas: records restoration areas.
- Relationship type: `mapped_in`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to festival: recognises projects permanently.
- Relationship type: `celebrated_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to guild: coordinates restoration teams.
- Relationship type: `organised_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A completion record.
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

# conservation_learning_scenarios — Stewardship Simulations
> Expansion II · Resource Stewardship and Restoration · Proposal 9 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Fictional scenarios ask players to balance habitat, route and settlement needs.
- Player need: Review constraints, select a plan, simulate and compare trade-offs.
- Why distinct: Stewardship Simulations owns a specific new mechanic and does not replace an existing domain.
- Successful output: A deterministic scenario result.
### 02 · Product boundary
- Stewardship turns fictional resource use into positive world-building. Resources remain available through repeatable routes.
- Owner: Resource Stewardship and Restoration (`conservation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `conservation_learning_scenarios`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Review constraints, select a plan, simulate and compare trade-offs.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Fictional scenarios ask players to balance habitat, route and settlement needs.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to climate: uses regional observations.
- Relationship type: `informed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to atlas: records restoration areas.
- Relationship type: `mapped_in`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to festival: recognises projects permanently.
- Relationship type: `celebrated_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to guild: coordinates restoration teams.
- Relationship type: `organised_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A deterministic scenario result.
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

# conservation_legacy_map — Restoration Legacy Map
> Expansion II · Resource Stewardship and Restoration · Proposal 10 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Completed projects remain visible as permanent changes to the world.
- Player need: View works, follow sources and contribute to an optional plan.
- Why distinct: Restoration Legacy Map owns a specific new mechanic and does not replace an existing domain.
- Successful output: A restoration map layer.
### 02 · Product boundary
- Stewardship turns fictional resource use into positive world-building. Resources remain available through repeatable routes.
- Owner: Resource Stewardship and Restoration (`conservation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `conservation_legacy_map`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- View works, follow sources and contribute to an optional plan.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Completed projects remain visible as permanent changes to the world.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to climate: uses regional observations.
- Relationship type: `informed_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to atlas: records restoration areas.
- Relationship type: `mapped_in`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to festival: recognises projects permanently.
- Relationship type: `celebrated_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to guild: coordinates restoration teams.
- Relationship type: `organised_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A restoration map layer.
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
## Civic Governance and Community Decisions (`governance`)

Governance provides transparent fictional town decisions and server administration. Votes are scoped and private ballots stay private.

**Domain connections:** server via configured_by — uses Discord administrator boundaries; guild via participates_through — lets members review projects; fairness via audited_by — records eligibility and tally rules; trust via protected_by — uses privacy and permissions.

# governance_town_charter — Town Charter
> Expansion II · Civic Governance and Community Decisions · Proposal 1 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A fictional charter defines proposals, public services, budgets and project approvals.
- Player need: Read the charter, inspect version and follow its history.
- Why distinct: Town Charter owns a specific new mechanic and does not replace an existing domain.
- Successful output: A versioned charter.
### 02 · Product boundary
- Governance provides transparent fictional town decisions and server administration. Votes are scoped and private ballots stay private.
- Owner: Civic Governance and Community Decisions (`governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `governance_town_charter`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Read the charter, inspect version and follow its history.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A fictional charter defines proposals, public services, budgets and project approvals.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to server: uses Discord administrator boundaries.
- Relationship type: `configured_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: lets members review projects.
- Relationship type: `participates_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: records eligibility and tally rules.
- Relationship type: `audited_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: uses privacy and permissions.
- Relationship type: `protected_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A versioned charter.
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

# governance_proposal_board — Civic Proposal Board
> Expansion II · Civic Governance and Community Decisions · Proposal 2 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Members submit proposals for town decorations, map updates or services using a template.
- Player need: Draft scope and resource needs, preview visibility and submit.
- Why distinct: Civic Proposal Board owns a specific new mechanic and does not replace an existing domain.
- Successful output: A structured proposal.
### 02 · Product boundary
- Governance provides transparent fictional town decisions and server administration. Votes are scoped and private ballots stay private.
- Owner: Civic Governance and Community Decisions (`governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `governance_proposal_board`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Draft scope and resource needs, preview visibility and submit.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Members submit proposals for town decorations, map updates or services using a template.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to server: uses Discord administrator boundaries.
- Relationship type: `configured_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: lets members review projects.
- Relationship type: `participates_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: records eligibility and tally rules.
- Relationship type: `audited_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: uses privacy and permissions.
- Relationship type: `protected_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A structured proposal.
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

# governance_deliberation — Deliberation Notes
> Expansion II · Civic Governance and Community Decisions · Proposal 3 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A proposal page collects arguments, evidence and impact notes before a decision opens.
- Player need: Read, add a note, cite world records and review summary.
- Why distinct: Deliberation Notes owns a specific new mechanic and does not replace an existing domain.
- Successful output: A deliberation record.
### 02 · Product boundary
- Governance provides transparent fictional town decisions and server administration. Votes are scoped and private ballots stay private.
- Owner: Civic Governance and Community Decisions (`governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `governance_deliberation`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Read, add a note, cite world records and review summary.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A proposal page collects arguments, evidence and impact notes before a decision opens.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to server: uses Discord administrator boundaries.
- Relationship type: `configured_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: lets members review projects.
- Relationship type: `participates_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: records eligibility and tally rules.
- Relationship type: `audited_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: uses privacy and permissions.
- Relationship type: `protected_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A deliberation record.
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

# governance_ballots — Transparent Community Ballots
> Expansion II · Civic Governance and Community Decisions · Proposal 4 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A ballot defines eligibility, options, close conditions and counting rules.
- Player need: Check eligibility, submit one private choice and verify the aggregate result.
- Why distinct: Transparent Community Ballots owns a specific new mechanic and does not replace an existing domain.
- Successful output: A ballot event and tally.
### 02 · Product boundary
- Governance provides transparent fictional town decisions and server administration. Votes are scoped and private ballots stay private.
- Owner: Civic Governance and Community Decisions (`governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `governance_ballots`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Check eligibility, submit one private choice and verify the aggregate result.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A ballot defines eligibility, options, close conditions and counting rules.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to server: uses Discord administrator boundaries.
- Relationship type: `configured_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: lets members review projects.
- Relationship type: `participates_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: records eligibility and tally rules.
- Relationship type: `audited_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: uses privacy and permissions.
- Relationship type: `protected_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A ballot event and tally.
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

# governance_quorum — Decision Threshold Rules
> Expansion II · Civic Governance and Community Decisions · Proposal 5 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Each decision defines whether it needs a simple count, quorum or review.
- Player need: Read the threshold, check participation and resolve under the declared condition.
- Why distinct: Decision Threshold Rules owns a specific new mechanic and does not replace an existing domain.
- Successful output: A decision rule.
### 02 · Product boundary
- Governance provides transparent fictional town decisions and server administration. Votes are scoped and private ballots stay private.
- Owner: Civic Governance and Community Decisions (`governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `governance_quorum`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Read the threshold, check participation and resolve under the declared condition.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Each decision defines whether it needs a simple count, quorum or review.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to server: uses Discord administrator boundaries.
- Relationship type: `configured_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: lets members review projects.
- Relationship type: `participates_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: records eligibility and tally rules.
- Relationship type: `audited_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: uses privacy and permissions.
- Relationship type: `protected_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A decision rule.
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

# governance_appeals — Civic Decision Appeal
> Expansion II · Civic Governance and Community Decisions · Proposal 6 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A participant may challenge procedural error using an evidence reference.
- Player need: Select a decision, state the procedural issue and request review.
- Why distinct: Civic Decision Appeal owns a specific new mechanic and does not replace an existing domain.
- Successful output: An appeal case.
### 02 · Product boundary
- Governance provides transparent fictional town decisions and server administration. Votes are scoped and private ballots stay private.
- Owner: Civic Governance and Community Decisions (`governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `governance_appeals`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Select a decision, state the procedural issue and request review.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A participant may challenge procedural error using an evidence reference.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to server: uses Discord administrator boundaries.
- Relationship type: `configured_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: lets members review projects.
- Relationship type: `participates_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: records eligibility and tally rules.
- Relationship type: `audited_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: uses privacy and permissions.
- Relationship type: `protected_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- An appeal case.
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

# governance_budget — Public Project Budget
> Expansion II · Civic Governance and Community Decisions · Proposal 7 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A budget displays materials, contributions, remaining requirements and fixed fees.
- Player need: Inspect cost, contribute after preview and review ledger.
- Why distinct: Public Project Budget owns a specific new mechanic and does not replace an existing domain.
- Successful output: A public project budget.
### 02 · Product boundary
- Governance provides transparent fictional town decisions and server administration. Votes are scoped and private ballots stay private.
- Owner: Civic Governance and Community Decisions (`governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `governance_budget`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect cost, contribute after preview and review ledger.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A budget displays materials, contributions, remaining requirements and fixed fees.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to server: uses Discord administrator boundaries.
- Relationship type: `configured_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: lets members review projects.
- Relationship type: `participates_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: records eligibility and tally rules.
- Relationship type: `audited_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: uses privacy and permissions.
- Relationship type: `protected_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A public project budget.
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

# governance_moderator_roles — Civic Steward Roles
> Expansion II · Civic Governance and Community Decisions · Proposal 8 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Moderators get narrow permissions without control of personal inventories.
- Player need: Review permissions, assign a steward and audit their actions.
- Why distinct: Civic Steward Roles owns a specific new mechanic and does not replace an existing domain.
- Successful output: A least-privilege role.
### 02 · Product boundary
- Governance provides transparent fictional town decisions and server administration. Votes are scoped and private ballots stay private.
- Owner: Civic Governance and Community Decisions (`governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `governance_moderator_roles`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Review permissions, assign a steward and audit their actions.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Moderators get narrow permissions without control of personal inventories.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to server: uses Discord administrator boundaries.
- Relationship type: `configured_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: lets members review projects.
- Relationship type: `participates_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: records eligibility and tally rules.
- Relationship type: `audited_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: uses privacy and permissions.
- Relationship type: `protected_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A least-privilege role.
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

# governance_rule_versions — Charter Revision History
> Expansion II · Civic Governance and Community Decisions · Proposal 9 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Rule changes require a proposal, visible diff and migration policy.
- Player need: Compare revisions, approve under charter and archive the prior version.
- Why distinct: Charter Revision History owns a specific new mechanic and does not replace an existing domain.
- Successful output: A revision history.
### 02 · Product boundary
- Governance provides transparent fictional town decisions and server administration. Votes are scoped and private ballots stay private.
- Owner: Civic Governance and Community Decisions (`governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `governance_rule_versions`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Compare revisions, approve under charter and archive the prior version.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Rule changes require a proposal, visible diff and migration policy.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to server: uses Discord administrator boundaries.
- Relationship type: `configured_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: lets members review projects.
- Relationship type: `participates_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: records eligibility and tally rules.
- Relationship type: `audited_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: uses privacy and permissions.
- Relationship type: `protected_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A revision history.
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

# governance_public_minutes — Public Decision Minutes
> Expansion II · Civic Governance and Community Decisions · Proposal 10 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- An archive records decisions and affected world objects without exposing ballots.
- Player need: Inspect summary, follow project IDs and request correction.
- Why distinct: Public Decision Minutes owns a specific new mechanic and does not replace an existing domain.
- Successful output: A public-safe decision record.
### 02 · Product boundary
- Governance provides transparent fictional town decisions and server administration. Votes are scoped and private ballots stay private.
- Owner: Civic Governance and Community Decisions (`governance`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `governance_public_minutes`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect summary, follow project IDs and request correction.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- An archive records decisions and affected world objects without exposing ballots.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to server: uses Discord administrator boundaries.
- Relationship type: `configured_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to guild: lets members review projects.
- Relationship type: `participates_through`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: records eligibility and tally rules.
- Relationship type: `audited_by`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: uses privacy and permissions.
- Relationship type: `protected_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A public-safe decision record.
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
## Content Studio and Release Operations (`liveops`)

A reviewed content pipeline keeps a large world coherent and reversible. Releases are evergreen, not scarcity traps.

**Domain connections:** creator via receives_content_from — takes creator drafts; trust via reviewed_by — applies safety and rights checks; arcade via publishes_to — makes games discoverable; quest via versions_story_for — keeps narrative dependencies aligned.

# liveops_content_registry — Content Registry
> Expansion II · Content Studio and Release Operations · Proposal 1 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A registry tracks stable IDs, authorship, versions, dependencies and review status.
- Player need: Register a package, validate manifest and publish an approved version.
- Why distinct: Content Registry owns a specific new mechanic and does not replace an existing domain.
- Successful output: A content registry entry.
### 02 · Product boundary
- A reviewed content pipeline keeps a large world coherent and reversible. Releases are evergreen, not scarcity traps.
- Owner: Content Studio and Release Operations (`liveops`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `liveops_content_registry`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Register a package, validate manifest and publish an approved version.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A registry tracks stable IDs, authorship, versions, dependencies and review status.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to creator: takes creator drafts.
- Relationship type: `receives_content_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to trust: applies safety and rights checks.
- Relationship type: `reviewed_by`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to arcade: makes games discoverable.
- Relationship type: `publishes_to`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to quest: keeps narrative dependencies aligned.
- Relationship type: `versions_story_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A content registry entry.
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

# liveops_editorial_calendar — Editorial Calendar
> Expansion II · Content Studio and Release Operations · Proposal 2 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A calendar plans chapters, puzzle packs and themes with evergreen access.
- Player need: Schedule a package, preview dependencies, publish and archive it.
- Why distinct: Editorial Calendar owns a specific new mechanic and does not replace an existing domain.
- Successful output: A release plan.
### 02 · Product boundary
- A reviewed content pipeline keeps a large world coherent and reversible. Releases are evergreen, not scarcity traps.
- Owner: Content Studio and Release Operations (`liveops`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `liveops_editorial_calendar`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Schedule a package, preview dependencies, publish and archive it.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A calendar plans chapters, puzzle packs and themes with evergreen access.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to creator: takes creator drafts.
- Relationship type: `receives_content_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to trust: applies safety and rights checks.
- Relationship type: `reviewed_by`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to arcade: makes games discoverable.
- Relationship type: `publishes_to`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to quest: keeps narrative dependencies aligned.
- Relationship type: `versions_story_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A release plan.
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

# liveops_release_notes — Player-Facing Release Notes
> Expansion II · Content Studio and Release Operations · Proposal 3 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Changes explain new content, rule changes, fixes and saved-state migrations.
- Player need: Read a note, inspect changed systems and follow help links.
- Why distinct: Player-Facing Release Notes owns a specific new mechanic and does not replace an existing domain.
- Successful output: A versioned release note.
### 02 · Product boundary
- A reviewed content pipeline keeps a large world coherent and reversible. Releases are evergreen, not scarcity traps.
- Owner: Content Studio and Release Operations (`liveops`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `liveops_release_notes`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Read a note, inspect changed systems and follow help links.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Changes explain new content, rule changes, fixes and saved-state migrations.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to creator: takes creator drafts.
- Relationship type: `receives_content_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to trust: applies safety and rights checks.
- Relationship type: `reviewed_by`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to arcade: makes games discoverable.
- Relationship type: `publishes_to`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to quest: keeps narrative dependencies aligned.
- Relationship type: `versions_story_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A versioned release note.
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

# liveops_hotfix_registry — Hotfix and Rollback Registry
> Expansion II · Content Studio and Release Operations · Proposal 4 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A hotfix records affected versions, mitigation and rollback route.
- Player need: Register, compatibility-check, deploy and verify or roll back.
- Why distinct: Hotfix and Rollback Registry owns a specific new mechanic and does not replace an existing domain.
- Successful output: A reversible deployment record.
### 02 · Product boundary
- A reviewed content pipeline keeps a large world coherent and reversible. Releases are evergreen, not scarcity traps.
- Owner: Content Studio and Release Operations (`liveops`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `liveops_hotfix_registry`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Register, compatibility-check, deploy and verify or roll back.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A hotfix records affected versions, mitigation and rollback route.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to creator: takes creator drafts.
- Relationship type: `receives_content_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to trust: applies safety and rights checks.
- Relationship type: `reviewed_by`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to arcade: makes games discoverable.
- Relationship type: `publishes_to`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to quest: keeps narrative dependencies aligned.
- Relationship type: `versions_story_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A reversible deployment record.
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

# liveops_qa_matrix — Content Quality Matrix
> Expansion II · Content Studio and Release Operations · Proposal 5 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A matrix tests solvability, permissions, accessibility, privacy, economy and localisation.
- Player need: Run tests, inspect failures and require sign-off.
- Why distinct: Content Quality Matrix owns a specific new mechanic and does not replace an existing domain.
- Successful output: A QA report.
### 02 · Product boundary
- A reviewed content pipeline keeps a large world coherent and reversible. Releases are evergreen, not scarcity traps.
- Owner: Content Studio and Release Operations (`liveops`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `liveops_qa_matrix`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Run tests, inspect failures and require sign-off.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A matrix tests solvability, permissions, accessibility, privacy, economy and localisation.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to creator: takes creator drafts.
- Relationship type: `receives_content_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to trust: applies safety and rights checks.
- Relationship type: `reviewed_by`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to arcade: makes games discoverable.
- Relationship type: `publishes_to`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to quest: keeps narrative dependencies aligned.
- Relationship type: `versions_story_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A QA report.
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

# liveops_dependency_graph — Content Dependency Graph
> Expansion II · Content Studio and Release Operations · Proposal 6 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Packages declare references so broken links are caught before release.
- Player need: Inspect dependencies, resolve missing IDs and validate compatibility.
- Why distinct: Content Dependency Graph owns a specific new mechanic and does not replace an existing domain.
- Successful output: A dependency graph.
### 02 · Product boundary
- A reviewed content pipeline keeps a large world coherent and reversible. Releases are evergreen, not scarcity traps.
- Owner: Content Studio and Release Operations (`liveops`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `liveops_dependency_graph`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect dependencies, resolve missing IDs and validate compatibility.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Packages declare references so broken links are caught before release.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to creator: takes creator drafts.
- Relationship type: `receives_content_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to trust: applies safety and rights checks.
- Relationship type: `reviewed_by`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to arcade: makes games discoverable.
- Relationship type: `publishes_to`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to quest: keeps narrative dependencies aligned.
- Relationship type: `versions_story_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A dependency graph.
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

# liveops_asset_pipeline — Pixel Asset Pipeline
> Expansion II · Content Studio and Release Operations · Proposal 7 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Art passes dimension, palette, frame, contrast, compression and licence checks.
- Player need: Record rights, run checks, preview at Discord size and publish.
- Why distinct: Pixel Asset Pipeline owns a specific new mechanic and does not replace an existing domain.
- Successful output: A validated asset package.
### 02 · Product boundary
- A reviewed content pipeline keeps a large world coherent and reversible. Releases are evergreen, not scarcity traps.
- Owner: Content Studio and Release Operations (`liveops`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `liveops_asset_pipeline`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Record rights, run checks, preview at Discord size and publish.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Art passes dimension, palette, frame, contrast, compression and licence checks.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to creator: takes creator drafts.
- Relationship type: `receives_content_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to trust: applies safety and rights checks.
- Relationship type: `reviewed_by`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to arcade: makes games discoverable.
- Relationship type: `publishes_to`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to quest: keeps narrative dependencies aligned.
- Relationship type: `versions_story_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A validated asset package.
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

# liveops_story_migrations — Story State Migrations
> Expansion II · Content Studio and Release Operations · Proposal 8 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Migrations preserve completed choices and old checkpoints when stories change.
- Player need: Define a deterministic mapping, run historical fixtures and publish.
- Why distinct: Story State Migrations owns a specific new mechanic and does not replace an existing domain.
- Successful output: A story migration map.
### 02 · Product boundary
- A reviewed content pipeline keeps a large world coherent and reversible. Releases are evergreen, not scarcity traps.
- Owner: Content Studio and Release Operations (`liveops`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `liveops_story_migrations`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Define a deterministic mapping, run historical fixtures and publish.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Migrations preserve completed choices and old checkpoints when stories change.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to creator: takes creator drafts.
- Relationship type: `receives_content_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to trust: applies safety and rights checks.
- Relationship type: `reviewed_by`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to arcade: makes games discoverable.
- Relationship type: `publishes_to`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to quest: keeps narrative dependencies aligned.
- Relationship type: `versions_story_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A story migration map.
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

# liveops_preview_world — Staging World Preview
> Expansion II · Content Studio and Release Operations · Proposal 9 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Reviewers play staged packages with test identities and isolated state.
- Player need: Create a preview, test views, report issues, reset and approve.
- Why distinct: Staging World Preview owns a specific new mechanic and does not replace an existing domain.
- Successful output: A disposable preview record.
### 02 · Product boundary
- A reviewed content pipeline keeps a large world coherent and reversible. Releases are evergreen, not scarcity traps.
- Owner: Content Studio and Release Operations (`liveops`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `liveops_preview_world`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Create a preview, test views, report issues, reset and approve.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Reviewers play staged packages with test identities and isolated state.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to creator: takes creator drafts.
- Relationship type: `receives_content_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to trust: applies safety and rights checks.
- Relationship type: `reviewed_by`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to arcade: makes games discoverable.
- Relationship type: `publishes_to`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to quest: keeps narrative dependencies aligned.
- Relationship type: `versions_story_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A disposable preview record.
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

# liveops_archive_retention — Evergreen Content Archive
> Expansion II · Content Studio and Release Operations · Proposal 10 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Retired themes and event packages remain available unless removal is legally or safety required.
- Player need: Archive, retain compatible links and preserve completed progress.
- Why distinct: Evergreen Content Archive owns a specific new mechanic and does not replace an existing domain.
- Successful output: An archive entry.
### 02 · Product boundary
- A reviewed content pipeline keeps a large world coherent and reversible. Releases are evergreen, not scarcity traps.
- Owner: Content Studio and Release Operations (`liveops`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `liveops_archive_retention`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Archive, retain compatible links and preserve completed progress.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Retired themes and event packages remain available unless removal is legally or safety required.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to creator: takes creator drafts.
- Relationship type: `receives_content_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to trust: applies safety and rights checks.
- Relationship type: `reviewed_by`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to arcade: makes games discoverable.
- Relationship type: `publishes_to`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to quest: keeps narrative dependencies aligned.
- Relationship type: `versions_story_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- An archive entry.
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
