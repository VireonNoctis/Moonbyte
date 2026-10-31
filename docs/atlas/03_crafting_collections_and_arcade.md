# Volume III — Crafting, Collections and Arcade

> Moonbyte World Loom · 40 connected system proposals · This is a design specification, not a claim of implementation.

Each card is a distinct proposal with a stable graph identity, a player loop, state and ownership boundaries, Discord and Activity surfaces, privacy rules, acceptance tests and explicit cross-system links.

## Domain 9 — Gathering, Crafting and Workshops

Crafting transforms known inputs into predictable outputs and links exploration, Bazaar demand, homes and collections. It is not a random loot system.

**Domain dependencies:** The Bazaar and Exchange (`bazaar`) · Collections, Bestiary and Museum (`archive`) · Homes, Gardens and Personal Spaces (`homestead`) · Expeditions and Exploration Sessions (`expedition`)

# craft_gathering — Field Gathering
> Node 1 of 10 in **Gathering, Crafting and Workshops** · World Loom identity `craft_gathering` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players discover fixed resource nodes through map exploration, puzzle completion and authored route objectives.
- Why this belongs in Moonbyte: It gives resources a clear origin and ties crafting to world knowledge rather than idle grinding.
- Distinctive interaction: Visit a node, inspect its known requirements, complete the associated activity and claim its disclosed yield.
- Successful completion produces: A provenance-marked resource batch with a known source.
### 02 · Purpose and boundaries
- Product role: Crafting transforms known inputs into predictable outputs and links exploration, Bazaar demand, homes and collections. It is not a random loot system.
- This system is the canonical owner of field gathering behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Gathering, Crafting and Workshops; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives resources a clear origin and ties crafting to world knowledge rather than idle grinding.
- Stable destination ID: craft_gathering; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .craft gathering and /craft gathering are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Visit a node and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to field gathering.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players discover fixed resource nodes through map exploration, puzzle completion and authored route objectives.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Visit a node, inspect its known requirements, complete the associated activity and claim its disclosed yield.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Field Gathering is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A provenance-marked resource batch with a known source.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- fulfils wanted-board requests and work orders through known recipe outputs: Player Stallfronts (bazaar_stallfronts).
- uses stable item definitions, provenance and collection membership: The Illustrated Compendium (archive_cards).
- turns crafted objects into useful home decorations and workshop upgrades: Decorative Collections (homestead_decor_sets).
- creates disclosed route kits and tools before an expedition begins: Expedition Route Archive (expedition_replay).
### 12 · Incoming system relationships
- Player Stallfronts (bazaar_stallfronts) can use this system because it connects player demand to published recipes, ingredient lists and crafting work orders.
- Logic Key Mechanisms (vault_logic_keys) can use this system because it connects known recipes and crafted tools to specific puzzle mechanics.
- The Museum Hall (archive_museum) can use this system because it maintains stable item and recipe identities with provenance.
- Equipment Ledger (character_equipment) can use this system because it uses known recipes and item definitions for non-random equipment.
### 13 · Reward and economy contract
- Reward policy: A provenance-marked resource batch with a known source.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive field gathering view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the craft_gathering rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for field gathering: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives resources a clear origin and ties crafting to world knowledge rather than idle grinding.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# craft_recipe_book — The Recipe Codex
> Node 2 of 10 in **Gathering, Crafting and Workshops** · World Loom identity `craft_recipe_book` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A searchable book explains every discovered recipe, exact input quantities, output, tool requirements and alternate uses.
- Why this belongs in Moonbyte: It removes guesswork from crafting and makes the economy understandable.
- Distinctive interaction: Browse a recipe, inspect available inputs, pin it to a workbench and craft when ready.
- Successful completion produces: A versioned recipe definition and a clear material plan.
### 02 · Purpose and boundaries
- Product role: Crafting transforms known inputs into predictable outputs and links exploration, Bazaar demand, homes and collections. It is not a random loot system.
- This system is the canonical owner of the recipe codex behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Gathering, Crafting and Workshops; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It removes guesswork from crafting and makes the economy understandable.
- Stable destination ID: craft_recipe_book; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .craft recipe book and /craft recipe book are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Browse a recipe and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the recipe codex.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A searchable book explains every discovered recipe, exact input quantities, output, tool requirements and alternate uses.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Browse a recipe, inspect available inputs, pin it to a workbench and craft when ready.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Recipe Codex is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned recipe definition and a clear material plan.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- fulfils wanted-board requests and work orders through known recipe outputs: Fixed-Price Listings (bazaar_fixed_listings).
- uses stable item definitions, provenance and collection membership: The Museum Hall (archive_museum).
- turns crafted objects into useful home decorations and workshop upgrades: Friend Visit Rooms (homestead_visits).
- creates disclosed route kits and tools before an expedition begins: Expedition Planner (expedition_planner).
### 12 · Incoming system relationships
- Fixed-Price Listings (bazaar_fixed_listings) can use this system because it connects player demand to published recipes, ingredient lists and crafting work orders.
- Relic Galleries (vault_relic_rooms) can use this system because it connects known recipes and crafted tools to specific puzzle mechanics.
- Collection Sets (archive_sets) can use this system because it maintains stable item and recipe identities with provenance.
- Titles and Identity Marks (character_titles) can use this system because it uses known recipes and item definitions for non-random equipment.
### 13 · Reward and economy contract
- Reward policy: A versioned recipe definition and a clear material plan.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive the recipe codex view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the craft_recipe_book rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the recipe codex: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It removes guesswork from crafting and makes the economy understandable.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# craft_workshop — The Maker's Workshop
> Node 3 of 10 in **Gathering, Crafting and Workshops** · World Loom identity `craft_workshop` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A shared workshop UI lets players queue one deterministic craft at a time, review its inputs and inspect the resulting item.
- Why this belongs in Moonbyte: It centralises crafting rules and gives crafters a recognisable role.
- Distinctive interaction: Select a recipe, reserve materials, confirm the cost, execute the recipe and inspect provenance.
- Successful completion produces: A crafted item and an immutable input/output record.
### 02 · Purpose and boundaries
- Product role: Crafting transforms known inputs into predictable outputs and links exploration, Bazaar demand, homes and collections. It is not a random loot system.
- This system is the canonical owner of the maker's workshop behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Gathering, Crafting and Workshops; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It centralises crafting rules and gives crafters a recognisable role.
- Stable destination ID: craft_workshop; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .craft workshop and /craft workshop are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select a recipe and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the maker's workshop.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A shared workshop UI lets players queue one deterministic craft at a time, review its inputs and inspect the resulting item.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select a recipe, reserve materials, confirm the cost, execute the recipe and inspect provenance.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Maker's Workshop is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A crafted item and an immutable input/output record.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- fulfils wanted-board requests and work orders through known recipe outputs: The Barter Desk (bazaar_barter_desk).
- uses stable item definitions, provenance and collection membership: Collection Sets (archive_sets).
- turns crafted objects into useful home decorations and workshop upgrades: Shared Neighbourhood Square (homestead_neighbourhood).
- creates disclosed route kits and tools before an expedition begins: Route Encounters (expedition_route_events).
### 12 · Incoming system relationships
- The Barter Desk (bazaar_barter_desk) can use this system because it connects player demand to published recipes, ingredient lists and crafting work orders.
- Mastery Trial Rooms (vault_mastery_trials) can use this system because it connects known recipes and crafted tools to specific puzzle mechanics.
- The Cosmetic Gallery (archive_cosmetics) can use this system because it maintains stable item and recipe identities with provenance.
- Pixel Persona Studio (character_appearance) can use this system because it uses known recipes and item definitions for non-random equipment.
### 13 · Reward and economy contract
- Reward policy: A crafted item and an immutable input/output record.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive the maker's workshop view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the craft_workshop rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the maker's workshop: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It centralises crafting rules and gives crafters a recognisable role.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# craft_quality — Known Quality Grades
> Node 4 of 10 in **Gathering, Crafting and Workshops** · World Loom identity `craft_quality` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Item quality comes from named, visible recipe conditions such as material grade or optional puzzle precision, not a hidden random roll.
- Why this belongs in Moonbyte: It permits craft mastery while keeping outcomes reproducible.
- Distinctive interaction: Review quality rules, select known inputs, satisfy the displayed condition and receive the corresponding grade.
- Successful completion produces: A deterministic quality value with its rule version.
### 02 · Purpose and boundaries
- Product role: Crafting transforms known inputs into predictable outputs and links exploration, Bazaar demand, homes and collections. It is not a random loot system.
- This system is the canonical owner of known quality grades behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Gathering, Crafting and Workshops; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It permits craft mastery while keeping outcomes reproducible.
- Stable destination ID: craft_quality; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .craft quality and /craft quality are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Review quality rules and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to known quality grades.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Item quality comes from named, visible recipe conditions such as material grade or optional puzzle precision, not a hidden random roll.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Review quality rules, select known inputs, satisfy the displayed condition and receive the corresponding grade.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Known Quality Grades is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A deterministic quality value with its rule version.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- fulfils wanted-board requests and work orders through known recipe outputs: The Wanted Board (bazaar_wanted_board).
- uses stable item definitions, provenance and collection membership: The Cosmetic Gallery (archive_cosmetics).
- turns crafted objects into useful home decorations and workshop upgrades: Home Design Snapshots (homestead_snapshots).
- creates disclosed route kits and tools before an expedition begins: Expedition Roles (expedition_roles).
### 12 · Incoming system relationships
- The Wanted Board (bazaar_wanted_board) can use this system because it connects player demand to published recipes, ingredient lists and crafting work orders.
- The Grand Mechanism (vault_boss_logic) can use this system because it connects known recipes and crafted tools to specific puzzle mechanics.
- The Living Lore Index (archive_lore) can use this system because it maintains stable item and recipe identities with provenance.
- Non-Combat Talents (character_utility) can use this system because it uses known recipes and item definitions for non-random equipment.
### 13 · Reward and economy contract
- Reward policy: A deterministic quality value with its rule version.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive known quality grades view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the craft_quality rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for known quality grades: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It permits craft mastery while keeping outcomes reproducible.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# craft_commissions — Commission Bench
> Node 5 of 10 in **Gathering, Crafting and Workshops** · World Loom identity `craft_commissions` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A crafter can accept a Bazaar work order with agreed inputs, fixed compensation and an explicit delivery contract.
- Why this belongs in Moonbyte: It connects player skill and supply to real community demand.
- Distinctive interaction: Review the order, accept terms, reserve inputs, craft, deliver and receive the exact settlement.
- Successful completion produces: A completed commission and trade receipt.
### 02 · Purpose and boundaries
- Product role: Crafting transforms known inputs into predictable outputs and links exploration, Bazaar demand, homes and collections. It is not a random loot system.
- This system is the canonical owner of commission bench behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Gathering, Crafting and Workshops; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It connects player skill and supply to real community demand.
- Stable destination ID: craft_commissions; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .craft commissions and /craft commissions are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Review the order and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to commission bench.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A crafter can accept a Bazaar work order with agreed inputs, fixed compensation and an explicit delivery contract.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Review the order, accept terms, reserve inputs, craft, deliver and receive the exact settlement.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Commission Bench is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A completed commission and trade receipt.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- fulfils wanted-board requests and work orders through known recipe outputs: Crafting Work Orders (bazaar_work_orders).
- uses stable item definitions, provenance and collection membership: The Living Lore Index (archive_lore).
- turns crafted objects into useful home decorations and workshop upgrades: Personal Home Plot (homestead_plot).
- creates disclosed route kits and tools before an expedition begins: Prepared Supply Kit (expedition_supplies).
### 12 · Incoming system relationships
- Crafting Work Orders (bazaar_work_orders) can use this system because it connects player demand to published recipes, ingredient lists and crafting work orders.
- Split-Path Chambers (vault_parallel_rooms) can use this system because it connects known recipes and crafted tools to specific puzzle mechanics.
- Collection Milestones (archive_milestones) can use this system because it maintains stable item and recipe identities with provenance.
- Player Character Sheet (character_identity) can use this system because it uses known recipes and item definitions for non-random equipment.
### 13 · Reward and economy contract
- Reward policy: A completed commission and trade receipt.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive commission bench view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the craft_commissions rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for commission bench: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It connects player skill and supply to real community demand.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# craft_recycling — Material Reclaim
> Node 6 of 10 in **Gathering, Crafting and Workshops** · World Loom identity `craft_recycling` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Unneeded eligible items can be converted into a published set of base materials using fixed recipes.
- Why this belongs in Moonbyte: It reduces inventory clutter and gives obsolete items a useful path without destroying value unpredictably.
- Distinctive interaction: Inspect reclaim output, choose eligible items, confirm the exact result and archive the conversion.
- Successful completion produces: A deterministic material conversion record.
### 02 · Purpose and boundaries
- Product role: Crafting transforms known inputs into predictable outputs and links exploration, Bazaar demand, homes and collections. It is not a random loot system.
- This system is the canonical owner of material reclaim behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Gathering, Crafting and Workshops; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It reduces inventory clutter and gives obsolete items a useful path without destroying value unpredictably.
- Stable destination ID: craft_recycling; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .craft recycling and /craft recycling are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect reclaim output and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to material reclaim.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Unneeded eligible items can be converted into a published set of base materials using fixed recipes.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect reclaim output, choose eligible items, confirm the exact result and archive the conversion.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Material Reclaim is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A deterministic material conversion record.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- fulfils wanted-board requests and work orders through known recipe outputs: The Price Reference Ledger (bazaar_price_ledger).
- uses stable item definitions, provenance and collection membership: Collection Milestones (archive_milestones).
- turns crafted objects into useful home decorations and workshop upgrades: Room Layout Studio (homestead_editor).
- creates disclosed route kits and tools before an expedition begins: Waystation Checkpoints (expedition_camps).
### 12 · Incoming system relationships
- The Price Reference Ledger (bazaar_price_ledger) can use this system because it connects player demand to published recipes, ingredient lists and crafting work orders.
- Vault Save Stones (vault_checkpoints) can use this system because it connects known recipes and crafted tools to specific puzzle mechanics.
- Collector's Provenance View (archive_provenance) can use this system because it maintains stable item and recipe identities with provenance.
- Archetype Codex (character_archetypes) can use this system because it uses known recipes and item definitions for non-random equipment.
### 13 · Reward and economy contract
- Reward policy: A deterministic material conversion record.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive material reclaim view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the craft_recycling rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for material reclaim: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It reduces inventory clutter and gives obsolete items a useful path without destroying value unpredictably.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# craft_repair — Workshop Repair
> Node 7 of 10 in **Gathering, Crafting and Workshops** · World Loom identity `craft_repair` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Durable equipment can be restored using known materials and a published repair cost, without punitive breakage timers.
- Why this belongs in Moonbyte: It gives equipment a maintenance role without forcing repetitive chores.
- Distinctive interaction: Inspect an item's condition, view the full repair cost, confirm and restore its listed state.
- Successful completion produces: A repair ledger entry and repaired item state.
### 02 · Purpose and boundaries
- Product role: Crafting transforms known inputs into predictable outputs and links exploration, Bazaar demand, homes and collections. It is not a random loot system.
- This system is the canonical owner of workshop repair behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Gathering, Crafting and Workshops; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives equipment a maintenance role without forcing repetitive chores.
- Stable destination ID: craft_repair; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .craft repair and /craft repair are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect an item's condition and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to workshop repair.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Durable equipment can be restored using known materials and a published repair cost, without punitive breakage timers.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect an item's condition, view the full repair cost, confirm and restore its listed state.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Workshop Repair is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A repair ledger entry and repaired item state.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- fulfils wanted-board requests and work orders through known recipe outputs: The Escrow Vault (bazaar_escrow_vault).
- uses stable item definitions, provenance and collection membership: Collector's Provenance View (archive_provenance).
- turns crafted objects into useful home decorations and workshop upgrades: Display Shelves (homestead_display).
- creates disclosed route kits and tools before an expedition begins: The Expedition Journal (expedition_journal).
### 12 · Incoming system relationships
- The Escrow Vault (bazaar_escrow_vault) can use this system because it connects player demand to published recipes, ingredient lists and crafting work orders.
- Fair Difficulty Profiles (vault_difficulty) can use this system because it connects known recipes and crafted tools to specific puzzle mechanics.
- The Grand Collection Index (archive_catalogue) can use this system because it maintains stable item and recipe identities with provenance.
- Skill Path Map (character_skill_paths) can use this system because it uses known recipes and item definitions for non-random equipment.
### 13 · Reward and economy contract
- Reward policy: A repair ledger entry and repaired item state.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive workshop repair view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the craft_repair rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for workshop repair: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives equipment a maintenance role without forcing repetitive chores.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# craft_sets — Pattern and Set Assembly
> Node 8 of 10 in **Gathering, Crafting and Workshops** · World Loom identity `craft_sets` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Several known components can combine into a decorative or functional set when the exact recipe is complete.
- Why this belongs in Moonbyte: It creates long-range goals that link collections, crafting and homes.
- Distinctive interaction: Review missing components, source them through play or trade, assemble the set and record the recipe.
- Successful completion produces: A set item with deterministic contents and provenance.
### 02 · Purpose and boundaries
- Product role: Crafting transforms known inputs into predictable outputs and links exploration, Bazaar demand, homes and collections. It is not a random loot system.
- This system is the canonical owner of pattern and set assembly behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Gathering, Crafting and Workshops; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates long-range goals that link collections, crafting and homes.
- Stable destination ID: craft_sets; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .craft sets and /craft sets are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Review missing components and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to pattern and set assembly.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Several known components can combine into a decorative or functional set when the exact recipe is complete.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Review missing components, source them through play or trade, assemble the set and record the recipe.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Pattern and Set Assembly is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A set item with deterministic contents and provenance.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- fulfils wanted-board requests and work orders through known recipe outputs: Item Provenance Cards (bazaar_provenance).
- uses stable item definitions, provenance and collection membership: The Grand Collection Index (archive_catalogue).
- turns crafted objects into useful home decorations and workshop upgrades: The Pocket Garden (homestead_garden).
- creates disclosed route kits and tools before an expedition begins: Return and Recovery Routes (expedition_recovery).
### 12 · Incoming system relationships
- Item Provenance Cards (bazaar_provenance) can use this system because it connects player demand to published recipes, ingredient lists and crafting work orders.
- Verified Puzzle Registry (vault_solution_registry) can use this system because it connects known recipes and crafted tools to specific puzzle mechanics.
- The Field Bestiary (archive_bestiary) can use this system because it maintains stable item and recipe identities with provenance.
- Adventure Loadouts (character_loadouts) can use this system because it uses known recipes and item definitions for non-random equipment.
### 13 · Reward and economy contract
- Reward policy: A set item with deterministic contents and provenance.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive pattern and set assembly view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the craft_sets rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for pattern and set assembly: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates long-range goals that link collections, crafting and homes.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# craft_shared_bench — Guild Workshop
> Node 9 of 10 in **Gathering, Crafting and Workshops** · World Loom identity `craft_shared_bench` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Guild members contribute approved materials to a shared project and use a published recipe to produce a community object.
- Why this belongs in Moonbyte: It turns crafting into cooperative infrastructure rather than private inventory accumulation.
- Distinctive interaction: Open the project, inspect contributions, add materials, reach the recipe threshold and execute once.
- Successful completion produces: A guild-owned crafted output and contribution ledger.
### 02 · Purpose and boundaries
- Product role: Crafting transforms known inputs into predictable outputs and links exploration, Bazaar demand, homes and collections. It is not a random loot system.
- This system is the canonical owner of guild workshop behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Gathering, Crafting and Workshops; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It turns crafting into cooperative infrastructure rather than private inventory accumulation.
- Stable destination ID: craft_shared_bench; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .craft shared bench and /craft shared bench are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open the project and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to guild workshop.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Guild members contribute approved materials to a shared project and use a published recipe to produce a community object.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open the project, inspect contributions, add materials, reach the recipe threshold and execute once.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Guild Workshop is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A guild-owned crafted output and contribution ledger.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- fulfils wanted-board requests and work orders through known recipe outputs: The Delivery Board (bazaar_delivery_board).
- uses stable item definitions, provenance and collection membership: The Field Bestiary (archive_bestiary).
- turns crafted objects into useful home decorations and workshop upgrades: Home Workshop Extension (homestead_workshop).
- creates disclosed route kits and tools before an expedition begins: Explorer Contracts (expedition_contracts).
### 12 · Incoming system relationships
- The Delivery Board (bazaar_delivery_board) can use this system because it connects player demand to published recipes, ingredient lists and crafting work orders.
- Vault Blueprint Library (vault_blueprints) can use this system because it connects known recipes and crafted tools to specific puzzle mechanics.
- The Relic Archive (archive_relics) can use this system because it maintains stable item and recipe identities with provenance.
- Companion Chronicle (character_companions) can use this system because it uses known recipes and item definitions for non-random equipment.
### 13 · Reward and economy contract
- Reward policy: A guild-owned crafted output and contribution ledger.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive guild workshop view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the craft_shared_bench rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for guild workshop: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It turns crafting into cooperative infrastructure rather than private inventory accumulation.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# craft_discovery — Recipe Research
> Node 10 of 10 in **Gathering, Crafting and Workshops** · World Loom identity `craft_discovery` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can infer new recipes from authored clues and verified item relationships, then submit a solution for validation.
- Why this belongs in Moonbyte: It makes crafting discovery a puzzle discipline instead of an RNG unlock.
- Distinctive interaction: Collect references, propose a recipe, test against a bounded sandbox and publish only after validation.
- Successful completion produces: A new versioned recipe with discoverer attribution.
### 02 · Purpose and boundaries
- Product role: Crafting transforms known inputs into predictable outputs and links exploration, Bazaar demand, homes and collections. It is not a random loot system.
- This system is the canonical owner of recipe research behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Gathering, Crafting and Workshops; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes crafting discovery a puzzle discipline instead of an RNG unlock.
- Stable destination ID: craft_discovery; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .craft discovery and /craft discovery are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Collect references and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to recipe research.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can infer new recipes from authored clues and verified item relationships, then submit a solution for validation.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Collect references, propose a recipe, test against a bounded sandbox and publish only after validation.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Recipe Research is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A new versioned recipe with discoverer attribution.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- fulfils wanted-board requests and work orders through known recipe outputs: The Trade Review Desk (bazaar_review_court).
- uses stable item definitions, provenance and collection membership: The Relic Archive (archive_relics).
- turns crafted objects into useful home decorations and workshop upgrades: Companion Nook (homestead_companion_nook).
- creates disclosed route kits and tools before an expedition begins: Discovery Scorecard (expedition_score).
### 12 · Incoming system relationships
- The Trade Review Desk (bazaar_review_court) can use this system because it connects player demand to published recipes, ingredient lists and crafting work orders.
- Puzzle Chambers (vault_chambers) can use this system because it connects known recipes and crafted tools to specific puzzle mechanics.
- The Illustrated Compendium (archive_cards) can use this system because it maintains stable item and recipe identities with provenance.
- Role Synergy Board (character_party_synergy) can use this system because it uses known recipes and item definitions for non-random equipment.
### 13 · Reward and economy contract
- Reward policy: A new versioned recipe with discoverer attribution.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive recipe research view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the craft_discovery rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for recipe research: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes crafting discovery a puzzle discipline instead of an RNG unlock.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

## Domain 10 — Collections, Bestiary and Museum

The Archive preserves discoveries, lore and earned cosmetics. Collection completion is an expression of exploration and authorship, not a paid random chase.

**Domain dependencies:** Vaults, Dungeons and Puzzle Spaces (`vault`) · Atlas, Regions and World Discovery (`atlas`) · Gathering, Crafting and Workshops (`craft`) · Homes, Gardens and Personal Spaces (`homestead`)

# archive_catalogue — The Grand Collection Index
> Node 1 of 10 in **Collections, Bestiary and Museum** · World Loom identity `archive_catalogue` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A searchable index records discovered relics, creatures, cards, recipes, cosmetics and authored content under stable IDs.
- Why this belongs in Moonbyte: It creates one consistent collection model across every game and activity.
- Distinctive interaction: Open the index, filter a category, inspect provenance, pin a set and follow a known discovery route.
- Successful completion produces: A personal collection state and stable content references.
### 02 · Purpose and boundaries
- Product role: The Archive preserves discoveries, lore and earned cosmetics. Collection completion is an expression of exploration and authorship, not a paid random chase.
- This system is the canonical owner of the grand collection index behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Collections, Bestiary and Museum; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates one consistent collection model across every game and activity.
- Stable destination ID: archive_catalogue; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .archive catalogue and /archive catalogue are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open the index and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the grand collection index.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A searchable index records discovered relics, creatures, cards, recipes, cosmetics and authored content under stable IDs.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open the index, filter a category, inspect provenance, pin a set and follow a known discovery route.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Grand Collection Index is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A personal collection state and stable content references.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- archives relics and challenge records produced by completed Vault rooms: Vault Blueprint Library (vault_blueprints).
- turns verified location discoveries and cartography into permanent entries: Landmark Network (atlas_landmarks).
- maintains stable item and recipe identities with provenance: Workshop Repair (craft_repair).
- lets players exhibit earned collections in a personally controlled space: Home Design Snapshots (homestead_snapshots).
### 12 · Incoming system relationships
- Player Stallfronts (bazaar_stallfronts) can use this system because it uses canonical item definitions and collection rules to determine what can be listed.
- Biome Ecology (atlas_biomes) can use this system because it stores discoveries, cartography and lore as durable collection entries.
- Waystation Checkpoints (expedition_camps) can use this system because it turns discoveries, route notes and completed objectives into permanent entries.
- Workshop Repair (craft_repair) can use this system because it uses stable item definitions, provenance and collection membership.
### 13 · Reward and economy contract
- Reward policy: A personal collection state and stable content references.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive the grand collection index view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the archive_catalogue rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the grand collection index: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates one consistent collection model across every game and activity.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# archive_bestiary — The Field Bestiary
> Node 2 of 10 in **Collections, Bestiary and Museum** · World Loom identity `archive_bestiary` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: An illustrated bestiary records original creatures, habitats, traits, puzzle clues and non-graphic encounter rules.
- Why this belongs in Moonbyte: It connects creature discovery to the Atlas, character abilities and story.
- Distinctive interaction: Find a creature, complete its authored discovery condition, read its entry and record a new observation.
- Successful completion produces: A bestiary entry with source region and verified facts.
### 02 · Purpose and boundaries
- Product role: The Archive preserves discoveries, lore and earned cosmetics. Collection completion is an expression of exploration and authorship, not a paid random chase.
- This system is the canonical owner of the field bestiary behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Collections, Bestiary and Museum; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It connects creature discovery to the Atlas, character abilities and story.
- Stable destination ID: archive_bestiary; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .archive bestiary and /archive bestiary are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Find a creature and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the field bestiary.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: An illustrated bestiary records original creatures, habitats, traits, puzzle clues and non-graphic encounter rules.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Find a creature, complete its authored discovery condition, read its entry and record a new observation.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Field Bestiary is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A bestiary entry with source region and verified facts.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- archives relics and challenge records produced by completed Vault rooms: Puzzle Chambers (vault_chambers).
- turns verified location discoveries and cartography into permanent entries: Known Passage Network (atlas_shortcuts).
- maintains stable item and recipe identities with provenance: Pattern and Set Assembly (craft_sets).
- lets players exhibit earned collections in a personally controlled space: Personal Home Plot (homestead_plot).
### 12 · Incoming system relationships
- Fixed-Price Listings (bazaar_fixed_listings) can use this system because it uses canonical item definitions and collection rules to determine what can be listed.
- Landmark Network (atlas_landmarks) can use this system because it stores discoveries, cartography and lore as durable collection entries.
- The Expedition Journal (expedition_journal) can use this system because it turns discoveries, route notes and completed objectives into permanent entries.
- Pattern and Set Assembly (craft_sets) can use this system because it uses stable item definitions, provenance and collection membership.
### 13 · Reward and economy contract
- Reward policy: A bestiary entry with source region and verified facts.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive the field bestiary view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the archive_bestiary rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the field bestiary: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It connects creature discovery to the Atlas, character abilities and story.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# archive_relics — The Relic Archive
> Node 3 of 10 in **Collections, Bestiary and Museum** · World Loom identity `archive_relics` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Relics are curated story objects with a known source, lore text, collection rule and display permissions.
- Why this belongs in Moonbyte: It makes exploration rewards meaningful through history and context.
- Distinctive interaction: Discover a relic, inspect its source, add it to the Archive and optionally display it.
- Successful completion produces: A permanent relic record with provenance.
### 02 · Purpose and boundaries
- Product role: The Archive preserves discoveries, lore and earned cosmetics. Collection completion is an expression of exploration and authorship, not a paid random chase.
- This system is the canonical owner of the relic archive behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Collections, Bestiary and Museum; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes exploration rewards meaningful through history and context.
- Stable destination ID: archive_relics; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .archive relics and /archive relics are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Discover a relic and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the relic archive.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Relics are curated story objects with a known source, lore text, collection rule and display permissions.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Discover a relic, inspect its source, add it to the Archive and optionally display it.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Relic Archive is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A permanent relic record with provenance.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- archives relics and challenge records produced by completed Vault rooms: Logic Key Mechanisms (vault_logic_keys).
- turns verified location discoveries and cartography into permanent entries: Environmental Logic Sites (atlas_environment_puzzles).
- maintains stable item and recipe identities with provenance: Guild Workshop (craft_shared_bench).
- lets players exhibit earned collections in a personally controlled space: Room Layout Studio (homestead_editor).
### 12 · Incoming system relationships
- The Barter Desk (bazaar_barter_desk) can use this system because it uses canonical item definitions and collection rules to determine what can be listed.
- Known Passage Network (atlas_shortcuts) can use this system because it stores discoveries, cartography and lore as durable collection entries.
- Return and Recovery Routes (expedition_recovery) can use this system because it turns discoveries, route notes and completed objectives into permanent entries.
- Guild Workshop (craft_shared_bench) can use this system because it uses stable item definitions, provenance and collection membership.
### 13 · Reward and economy contract
- Reward policy: A permanent relic record with provenance.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive the relic archive view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the archive_relics rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the relic archive: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes exploration rewards meaningful through history and context.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# archive_cards — The Illustrated Compendium
> Node 4 of 10 in **Collections, Bestiary and Museum** · World Loom identity `archive_cards` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Collectible cards represent characters, landmarks, puzzles and authored scenes, with guaranteed sources and no random packs.
- Why this belongs in Moonbyte: It offers a visual collection layer without loot boxes or chance-based tradable rewards.
- Distinctive interaction: Inspect a card source, complete its listed activity, claim the exact card and add it to a set.
- Successful completion produces: An account-bound or explicitly eligible card with documented origin.
### 02 · Purpose and boundaries
- Product role: The Archive preserves discoveries, lore and earned cosmetics. Collection completion is an expression of exploration and authorship, not a paid random chase.
- This system is the canonical owner of the illustrated compendium behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Collections, Bestiary and Museum; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It offers a visual collection layer without loot boxes or chance-based tradable rewards.
- Stable destination ID: archive_cards; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .archive cards and /archive cards are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect a card source and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the illustrated compendium.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Collectible cards represent characters, landmarks, puzzles and authored scenes, with guaranteed sources and no random packs.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect a card source, complete its listed activity, claim the exact card and add it to a set.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Illustrated Compendium is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: An account-bound or explicitly eligible card with documented origin.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- archives relics and challenge records produced by completed Vault rooms: Relic Galleries (vault_relic_rooms).
- turns verified location discoveries and cartography into permanent entries: Atlas Overlays (atlas_map_layers).
- maintains stable item and recipe identities with provenance: Recipe Research (craft_discovery).
- lets players exhibit earned collections in a personally controlled space: Display Shelves (homestead_display).
### 12 · Incoming system relationships
- The Wanted Board (bazaar_wanted_board) can use this system because it uses canonical item definitions and collection rules to determine what can be listed.
- Environmental Logic Sites (atlas_environment_puzzles) can use this system because it stores discoveries, cartography and lore as durable collection entries.
- Explorer Contracts (expedition_contracts) can use this system because it turns discoveries, route notes and completed objectives into permanent entries.
- Recipe Research (craft_discovery) can use this system because it uses stable item definitions, provenance and collection membership.
### 13 · Reward and economy contract
- Reward policy: An account-bound or explicitly eligible card with documented origin.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive the illustrated compendium view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the archive_cards rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the illustrated compendium: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It offers a visual collection layer without loot boxes or chance-based tradable rewards.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# archive_museum — The Museum Hall
> Node 5 of 10 in **Collections, Bestiary and Museum** · World Loom identity `archive_museum` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players and guilds can arrange approved items and discoveries into themed exhibitions with clear authorship and visibility.
- Why this belongs in Moonbyte: It gives collections a social purpose and turns personal progress into optional public storytelling.
- Distinctive interaction: Choose a theme, select entries, arrange the display, preview permissions and publish or keep private.
- Successful completion produces: A versioned exhibition that can be edited or removed by its owner.
### 02 · Purpose and boundaries
- Product role: The Archive preserves discoveries, lore and earned cosmetics. Collection completion is an expression of exploration and authorship, not a paid random chase.
- This system is the canonical owner of the museum hall behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Collections, Bestiary and Museum; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives collections a social purpose and turns personal progress into optional public storytelling.
- Stable destination ID: archive_museum; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .archive museum and /archive museum are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a theme and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the museum hall.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players and guilds can arrange approved items and discoveries into themed exhibitions with clear authorship and visibility.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a theme, select entries, arrange the display, preview permissions and publish or keep private.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Museum Hall is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned exhibition that can be edited or removed by its owner.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- archives relics and challenge records produced by completed Vault rooms: Mastery Trial Rooms (vault_mastery_trials).
- turns verified location discoveries and cartography into permanent entries: Cartographer's Desk (atlas_cartographer).
- maintains stable item and recipe identities with provenance: Field Gathering (craft_gathering).
- lets players exhibit earned collections in a personally controlled space: The Pocket Garden (homestead_garden).
### 12 · Incoming system relationships
- Crafting Work Orders (bazaar_work_orders) can use this system because it uses canonical item definitions and collection rules to determine what can be listed.
- Atlas Overlays (atlas_map_layers) can use this system because it stores discoveries, cartography and lore as durable collection entries.
- Discovery Scorecard (expedition_score) can use this system because it turns discoveries, route notes and completed objectives into permanent entries.
- Field Gathering (craft_gathering) can use this system because it uses stable item definitions, provenance and collection membership.
### 13 · Reward and economy contract
- Reward policy: A versioned exhibition that can be edited or removed by its owner.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive the museum hall view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the archive_museum rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the museum hall: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives collections a social purpose and turns personal progress into optional public storytelling.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# archive_sets — Collection Sets
> Node 6 of 10 in **Collections, Bestiary and Museum** · World Loom identity `archive_sets` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Sets group related discoveries by theme, region or chapter and show exact missing entries and source activities.
- Why this belongs in Moonbyte: It provides clear, attainable goals without hidden rarity odds.
- Distinctive interaction: Choose a set, inspect each item source, complete optional activities and claim the fixed set completion mark.
- Successful completion produces: A completed set record and non-power cosmetic recognition.
### 02 · Purpose and boundaries
- Product role: The Archive preserves discoveries, lore and earned cosmetics. Collection completion is an expression of exploration and authorship, not a paid random chase.
- This system is the canonical owner of collection sets behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Collections, Bestiary and Museum; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It provides clear, attainable goals without hidden rarity odds.
- Stable destination ID: archive_sets; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .archive sets and /archive sets are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a set and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to collection sets.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Sets group related discoveries by theme, region or chapter and show exact missing entries and source activities.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a set, inspect each item source, complete optional activities and claim the fixed set completion mark.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Collection Sets is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A completed set record and non-power cosmetic recognition.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- archives relics and challenge records produced by completed Vault rooms: The Grand Mechanism (vault_boss_logic).
- turns verified location discoveries and cartography into permanent entries: The Route Planner (atlas_route_planner).
- maintains stable item and recipe identities with provenance: The Recipe Codex (craft_recipe_book).
- lets players exhibit earned collections in a personally controlled space: Home Workshop Extension (homestead_workshop).
### 12 · Incoming system relationships
- The Price Reference Ledger (bazaar_price_ledger) can use this system because it uses canonical item definitions and collection rules to determine what can be listed.
- Cartographer's Desk (atlas_cartographer) can use this system because it stores discoveries, cartography and lore as durable collection entries.
- Expedition Route Archive (expedition_replay) can use this system because it turns discoveries, route notes and completed objectives into permanent entries.
- The Recipe Codex (craft_recipe_book) can use this system because it uses stable item definitions, provenance and collection membership.
### 13 · Reward and economy contract
- Reward policy: A completed set record and non-power cosmetic recognition.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive collection sets view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the archive_sets rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for collection sets: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It provides clear, attainable goals without hidden rarity odds.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# archive_cosmetics — The Cosmetic Gallery
> Node 7 of 10 in **Collections, Bestiary and Museum** · World Loom identity `archive_cosmetics` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A gallery organises earned palettes, frames, emotes and profile decorations while separating appearance from power.
- Why this belongs in Moonbyte: It gives progression visible expression without pay-to-win effects.
- Distinctive interaction: Preview a cosmetic, inspect its source, equip it or place it in a display and change it freely.
- Successful completion produces: A cosmetic ownership flag and equipped appearance setting.
### 02 · Purpose and boundaries
- Product role: The Archive preserves discoveries, lore and earned cosmetics. Collection completion is an expression of exploration and authorship, not a paid random chase.
- This system is the canonical owner of the cosmetic gallery behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Collections, Bestiary and Museum; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives progression visible expression without pay-to-win effects.
- Stable destination ID: archive_cosmetics; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .archive cosmetics and /archive cosmetics are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Preview a cosmetic and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the cosmetic gallery.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A gallery organises earned palettes, frames, emotes and profile decorations while separating appearance from power.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Preview a cosmetic, inspect its source, equip it or place it in a display and change it freely.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Cosmetic Gallery is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A cosmetic ownership flag and equipped appearance setting.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- archives relics and challenge records produced by completed Vault rooms: Split-Path Chambers (vault_parallel_rooms).
- turns verified location discoveries and cartography into permanent entries: Living World State (atlas_world_state).
- maintains stable item and recipe identities with provenance: The Maker's Workshop (craft_workshop).
- lets players exhibit earned collections in a personally controlled space: Companion Nook (homestead_companion_nook).
### 12 · Incoming system relationships
- The Escrow Vault (bazaar_escrow_vault) can use this system because it uses canonical item definitions and collection rules to determine what can be listed.
- The Route Planner (atlas_route_planner) can use this system because it stores discoveries, cartography and lore as durable collection entries.
- Expedition Planner (expedition_planner) can use this system because it turns discoveries, route notes and completed objectives into permanent entries.
- The Maker's Workshop (craft_workshop) can use this system because it uses stable item definitions, provenance and collection membership.
### 13 · Reward and economy contract
- Reward policy: A cosmetic ownership flag and equipped appearance setting.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive the cosmetic gallery view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the archive_cosmetics rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the cosmetic gallery: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives progression visible expression without pay-to-win effects.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# archive_lore — The Living Lore Index
> Node 8 of 10 in **Collections, Bestiary and Museum** · World Loom identity `archive_lore` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Lore entries link characters, factions, landmarks, items and chapters through citations to the actual game content.
- Why this belongs in Moonbyte: It makes story comprehension cumulative and supports returning players.
- Distinctive interaction: Open a lore topic, follow related nodes, inspect source chapters and save a reading path.
- Successful completion produces: A graph of unlocked knowledge with provenance.
### 02 · Purpose and boundaries
- Product role: The Archive preserves discoveries, lore and earned cosmetics. Collection completion is an expression of exploration and authorship, not a paid random chase.
- This system is the canonical owner of the living lore index behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Collections, Bestiary and Museum; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes story comprehension cumulative and supports returning players.
- Stable destination ID: archive_lore; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .archive lore and /archive lore are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open a lore topic and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the living lore index.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Lore entries link characters, factions, landmarks, items and chapters through citations to the actual game content.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open a lore topic, follow related nodes, inspect source chapters and save a reading path.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Living Lore Index is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A graph of unlocked knowledge with provenance.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- archives relics and challenge records produced by completed Vault rooms: Vault Save Stones (vault_checkpoints).
- turns verified location discoveries and cartography into permanent entries: The Grand Atlas (atlas_world_index).
- maintains stable item and recipe identities with provenance: Known Quality Grades (craft_quality).
- lets players exhibit earned collections in a personally controlled space: Decorative Collections (homestead_decor_sets).
### 12 · Incoming system relationships
- Item Provenance Cards (bazaar_provenance) can use this system because it uses canonical item definitions and collection rules to determine what can be listed.
- Living World State (atlas_world_state) can use this system because it stores discoveries, cartography and lore as durable collection entries.
- Route Encounters (expedition_route_events) can use this system because it turns discoveries, route notes and completed objectives into permanent entries.
- Known Quality Grades (craft_quality) can use this system because it uses stable item definitions, provenance and collection membership.
### 13 · Reward and economy contract
- Reward policy: A graph of unlocked knowledge with provenance.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive the living lore index view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the archive_lore rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the living lore index: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes story comprehension cumulative and supports returning players.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# archive_milestones — Collection Milestones
> Node 9 of 10 in **Collections, Bestiary and Museum** · World Loom identity `archive_milestones` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Milestones recognise breadth, curation and exploration across sets rather than requiring one rare item drop.
- Why this belongs in Moonbyte: It rewards varied participation and makes completion criteria legible.
- Distinctive interaction: Inspect a milestone, see its required set of actions, complete them naturally and claim its fixed recognition.
- Successful completion produces: An idempotent achievement tied to collection state.
### 02 · Purpose and boundaries
- Product role: The Archive preserves discoveries, lore and earned cosmetics. Collection completion is an expression of exploration and authorship, not a paid random chase.
- This system is the canonical owner of collection milestones behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Collections, Bestiary and Museum; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It rewards varied participation and makes completion criteria legible.
- Stable destination ID: archive_milestones; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .archive milestones and /archive milestones are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect a milestone and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to collection milestones.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Milestones recognise breadth, curation and exploration across sets rather than requiring one rare item drop.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect a milestone, see its required set of actions, complete them naturally and claim its fixed recognition.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Collection Milestones is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: An idempotent achievement tied to collection state.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- archives relics and challenge records produced by completed Vault rooms: Fair Difficulty Profiles (vault_difficulty).
- turns verified location discoveries and cartography into permanent entries: Discovery Layers (atlas_discovery_fog).
- maintains stable item and recipe identities with provenance: Commission Bench (craft_commissions).
- lets players exhibit earned collections in a personally controlled space: Friend Visit Rooms (homestead_visits).
### 12 · Incoming system relationships
- The Delivery Board (bazaar_delivery_board) can use this system because it uses canonical item definitions and collection rules to determine what can be listed.
- The Grand Atlas (atlas_world_index) can use this system because it stores discoveries, cartography and lore as durable collection entries.
- Expedition Roles (expedition_roles) can use this system because it turns discoveries, route notes and completed objectives into permanent entries.
- Commission Bench (craft_commissions) can use this system because it uses stable item definitions, provenance and collection membership.
### 13 · Reward and economy contract
- Reward policy: An idempotent achievement tied to collection state.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive collection milestones view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the archive_milestones rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for collection milestones: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It rewards varied participation and makes completion criteria legible.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# archive_provenance — Collector's Provenance View
> Node 10 of 10 in **Collections, Bestiary and Museum** · World Loom identity `archive_provenance` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A detailed item history shows the recipe, quest or discovery that created an asset without exposing the private identity of previous holders.
- Why this belongs in Moonbyte: It strengthens marketplace trust and protects legitimate earned ownership.
- Distinctive interaction: Open an asset, inspect allowed origin records, verify its definition and return to the collection or Bazaar.
- Successful completion produces: A privacy-filtered provenance chain and integrity reference.
### 02 · Purpose and boundaries
- Product role: The Archive preserves discoveries, lore and earned cosmetics. Collection completion is an expression of exploration and authorship, not a paid random chase.
- This system is the canonical owner of collector's provenance view behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Collections, Bestiary and Museum; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It strengthens marketplace trust and protects legitimate earned ownership.
- Stable destination ID: archive_provenance; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .archive provenance and /archive provenance are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open an asset and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to collector's provenance view.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A detailed item history shows the recipe, quest or discovery that created an asset without exposing the private identity of previous holders.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open an asset, inspect allowed origin records, verify its definition and return to the collection or Bazaar.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Collector's Provenance View is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A privacy-filtered provenance chain and integrity reference.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- archives relics and challenge records produced by completed Vault rooms: Verified Puzzle Registry (vault_solution_registry).
- turns verified location discoveries and cartography into permanent entries: Biome Ecology (atlas_biomes).
- maintains stable item and recipe identities with provenance: Material Reclaim (craft_recycling).
- lets players exhibit earned collections in a personally controlled space: Shared Neighbourhood Square (homestead_neighbourhood).
### 12 · Incoming system relationships
- The Trade Review Desk (bazaar_review_court) can use this system because it uses canonical item definitions and collection rules to determine what can be listed.
- Discovery Layers (atlas_discovery_fog) can use this system because it stores discoveries, cartography and lore as durable collection entries.
- Prepared Supply Kit (expedition_supplies) can use this system because it turns discoveries, route notes and completed objectives into permanent entries.
- Material Reclaim (craft_recycling) can use this system because it uses stable item definitions, provenance and collection membership.
### 13 · Reward and economy contract
- Reward policy: A privacy-filtered provenance chain and integrity reference.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive collector's provenance view view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the archive_provenance rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for collector's provenance view: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It strengthens marketplace trust and protects legitimate earned ownership.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

## Domain 11 — Arcade and Original Mini-Games

The Arcade is a curated library of short, medium and mastery-based games. Results feed profiles and quests through explicit, bounded contracts.

**Domain dependencies:** The Hearth and Tavern (`hearth`) · Progression, Mastery and Recognition (`mastery`) · Fair Play, Competition and Verification (`fairness`) · Friends, Community Bonds and Social Play (`fellowship`)

# arcade_library — The Pocket Arcade
> Node 1 of 10 in **Arcade and Original Mini-Games** · World Loom identity `arcade_library` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A visual catalogue presents approved games with rules, player count, expected session length, accessibility options and release status.
- Why this belongs in Moonbyte: It makes Moonbyte's broad game library discoverable and helps players choose an experience that fits their time.
- Distinctive interaction: Browse categories, compare game cards, preview rules and start a solo or multiplayer session.
- Successful completion produces: A game selection and a validated session request.
### 02 · Purpose and boundaries
- Product role: The Arcade is a curated library of short, medium and mastery-based games. Results feed profiles and quests through explicit, bounded contracts.
- This system is the canonical owner of the pocket arcade behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Arcade and Original Mini-Games; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes Moonbyte's broad game library discoverable and helps players choose an experience that fits their time.
- Stable destination ID: arcade_library; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .arcade library and /arcade library are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Browse categories and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the pocket arcade.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A visual catalogue presents approved games with rules, player count, expected session length, accessibility options and release status.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Browse categories, compare game cards, preview rules and start a solo or multiplayer session.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Pocket Arcade is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A game selection and a validated session request.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- lets players start a short game from a Tavern table without navigating a separate service: The Lantern and Ladle (hearth_main_room).
- creates personal records, practice paths and transparent skill achievements: Player Honour Record (mastery_reputation).
- documents random inputs and competitive scoring where verification is useful: Tournament Rules Desk (fairness_tournament).
- supports friend challenges and shared play with clear exit and privacy controls: Social Boundaries Panel (fellowship_safety).
### 12 · Incoming system relationships
- The Lantern and Ladle (hearth_main_room) can use this system because it provides immediate short-session games between longer adventures.
- Channel Wayfinding (server_channels) can use this system because it lets administrators choose which approved game categories appear in the server library.
- Simultaneous Choice Windows (multiplayer_simultaneous) can use this system because it runs approved games through the same lobby and session engine.
- Earned Cosmetic Unlocks (mastery_cosmetics) can use this system because it uses game results and practice lessons as skill evidence.
### 13 · Reward and economy contract
- Reward policy: A game selection and a validated session request.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive the pocket arcade view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the arcade_library rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the pocket arcade: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes Moonbyte's broad game library discoverable and helps players choose an experience that fits their time.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# arcade_daily_logic — Optional Daily Logic Desk
> Node 2 of 10 in **Arcade and Original Mini-Games** · World Loom identity `arcade_daily_logic` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A shared daily puzzle uses a published ruleset and lets anyone attempt it without streak loss or missed-day penalties.
- Why this belongs in Moonbyte: It gives the community a common puzzle to discuss while keeping participation entirely optional.
- Distinctive interaction: Open today's puzzle, inspect the rules, play at a chosen time and review the solution after completion.
- Successful completion produces: A dated puzzle result and a personal best that does not decay.
### 02 · Purpose and boundaries
- Product role: The Arcade is a curated library of short, medium and mastery-based games. Results feed profiles and quests through explicit, bounded contracts.
- This system is the canonical owner of optional daily logic desk behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Arcade and Original Mini-Games; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives the community a common puzzle to discuss while keeping participation entirely optional.
- Stable destination ID: arcade_daily_logic; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .arcade daily logic and /arcade daily logic are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open today's puzzle and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to optional daily logic desk.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A shared daily puzzle uses a published ruleset and lets anyone attempt it without streak loss or missed-day penalties.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open today's puzzle, inspect the rules, play at a chosen time and review the solution after completion.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Optional Daily Logic Desk is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A dated puzzle result and a personal best that does not decay.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- lets players start a short game from a Tavern table without navigating a separate service: The Notice Wall (hearth_notice_wall).
- creates personal records, practice paths and transparent skill achievements: Personal Milestone Contracts (mastery_contracts).
- documents random inputs and competitive scoring where verification is useful: Leaderboard Integrity (fairness_leaderboard).
- supports friend challenges and shared play with clear exit and privacy controls: Friend Circle (fellowship_friends).
### 12 · Incoming system relationships
- The Notice Wall (hearth_notice_wall) can use this system because it provides immediate short-session games between longer adventures.
- Server Challenge Board (server_challenges) can use this system because it lets administrators choose which approved game categories appear in the server library.
- Spectator Gallery (multiplayer_spectators) can use this system because it runs approved games through the same lobby and session engine.
- Skill Badge Portfolio (mastery_badges) can use this system because it uses game results and practice lessons as skill evidence.
### 13 · Reward and economy contract
- Reward policy: A dated puzzle result and a personal best that does not decay.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive optional daily logic desk view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the arcade_daily_logic rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for optional daily logic desk: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives the community a common puzzle to discuss while keeping participation entirely optional.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# arcade_skill_school — The Practice School
> Node 3 of 10 in **Arcade and Original Mini-Games** · World Loom identity `arcade_skill_school` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Each supported game can expose guided lessons, sample turns and progressively harder practice challenges.
- Why this belongs in Moonbyte: It helps players learn rather than relying on unexplained losses or hidden difficulty.
- Distinctive interaction: Choose a lesson, inspect the goal, try a guided example, practise independently and review the method.
- Successful completion produces: A completed lesson record and recommended next difficulty.
### 02 · Purpose and boundaries
- Product role: The Arcade is a curated library of short, medium and mastery-based games. Results feed profiles and quests through explicit, bounded contracts.
- This system is the canonical owner of the practice school behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Arcade and Original Mini-Games; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It helps players learn rather than relying on unexplained losses or hidden difficulty.
- Stable destination ID: arcade_skill_school; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .arcade skill school and /arcade skill school are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a lesson and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the practice school.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Each supported game can expose guided lessons, sample turns and progressively harder practice challenges.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a lesson, inspect the goal, try a guided example, practise independently and review the method.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Practice School is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A completed lesson record and recommended next difficulty.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- lets players start a short game from a Tavern table without navigating a separate service: The Rumour Circle (hearth_rumour_circle).
- creates personal records, practice paths and transparent skill achievements: Milestone Route Planner (mastery_routes).
- documents random inputs and competitive scoring where verification is useful: Equal Conditions Profiles (fairness_access).
- supports friend challenges and shared play with clear exit and privacy controls: Cooperative Role Planner (fellowship_roles).
### 12 · Incoming system relationships
- The Rumour Circle (hearth_rumour_circle) can use this system because it provides immediate short-session games between longer adventures.
- The Community Museum (server_museum) can use this system because it lets administrators choose which approved game categories appear in the server library.
- Friendly Tournament Brackets (multiplayer_brackets) can use this system because it runs approved games through the same lobby and session engine.
- Personal Best Registry (mastery_records) can use this system because it uses game results and practice lessons as skill evidence.
### 13 · Reward and economy contract
- Reward policy: A completed lesson record and recommended next difficulty.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive the practice school view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the arcade_skill_school rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the practice school: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It helps players learn rather than relying on unexplained losses or hidden difficulty.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# arcade_puzzle_factory — Puzzle Generator and Validator
> Node 4 of 10 in **Arcade and Original Mini-Games** · World Loom identity `arcade_puzzle_factory` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A deterministic generator produces puzzles only when a solution validator confirms their solvability and intended difficulty.
- Why this belongs in Moonbyte: It creates scalable content without publishing broken or ambiguous puzzles.
- Distinctive interaction: Request a puzzle profile, generate from a versioned seed, validate the solution and publish only approved content.
- Successful completion produces: A verified puzzle instance and reproducible solution key.
### 02 · Purpose and boundaries
- Product role: The Arcade is a curated library of short, medium and mastery-based games. Results feed profiles and quests through explicit, bounded contracts.
- This system is the canonical owner of puzzle generator and validator behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Arcade and Original Mini-Games; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates scalable content without publishing broken or ambiguous puzzles.
- Stable destination ID: arcade_puzzle_factory; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .arcade puzzle factory and /arcade puzzle factory are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Request a puzzle profile and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to puzzle generator and validator.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A deterministic generator produces puzzles only when a solution validator confirms their solvability and intended difficulty.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Request a puzzle profile, generate from a versioned seed, validate the solution and publish only approved content.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Puzzle Generator and Validator is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A verified puzzle instance and reproducible solution key.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- lets players start a short game from a Tavern table without navigating a separate service: The Common Tables (hearth_table_games).
- creates personal records, practice paths and transparent skill achievements: Earned Cosmetic Unlocks (mastery_cosmetics).
- documents random inputs and competitive scoring where verification is useful: Result Appeal Desk (fairness_appeals).
- supports friend challenges and shared play with clear exit and privacy controls: Opt-In Mentor Sessions (fellowship_mentor).
### 12 · Incoming system relationships
- The Common Tables (hearth_table_games) can use this system because it provides immediate short-session games between longer adventures.
- Neighbourhood Goals (server_goals) can use this system because it lets administrators choose which approved game categories appear in the server library.
- Shared Cooperative Boards (multiplayer_coop_boards) can use this system because it runs approved games through the same lobby and session engine.
- Account-Bound Recognition (mastery_account_rewards) can use this system because it uses game results and practice lessons as skill evidence.
### 13 · Reward and economy contract
- Reward policy: A verified puzzle instance and reproducible solution key.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive puzzle generator and validator view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the arcade_puzzle_factory rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for puzzle generator and validator: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates scalable content without publishing broken or ambiguous puzzles.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# arcade_party_collection — The Party Game Shelf
> Node 5 of 10 in **Arcade and Original Mini-Games** · World Loom identity `arcade_party_collection` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Fast social games share the same lobby, turn ownership, scoring and leave controls rather than inventing unique multiplayer plumbing.
- Why this belongs in Moonbyte: It makes the Arcade feel cohesive and reduces bugs across multiple titles.
- Distinctive interaction: Choose a party game, invite participants, ready the table, play a bounded round and show a result.
- Successful completion produces: A completed party session and shared summary.
### 02 · Purpose and boundaries
- Product role: The Arcade is a curated library of short, medium and mastery-based games. Results feed profiles and quests through explicit, bounded contracts.
- This system is the canonical owner of the party game shelf behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Arcade and Original Mini-Games; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes the Arcade feel cohesive and reduces bugs across multiple titles.
- Stable destination ID: arcade_party_collection; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .arcade party collection and /arcade party collection are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a party game and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the party game shelf.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Fast social games share the same lobby, turn ownership, scoring and leave controls rather than inventing unique multiplayer plumbing.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a party game, invite participants, ready the table, play a bounded round and show a result.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Party Game Shelf is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A completed party session and shared summary.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- lets players start a short game from a Tavern table without navigating a separate service: The Host's Bench (hearth_hosts_bench).
- creates personal records, practice paths and transparent skill achievements: Skill Badge Portfolio (mastery_badges).
- documents random inputs and competitive scoring where verification is useful: Published Rulebooks (fairness_rulebooks).
- supports friend challenges and shared play with clear exit and privacy controls: Interest Clubs (fellowship_clubs).
### 12 · Incoming system relationships
- The Host's Bench (hearth_hosts_bench) can use this system because it provides immediate short-session games between longer adventures.
- Scoped Scoreboards (server_leaderboards) can use this system because it lets administrators choose which approved game categories appear in the server library.
- Reconnect and Rejoin (multiplayer_reconnect) can use this system because it runs approved games through the same lobby and session engine.
- Experience and Milestone Ledger (mastery_experience) can use this system because it uses game results and practice lessons as skill evidence.
### 13 · Reward and economy contract
- Reward policy: A completed party session and shared summary.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive the party game shelf view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the arcade_party_collection rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the party game shelf: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes the Arcade feel cohesive and reduces bugs across multiple titles.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# arcade_async_challenges — Asynchronous Challenge Cards
> Node 6 of 10 in **Arcade and Original Mini-Games** · World Loom identity `arcade_async_challenges` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can send a friend a defined puzzle or score challenge that can be completed independently within a generous window.
- Why this belongs in Moonbyte: It supports friends in different time zones without a live-session requirement.
- Distinctive interaction: Create a challenge from a supported game, share a link, complete one attempt and compare verified scores.
- Successful completion produces: A paired challenge record with deterministic comparison rules.
### 02 · Purpose and boundaries
- Product role: The Arcade is a curated library of short, medium and mastery-based games. Results feed profiles and quests through explicit, bounded contracts.
- This system is the canonical owner of asynchronous challenge cards behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Arcade and Original Mini-Games; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports friends in different time zones without a live-session requirement.
- Stable destination ID: arcade_async_challenges; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .arcade async challenges and /arcade async challenges are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Create a challenge from a supported game and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to asynchronous challenge cards.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can send a friend a defined puzzle or score challenge that can be completed independently within a generous window.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Create a challenge from a supported game, share a link, complete one attempt and compare verified scores.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Asynchronous Challenge Cards is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A paired challenge record with deterministic comparison rules.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- lets players start a short game from a Tavern table without navigating a separate service: The Story Stage (hearth_bard_stage).
- creates personal records, practice paths and transparent skill achievements: Personal Best Registry (mastery_records).
- documents random inputs and competitive scoring where verification is useful: Commit and Reveal Service (fairness_commit_reveal).
- supports friend challenges and shared play with clear exit and privacy controls: Thoughtful Gift Exchange (fellowship_gifting).
### 12 · Incoming system relationships
- The Story Stage (hearth_bard_stage) can use this system because it provides immediate short-session games between longer adventures.
- Server Steward Desk (server_moderation) can use this system because it lets administrators choose which approved game categories appear in the server library.
- Table Votes and Host Controls (multiplayer_table_votes) can use this system because it runs approved games through the same lobby and session engine.
- Skill Mastery Tracks (mastery_tracks) can use this system because it uses game results and practice lessons as skill evidence.
### 13 · Reward and economy contract
- Reward policy: A paired challenge record with deterministic comparison rules.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive asynchronous challenge cards view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the arcade_async_challenges rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for asynchronous challenge cards: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports friends in different time zones without a live-session requirement.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# arcade_league — Friendly Arcade League
> Node 7 of 10 in **Arcade and Original Mini-Games** · World Loom identity `arcade_league` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: An opt-in league groups comparable scores across a fixed rules version and gives participants a clear way to leave or hide results.
- Why this belongs in Moonbyte: It adds structured competition without prizes, entry fees or daily attendance pressure.
- Distinctive interaction: Join a league, inspect ranking rules, play during the published period and review final standings.
- Successful completion produces: A rules-versioned score table and cosmetic recognition only.
### 02 · Purpose and boundaries
- Product role: The Arcade is a curated library of short, medium and mastery-based games. Results feed profiles and quests through explicit, bounded contracts.
- This system is the canonical owner of friendly arcade league behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Arcade and Original Mini-Games; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It adds structured competition without prizes, entry fees or daily attendance pressure.
- Stable destination ID: arcade_league; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .arcade league and /arcade league are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Join a league and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to friendly arcade league.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: An opt-in league groups comparable scores across a fixed rules version and gives participants a clear way to leave or hide results.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Join a league, inspect ranking rules, play during the published period and review final standings.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Friendly Arcade League is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A rules-versioned score table and cosmetic recognition only.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- lets players start a short game from a Tavern table without navigating a separate service: The Hearth Kitchen (hearth_kitchen).
- creates personal records, practice paths and transparent skill achievements: Account-Bound Recognition (mastery_account_rewards).
- documents random inputs and competitive scoring where verification is useful: Scoring Contract Registry (fairness_scoring).
- supports friend challenges and shared play with clear exit and privacy controls: Community Message Board (fellowship_mailbox).
### 12 · Incoming system relationships
- The Hearth Kitchen (hearth_kitchen) can use this system because it provides immediate short-session games between longer adventures.
- Server Data Boundary (server_privacy) can use this system because it lets administrators choose which approved game categories appear in the server library.
- Universal Lobby Contract (multiplayer_lobby) can use this system because it runs approved games through the same lobby and session engine.
- Achievement Registry (mastery_achievements) can use this system because it uses game results and practice lessons as skill evidence.
### 13 · Reward and economy contract
- Reward policy: A rules-versioned score table and cosmetic recognition only.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive friendly arcade league view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the arcade_league rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for friendly arcade league: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It adds structured competition without prizes, entry fees or daily attendance pressure.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# arcade_replays — Replay and Review Desk
> Node 8 of 10 in **Arcade and Original Mini-Games** · World Loom identity `arcade_replays` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A replay view reconstructs legal actions and key decisions for supported turn-based games without exposing hidden player information.
- Why this belongs in Moonbyte: It helps players learn from a match and creates useful shared moments.
- Distinctive interaction: Open a completed match, choose an allowed perspective, step through actions and inspect the rule explanation.
- Successful completion produces: A redacted action replay tied to a match version.
### 02 · Purpose and boundaries
- Product role: The Arcade is a curated library of short, medium and mastery-based games. Results feed profiles and quests through explicit, bounded contracts.
- This system is the canonical owner of replay and review desk behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Arcade and Original Mini-Games; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It helps players learn from a match and creates useful shared moments.
- Stable destination ID: arcade_replays; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .arcade replays and /arcade replays are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open a completed match and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to replay and review desk.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A replay view reconstructs legal actions and key decisions for supported turn-based games without exposing hidden player information.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open a completed match, choose an allowed perspective, step through actions and inspect the rule explanation.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Replay and Review Desk is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A redacted action replay tied to a match version.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- lets players start a short game from a Tavern table without navigating a separate service: The Regulars' Circle (hearth_regulars).
- creates personal records, practice paths and transparent skill achievements: Experience and Milestone Ledger (mastery_experience).
- documents random inputs and competitive scoring where verification is useful: Optional Skill Rating (fairness_ratings).
- supports friend challenges and shared play with clear exit and privacy controls: Positive Contribution Notes (fellowship_commendations).
### 12 · Incoming system relationships
- The Regulars' Circle (hearth_regulars) can use this system because it provides immediate short-session games between longer adventures.
- The Quiet Community Digest (server_digest) can use this system because it lets administrators choose which approved game categories appear in the server library.
- Party Formation Desk (multiplayer_party) can use this system because it runs approved games through the same lobby and session engine.
- Player Honour Record (mastery_reputation) can use this system because it uses game results and practice lessons as skill evidence.
### 13 · Reward and economy contract
- Reward policy: A redacted action replay tied to a match version.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive replay and review desk view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the arcade_replays rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for replay and review desk: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It helps players learn from a match and creates useful shared moments.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# arcade_accessibility — Arcade Control Profiles
> Node 9 of 10 in **Arcade and Original Mini-Games** · World Loom identity `arcade_accessibility` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Games expose named control profiles for simplified inputs, longer thinking windows, text-only boards and reduced-motion presentation.
- Why this belongs in Moonbyte: It lets more people participate on Discord without requiring fast reactions or precise timing.
- Distinctive interaction: Preview a profile, apply it before a match, check supported modes and retain the preference.
- Successful completion produces: A per-player input preference with game compatibility metadata.
### 02 · Purpose and boundaries
- Product role: The Arcade is a curated library of short, medium and mastery-based games. Results feed profiles and quests through explicit, bounded contracts.
- This system is the canonical owner of arcade control profiles behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Arcade and Original Mini-Games; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It lets more people participate on Discord without requiring fast reactions or precise timing.
- Stable destination ID: arcade_accessibility; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .arcade accessibility and /arcade accessibility are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Preview a profile and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to arcade control profiles.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Games expose named control profiles for simplified inputs, longer thinking windows, text-only boards and reduced-motion presentation.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Preview a profile, apply it before a match, check supported modes and retain the preference.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Arcade Control Profiles is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A per-player input preference with game compatibility metadata.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- lets players start a short game from a Tavern table without navigating a separate service: The Common Room Project (hearth_renovation).
- creates personal records, practice paths and transparent skill achievements: Skill Mastery Tracks (mastery_tracks).
- documents random inputs and competitive scoring where verification is useful: Match Review Transcript (fairness_match_review).
- supports friend challenges and shared play with clear exit and privacy controls: Shared Session Memories (fellowship_memories).
### 12 · Incoming system relationships
- The Common Room Project (hearth_renovation) can use this system because it provides immediate short-session games between longer adventures.
- Server World Identity (server_identity) can use this system because it lets administrators choose which approved game categories appear in the server library.
- Friend-First Match Finder (multiplayer_matchmaking) can use this system because it runs approved games through the same lobby and session engine.
- Personal Milestone Contracts (mastery_contracts) can use this system because it uses game results and practice lessons as skill evidence.
### 13 · Reward and economy contract
- Reward policy: A per-player input preference with game compatibility metadata.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive arcade control profiles view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the arcade_accessibility rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for arcade control profiles: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It lets more people participate on Discord without requiring fast reactions or precise timing.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# arcade_playlists — Curated Game Routes
> Node 10 of 10 in **Arcade and Original Mini-Games** · World Loom identity `arcade_playlists` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A playlist links several games around a theme, estimated total time and optional transitions rather than demanding an endless session.
- Why this belongs in Moonbyte: It creates guided discovery and lets communities run compact themed nights.
- Distinctive interaction: Choose a playlist, inspect every included game, launch one step, finish or skip, then stop at any point.
- Successful completion produces: A completed playlist path and optional recap.
### 02 · Purpose and boundaries
- Product role: The Arcade is a curated library of short, medium and mastery-based games. Results feed profiles and quests through explicit, bounded contracts.
- This system is the canonical owner of curated game routes behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Arcade and Original Mini-Games; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates guided discovery and lets communities run compact themed nights.
- Stable destination ID: arcade_playlists; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .arcade playlists and /arcade playlists are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a playlist and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to curated game routes.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A playlist links several games around a theme, estimated total time and optional transitions rather than demanding an endless session.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a playlist, inspect every included game, launch one step, finish or skip, then stop at any point.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Curated Game Routes is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A completed playlist path and optional recap.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- lets players start a short game from a Tavern table without navigating a separate service: The Tavern Wayfinder (hearth_routes).
- creates personal records, practice paths and transparent skill achievements: Achievement Registry (mastery_achievements).
- documents random inputs and competitive scoring where verification is useful: Integrity Guardrails (fairness_anti_abuse).
- supports friend challenges and shared play with clear exit and privacy controls: Friend Discovery Routes (fellowship_discovery).
### 12 · Incoming system relationships
- The Tavern Wayfinder (hearth_routes) can use this system because it provides immediate short-session games between longer adventures.
- Community Calendar (server_calendar) can use this system because it lets administrators choose which approved game categories appear in the server library.
- Shared Turn Engine (multiplayer_turn_engine) can use this system because it runs approved games through the same lobby and session engine.
- Milestone Route Planner (mastery_routes) can use this system because it uses game results and practice lessons as skill evidence.
### 13 · Reward and economy contract
- Reward policy: A completed playlist path and optional recap.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive curated game routes view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the arcade_playlists rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for curated game routes: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates guided discovery and lets communities run compact themed nights.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

## Domain 12 — Parties, Lobbies and Multiplayer Rules

Multiplayer uses one shared session contract for identity, turn ownership, deadlines, reconnects, spectators and host controls.

**Domain dependencies:** Guilds and Long-Form Cooperation (`guild`) · Arcade and Original Mini-Games (`arcade`) · Friends, Community Bonds and Social Play (`fellowship`) · Fair Play, Competition and Verification (`fairness`)

# multiplayer_lobby — Universal Lobby Contract
> Node 1 of 10 in **Parties, Lobbies and Multiplayer Rules** · World Loom identity `multiplayer_lobby` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A single lobby system handles visibility, host identity, seat limits, invitations, readiness and launch conditions for every multiplayer game.
- Why this belongs in Moonbyte: It prevents game-specific invitation bugs and makes lobbies familiar across Moonbyte.
- Distinctive interaction: Create a lobby, choose privacy, invite or admit players, inspect the rule preview and launch when valid.
- Successful completion produces: A versioned lobby state and membership list.
### 02 · Purpose and boundaries
- Product role: Multiplayer uses one shared session contract for identity, turn ownership, deadlines, reconnects, spectators and host controls.
- This system is the canonical owner of universal lobby contract behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Parties, Lobbies and Multiplayer Rules; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It prevents game-specific invitation bugs and makes lobbies familiar across Moonbyte.
- Stable destination ID: multiplayer_lobby; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .multiplayer lobby and /multiplayer lobby are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Create a lobby and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to universal lobby contract.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A single lobby system handles visibility, host identity, seat limits, invitations, readiness and launch conditions for every multiplayer game.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Create a lobby, choose privacy, invite or admit players, inspect the rule preview and launch when valid.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Universal Lobby Contract is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned lobby state and membership list.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- allows guild members to coordinate long-running group activities: Guild Charters (guild_charters).
- runs approved games through the same lobby and session engine: Puzzle Generator and Validator (arcade_puzzle_factory).
- provides friend invitations, opt-in groups and cooperative roles: Positive Contribution Notes (fellowship_commendations).
- uses documented rules and verifiable transcripts for supported competitive outcomes: Result Appeal Desk (fairness_appeals).
### 12 · Incoming system relationships
- Published Rulebooks (fairness_rulebooks) can use this system because it ensures turn order, scoring and participant contributions are reproducible.
- Opt-In Mentor Sessions (fellowship_mentor) can use this system because it uses the universal lobby, turn and reconnect contracts.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A versioned lobby state and membership list.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive universal lobby contract view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the multiplayer_lobby rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for universal lobby contract: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It prevents game-specific invitation bugs and makes lobbies familiar across Moonbyte.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# multiplayer_party — Party Formation Desk
> Node 2 of 10 in **Parties, Lobbies and Multiplayer Rules** · World Loom identity `multiplayer_party` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can create a temporary party, invite known friends and carry the group between compatible activities.
- Why this belongs in Moonbyte: It makes cooperation persist across games without inventing a public matchmaking pressure loop.
- Distinctive interaction: Create a party, review membership, choose a destination, confirm availability and enter a session together.
- Successful completion produces: A temporary party identity with explicit membership and exit controls.
### 02 · Purpose and boundaries
- Product role: Multiplayer uses one shared session contract for identity, turn ownership, deadlines, reconnects, spectators and host controls.
- This system is the canonical owner of party formation desk behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Parties, Lobbies and Multiplayer Rules; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes cooperation persist across games without inventing a public matchmaking pressure loop.
- Stable destination ID: multiplayer_party; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .multiplayer party and /multiplayer party are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Create a party and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to party formation desk.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can create a temporary party, invite known friends and carry the group between compatible activities.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Create a party, review membership, choose a destination, confirm availability and enter a session together.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Party Formation Desk is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A temporary party identity with explicit membership and exit controls.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- allows guild members to coordinate long-running group activities: The Guild Hall (guild_hall).
- runs approved games through the same lobby and session engine: The Party Game Shelf (arcade_party_collection).
- provides friend invitations, opt-in groups and cooperative roles: Shared Session Memories (fellowship_memories).
- uses documented rules and verifiable transcripts for supported competitive outcomes: Published Rulebooks (fairness_rulebooks).
### 12 · Incoming system relationships
- Commit and Reveal Service (fairness_commit_reveal) can use this system because it ensures turn order, scoring and participant contributions are reproducible.
- Interest Clubs (fellowship_clubs) can use this system because it uses the universal lobby, turn and reconnect contracts.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A temporary party identity with explicit membership and exit controls.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive party formation desk view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the multiplayer_party rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for party formation desk: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes cooperation persist across games without inventing a public matchmaking pressure loop.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# multiplayer_matchmaking — Friend-First Match Finder
> Node 3 of 10 in **Parties, Lobbies and Multiplayer Rules** · World Loom identity `multiplayer_matchmaking` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: The match finder prioritises invited friends and clearly labelled public rooms, with skill grouping only where a game needs it.
- Why this belongs in Moonbyte: It helps players find a match without dark patterns or opaque engagement ranking.
- Distinctive interaction: Choose a game and mode, inspect queue visibility, join or cancel, then accept a compatible lobby.
- Successful completion produces: A match proposal with visible constraints and cancellation state.
### 02 · Purpose and boundaries
- Product role: Multiplayer uses one shared session contract for identity, turn ownership, deadlines, reconnects, spectators and host controls.
- This system is the canonical owner of friend-first match finder behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Parties, Lobbies and Multiplayer Rules; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It helps players find a match without dark patterns or opaque engagement ranking.
- Stable destination ID: multiplayer_matchmaking; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .multiplayer matchmaking and /multiplayer matchmaking are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a game and mode and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to friend-first match finder.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: The match finder prioritises invited friends and clearly labelled public rooms, with skill grouping only where a game needs it.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a game and mode, inspect queue visibility, join or cancel, then accept a compatible lobby.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Friend-First Match Finder is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A match proposal with visible constraints and cancellation state.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- allows guild members to coordinate long-running group activities: Guild Role Charter (guild_roles).
- runs approved games through the same lobby and session engine: Asynchronous Challenge Cards (arcade_async_challenges).
- provides friend invitations, opt-in groups and cooperative roles: Friend Discovery Routes (fellowship_discovery).
- uses documented rules and verifiable transcripts for supported competitive outcomes: Commit and Reveal Service (fairness_commit_reveal).
### 12 · Incoming system relationships
- Scoring Contract Registry (fairness_scoring) can use this system because it ensures turn order, scoring and participant contributions are reproducible.
- Thoughtful Gift Exchange (fellowship_gifting) can use this system because it uses the universal lobby, turn and reconnect contracts.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A match proposal with visible constraints and cancellation state.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive friend-first match finder view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the multiplayer_matchmaking rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for friend-first match finder: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It helps players find a match without dark patterns or opaque engagement ranking.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# multiplayer_turn_engine — Shared Turn Engine
> Node 4 of 10 in **Parties, Lobbies and Multiplayer Rules** · World Loom identity `multiplayer_turn_engine` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Turn-based games use one engine for actor ownership, legal action validation, phase changes and stale interaction rejection.
- Why this belongs in Moonbyte: It gives every game a tested foundation and protects against button races.
- Distinctive interaction: Receive the turn, inspect legal actions, submit one action, receive authoritative resolution and pass control.
- Successful completion produces: A canonical turn event and versioned game state.
### 02 · Purpose and boundaries
- Product role: Multiplayer uses one shared session contract for identity, turn ownership, deadlines, reconnects, spectators and host controls.
- This system is the canonical owner of shared turn engine behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Parties, Lobbies and Multiplayer Rules; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives every game a tested foundation and protects against button races.
- Stable destination ID: multiplayer_turn_engine; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .multiplayer turn engine and /multiplayer turn engine are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Receive the turn and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to shared turn engine.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Turn-based games use one engine for actor ownership, legal action validation, phase changes and stale interaction rejection.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Receive the turn, inspect legal actions, submit one action, receive authoritative resolution and pass control.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Shared Turn Engine is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A canonical turn event and versioned game state.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- allows guild members to coordinate long-running group activities: The Great Works Board (guild_projects).
- runs approved games through the same lobby and session engine: Friendly Arcade League (arcade_league).
- provides friend invitations, opt-in groups and cooperative roles: Social Boundaries Panel (fellowship_safety).
- uses documented rules and verifiable transcripts for supported competitive outcomes: Scoring Contract Registry (fairness_scoring).
### 12 · Incoming system relationships
- Optional Skill Rating (fairness_ratings) can use this system because it ensures turn order, scoring and participant contributions are reproducible.
- Community Message Board (fellowship_mailbox) can use this system because it uses the universal lobby, turn and reconnect contracts.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A canonical turn event and versioned game state.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive shared turn engine view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the multiplayer_turn_engine rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for shared turn engine: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives every game a tested foundation and protects against button races.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# multiplayer_simultaneous — Simultaneous Choice Windows
> Node 5 of 10 in **Parties, Lobbies and Multiplayer Rules** · World Loom identity `multiplayer_simultaneous` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Selected games collect sealed choices from all participants before resolving the phase using a published rule order.
- Why this belongs in Moonbyte: It enables strategic group play without frame-based real-time networking.
- Distinctive interaction: Open a phase, submit a private choice, revise before lock, resolve once and show a shared summary.
- Successful completion produces: A sealed phase record with deterministic resolution.
### 02 · Purpose and boundaries
- Product role: Multiplayer uses one shared session contract for identity, turn ownership, deadlines, reconnects, spectators and host controls.
- This system is the canonical owner of simultaneous choice windows behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Parties, Lobbies and Multiplayer Rules; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It enables strategic group play without frame-based real-time networking.
- Stable destination ID: multiplayer_simultaneous; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .multiplayer simultaneous and /multiplayer simultaneous are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open a phase and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to simultaneous choice windows.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Selected games collect sealed choices from all participants before resolving the phase using a published rule order.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open a phase, submit a private choice, revise before lock, resolve once and show a shared summary.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Simultaneous Choice Windows is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A sealed phase record with deterministic resolution.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- allows guild members to coordinate long-running group activities: Guild Story Contracts (guild_quests).
- runs approved games through the same lobby and session engine: Replay and Review Desk (arcade_replays).
- provides friend invitations, opt-in groups and cooperative roles: Friend Circle (fellowship_friends).
- uses documented rules and verifiable transcripts for supported competitive outcomes: Optional Skill Rating (fairness_ratings).
### 12 · Incoming system relationships
- Match Review Transcript (fairness_match_review) can use this system because it ensures turn order, scoring and participant contributions are reproducible.
- Positive Contribution Notes (fellowship_commendations) can use this system because it uses the universal lobby, turn and reconnect contracts.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A sealed phase record with deterministic resolution.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive simultaneous choice windows view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the multiplayer_simultaneous rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for simultaneous choice windows: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It enables strategic group play without frame-based real-time networking.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# multiplayer_spectators — Spectator Gallery
> Node 6 of 10 in **Parties, Lobbies and Multiplayer Rules** · World Loom identity `multiplayer_spectators` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Hosts can enable read-only spectators for supported public games with deliberate information redaction.
- Why this belongs in Moonbyte: It allows community viewing without giving spectators control or exposing hidden hands and roles.
- Distinctive interaction: Inspect spectator policy, join as view-only, receive public board updates and leave independently.
- Successful completion produces: A spectator permission and filtered view model.
### 02 · Purpose and boundaries
- Product role: Multiplayer uses one shared session contract for identity, turn ownership, deadlines, reconnects, spectators and host controls.
- This system is the canonical owner of spectator gallery behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Parties, Lobbies and Multiplayer Rules; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It allows community viewing without giving spectators control or exposing hidden hands and roles.
- Stable destination ID: multiplayer_spectators; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .multiplayer spectators and /multiplayer spectators are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect spectator policy and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to spectator gallery.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Hosts can enable read-only spectators for supported public games with deliberate information redaction.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect spectator policy, join as view-only, receive public board updates and leave independently.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Spectator Gallery is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A spectator permission and filtered view model.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- allows guild members to coordinate long-running group activities: The Guild Vault (guild_vault).
- runs approved games through the same lobby and session engine: Arcade Control Profiles (arcade_accessibility).
- provides friend invitations, opt-in groups and cooperative roles: Cooperative Role Planner (fellowship_roles).
- uses documented rules and verifiable transcripts for supported competitive outcomes: Match Review Transcript (fairness_match_review).
### 12 · Incoming system relationships
- Integrity Guardrails (fairness_anti_abuse) can use this system because it ensures turn order, scoring and participant contributions are reproducible.
- Shared Session Memories (fellowship_memories) can use this system because it uses the universal lobby, turn and reconnect contracts.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A spectator permission and filtered view model.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive spectator gallery view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the multiplayer_spectators rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for spectator gallery: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It allows community viewing without giving spectators control or exposing hidden hands and roles.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# multiplayer_brackets — Friendly Tournament Brackets
> Node 7 of 10 in **Parties, Lobbies and Multiplayer Rules** · World Loom identity `multiplayer_brackets` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A bracket organises opt-in matches with clear seeds, tie rules and schedules but no pooled stakes or cash-equivalent prizes.
- Why this belongs in Moonbyte: It provides tournament structure while respecting Moonbyte's no-wagering boundary.
- Distinctive interaction: Create a bracket, publish rules, register teams, play each round and archive the final results.
- Successful completion produces: A deterministic bracket and cosmetic achievement record.
### 02 · Purpose and boundaries
- Product role: Multiplayer uses one shared session contract for identity, turn ownership, deadlines, reconnects, spectators and host controls.
- This system is the canonical owner of friendly tournament brackets behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Parties, Lobbies and Multiplayer Rules; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It provides tournament structure while respecting Moonbyte's no-wagering boundary.
- Stable destination ID: multiplayer_brackets; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .multiplayer brackets and /multiplayer brackets are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Create a bracket and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to friendly tournament brackets.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A bracket organises opt-in matches with clear seeds, tie rules and schedules but no pooled stakes or cash-equivalent prizes.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Create a bracket, publish rules, register teams, play each round and archive the final results.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Friendly Tournament Brackets is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A deterministic bracket and cosmetic achievement record.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- allows guild members to coordinate long-running group activities: The Guild Cartography Room (guild_map).
- runs approved games through the same lobby and session engine: Curated Game Routes (arcade_playlists).
- provides friend invitations, opt-in groups and cooperative roles: Opt-In Mentor Sessions (fellowship_mentor).
- uses documented rules and verifiable transcripts for supported competitive outcomes: Integrity Guardrails (fairness_anti_abuse).
### 12 · Incoming system relationships
- Tournament Rules Desk (fairness_tournament) can use this system because it ensures turn order, scoring and participant contributions are reproducible.
- Friend Discovery Routes (fellowship_discovery) can use this system because it uses the universal lobby, turn and reconnect contracts.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A deterministic bracket and cosmetic achievement record.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive friendly tournament brackets view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the multiplayer_brackets rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for friendly tournament brackets: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It provides tournament structure while respecting Moonbyte's no-wagering boundary.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# multiplayer_coop_boards — Shared Cooperative Boards
> Node 8 of 10 in **Parties, Lobbies and Multiplayer Rules** · World Loom identity `multiplayer_coop_boards` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A cooperative board lets players contribute legal actions toward one group objective with visible contribution attribution.
- Why this belongs in Moonbyte: It makes collaboration more than parallel solo play.
- Distinctive interaction: Create a group board, assign or choose a task, submit an action, inspect shared state and resolve the objective.
- Successful completion produces: A shared board state with a contribution timeline.
### 02 · Purpose and boundaries
- Product role: Multiplayer uses one shared session contract for identity, turn ownership, deadlines, reconnects, spectators and host controls.
- This system is the canonical owner of shared cooperative boards behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Parties, Lobbies and Multiplayer Rules; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes collaboration more than parallel solo play.
- Stable destination ID: multiplayer_coop_boards; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .multiplayer coop boards and /multiplayer coop boards are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Create a group board and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to shared cooperative boards.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A cooperative board lets players contribute legal actions toward one group objective with visible contribution attribution.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Create a group board, assign or choose a task, submit an action, inspect shared state and resolve the objective.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Shared Cooperative Boards is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A shared board state with a contribution timeline.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- allows guild members to coordinate long-running group activities: Guild Neighbourhoods (guild_alliances).
- runs approved games through the same lobby and session engine: The Pocket Arcade (arcade_library).
- provides friend invitations, opt-in groups and cooperative roles: Interest Clubs (fellowship_clubs).
- uses documented rules and verifiable transcripts for supported competitive outcomes: Tournament Rules Desk (fairness_tournament).
### 12 · Incoming system relationships
- Leaderboard Integrity (fairness_leaderboard) can use this system because it ensures turn order, scoring and participant contributions are reproducible.
- Social Boundaries Panel (fellowship_safety) can use this system because it uses the universal lobby, turn and reconnect contracts.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A shared board state with a contribution timeline.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive shared cooperative boards view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the multiplayer_coop_boards rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for shared cooperative boards: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes collaboration more than parallel solo play.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# multiplayer_reconnect — Reconnect and Rejoin
> Node 9 of 10 in **Parties, Lobbies and Multiplayer Rules** · World Loom identity `multiplayer_reconnect` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A reconnect path restores a participant to a valid session state after app reload, network interruption or Discord message replacement.
- Why this belongs in Moonbyte: It prevents accidental session loss and reduces reliance on one live interaction.
- Distinctive interaction: Reopen the session, authenticate through Discord context, inspect the saved phase and rejoin if permitted.
- Successful completion produces: A recovered participant state with stale-action protection.
### 02 · Purpose and boundaries
- Product role: Multiplayer uses one shared session contract for identity, turn ownership, deadlines, reconnects, spectators and host controls.
- This system is the canonical owner of reconnect and rejoin behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Parties, Lobbies and Multiplayer Rules; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It prevents accidental session loss and reduces reliance on one live interaction.
- Stable destination ID: multiplayer_reconnect; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .multiplayer reconnect and /multiplayer reconnect are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Reopen the session and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to reconnect and rejoin.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A reconnect path restores a participant to a valid session state after app reload, network interruption or Discord message replacement.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Reopen the session, authenticate through Discord context, inspect the saved phase and rejoin if permitted.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Reconnect and Rejoin is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A recovered participant state with stale-action protection.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- allows guild members to coordinate long-running group activities: Guild Trust Record (guild_reputation).
- runs approved games through the same lobby and session engine: Optional Daily Logic Desk (arcade_daily_logic).
- provides friend invitations, opt-in groups and cooperative roles: Thoughtful Gift Exchange (fellowship_gifting).
- uses documented rules and verifiable transcripts for supported competitive outcomes: Leaderboard Integrity (fairness_leaderboard).
### 12 · Incoming system relationships
- Equal Conditions Profiles (fairness_access) can use this system because it ensures turn order, scoring and participant contributions are reproducible.
- Friend Circle (fellowship_friends) can use this system because it uses the universal lobby, turn and reconnect contracts.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A recovered participant state with stale-action protection.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive reconnect and rejoin view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the multiplayer_reconnect rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for reconnect and rejoin: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It prevents accidental session loss and reduces reliance on one live interaction.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---

# multiplayer_table_votes — Table Votes and Host Controls
> Node 10 of 10 in **Parties, Lobbies and Multiplayer Rules** · World Loom identity `multiplayer_table_votes` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A bounded host toolkit handles start, pause, continue, kick, transfer-host and cancellation under clear permission rules.
- Why this belongs in Moonbyte: It makes group sessions manageable without allowing one participant to silently rewrite outcomes.
- Distinctive interaction: Inspect the current phase, choose an allowed host action, confirm consequential changes and record the event.
- Successful completion produces: An auditable host action and explicit session transition.
### 02 · Purpose and boundaries
- Product role: Multiplayer uses one shared session contract for identity, turn ownership, deadlines, reconnects, spectators and host controls.
- This system is the canonical owner of table votes and host controls behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Parties, Lobbies and Multiplayer Rules; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes group sessions manageable without allowing one participant to silently rewrite outcomes.
- Stable destination ID: multiplayer_table_votes; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .multiplayer table votes and /multiplayer table votes are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect the current phase and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to table votes and host controls.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A bounded host toolkit handles start, pause, continue, kick, transfer-host and cancellation under clear permission rules.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect the current phase, choose an allowed host action, confirm consequential changes and record the event.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Table Votes and Host Controls is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: An auditable host action and explicit session transition.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- allows guild members to coordinate long-running group activities: The Guild Chronicle (guild_archive).
- runs approved games through the same lobby and session engine: The Practice School (arcade_skill_school).
- provides friend invitations, opt-in groups and cooperative roles: Community Message Board (fellowship_mailbox).
- uses documented rules and verifiable transcripts for supported competitive outcomes: Equal Conditions Profiles (fairness_access).
### 12 · Incoming system relationships
- Result Appeal Desk (fairness_appeals) can use this system because it ensures turn order, scoring and participant contributions are reproducible.
- Cooperative Role Planner (fellowship_roles) can use this system because it uses the universal lobby, turn and reconnect contracts.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: An auditable host action and explicit session transition.; exact eligible rewards and costs are disclosed before confirmation.
- Moonbits are earned-only, non-redeemable and never staked; this node cannot introduce money purchases, cash-out or external exchange.
- Randomness, when a game genuinely needs it, is for score-only outcomes and follows FAIRNESS.md; it never allocates tradable assets.
- No reward is tied to punitive streaks, artificial scarcity, attendance pressure or an unreasonably long online session.
### 14 · Discord-native interaction
- Prefix and slash commands share one Rust application service; command parsing never owns game rules.
- Messages, embeds, buttons, select menus and modals expose only the actions valid for the current state.
- Public boards contain only public game state; private clues, profile details, exports and trade offers use private delivery where supported.
- A deleted message or expired interaction can be recreated from durable state without inventing a second session.
### 15 · Visual Library and Activity surface
- The visual library uses Next.js, React and TypeScript as a presentation layer, with the official Discord Embedded App SDK for Activity integration.
- A responsive table votes and host controls view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the multiplayer_table_votes rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for table votes and host controls: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes group sessions manageable without allowing one participant to silently rewrite outcomes.; it must remain satisfying in a bounded session and support a natural stopping point.
- No punitive streak, forced login, artificial scarcity, nagging reminder, hidden countdown or infinite reward loop is permitted.
- Daily or rotating content is optional and archived where practical; absence never destroys earned progress or blocks the main story.
- User-created text and assets follow age-appropriate content policy, reporting, rights review and server moderation settings.
### 20 · Accessibility and resilient UX
- Every important action has a clear label, keyboard-accessible web control and Discord-native alternative where the platform supports it.
- Colour is never the only state signal; contrast, reduced motion, readable text density and text-only views are supported.
- Mobile layout must keep the primary action, current phase and exit control visible without horizontal scrolling.
- Loading, stale, unavailable and permission-denied states explain what happened without implying that an uncommitted action succeeded.
### 21 · Abuse cases and failure recovery
- Test duplicate clicks, stale buttons, concurrent requests, rate-limit responses, deleted messages and reconnects against the same state version.
- Test self-trade, duplicate claims, contribution farming, invalid ownership, cross-guild reads and permission changes where relevant.
- On partial failure, preserve committed state, return uncommitted reservations exactly once and expose a recoverable status.
- Moderation actions are scoped, reasoned and audited; they cannot silently rewrite game results or delete a player's unrelated progress.
### 22 · Acceptance criteria and release path
- Unit tests prove legal and illegal actions, state transitions, deterministic outputs and edge cases for this node.
- Integration tests prove Discord delivery, SQLite persistence, restart recovery, privacy export/deletion and guild isolation where applicable.
- MVP release begins with one narrow vertical slice; expanded variants ship only after the core contract is stable and measured.
- The feature is not marked implemented merely because this concept document exists; release status must match tested code.

---
