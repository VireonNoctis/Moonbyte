# Volume V — Creation, Competition and Trust

> Moonbyte World Loom · 40 connected system proposals · This is a design specification, not a claim of implementation.

Each card is a distinct proposal with a stable graph identity, a player loop, state and ownership boundaries, Discord and Activity surfaces, privacy rules, acceptance tests and explicit cross-system links.

## Domain 17 — Player Creation and Community Content

Creation tools use constrained templates, provenance, moderation and reproducible tests. Original work can be shared, but no user submission silently becomes trusted game logic.

**Domain dependencies:** Accessibility, Moderation, Privacy and Operations (`trust`) · Arcade and Original Mini-Games (`arcade`) · Quests, Mystery and Story (`quest`) · Collections, Bestiary and Museum (`archive`)

# creator_puzzle_studio — Puzzle Studio
> Node 1 of 10 in **Player Creation and Community Content** · World Loom identity `creator_puzzle_studio` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can compose puzzles from approved templates with clues, answer rules, difficulty labels and validation tests.
- Why this belongs in Moonbyte: It invites community creativity while preventing ambiguous or impossible puzzles from entering public play.
- Distinctive interaction: Choose a template, enter original clues, run local validation, preview text and submit for review.
- Successful completion produces: A draft puzzle package and validation report.
### 02 · Purpose and boundaries
- Product role: Creation tools use constrained templates, provenance, moderation and reproducible tests. Original work can be shared, but no user submission silently becomes trusted game logic.
- This system is the canonical owner of puzzle studio behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Player Creation and Community Content; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It invites community creativity while preventing ambiguous or impossible puzzles from entering public play.
- Stable destination ID: creator_puzzle_studio; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .creator puzzle studio and /creator puzzle studio are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a template and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to puzzle studio.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can compose puzzles from approved templates with clues, answer rules, difficulty labels and validation tests.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a template, enter original clues, run local validation, preview text and submit for review.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Puzzle Studio is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A draft puzzle package and validation report.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- applies reporting, review, permissions and privacy policy to player-submitted material: Universal Control Profiles (trust_control_profiles).
- allows approved puzzle packs and variants to appear in the curated game library: Puzzle Generator and Validator (arcade_puzzle_factory).
- supports authored side stories and scenario prompts through bounded dialogue templates: Local Tales (quest_side_stories).
- stores approved community works and their creators with explicit visibility: Collector's Provenance View (archive_provenance).
### 12 · Incoming system relationships
- Universal Control Profiles (trust_control_profiles) can use this system because it validates player-generated material before public distribution.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A draft puzzle package and validation report.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive puzzle studio view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the creator_puzzle_studio rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for puzzle studio: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It invites community creativity while preventing ambiguous or impossible puzzles from entering public play.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# creator_quest_editor — Quest Workshop
> Node 2 of 10 in **Player Creation and Community Content** · World Loom identity `creator_quest_editor` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: An authoring tool creates short branching stories from constrained scenes, conditions and original dialogue blocks.
- Why this belongs in Moonbyte: It expands narrative content without allowing arbitrary scripts or untrusted code execution.
- Distinctive interaction: Choose a story template, connect scenes, define conditions, preview all branches and submit for review.
- Successful completion produces: A versioned quest graph with author attribution.
### 02 · Purpose and boundaries
- Product role: Creation tools use constrained templates, provenance, moderation and reproducible tests. Original work can be shared, but no user submission silently becomes trusted game logic.
- This system is the canonical owner of quest workshop behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Player Creation and Community Content; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It expands narrative content without allowing arbitrary scripts or untrusted code execution.
- Stable destination ID: creator_quest_editor; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .creator quest editor and /creator quest editor are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a story template and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to quest workshop.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: An authoring tool creates short branching stories from constrained scenes, conditions and original dialogue blocks.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a story template, connect scenes, define conditions, preview all branches and submit for review.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Quest Workshop is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned quest graph with author attribution.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- applies reporting, review, permissions and privacy policy to player-submitted material: Visual Comfort Settings (trust_contrast_motion).
- allows approved puzzle packs and variants to appear in the curated game library: The Party Game Shelf (arcade_party_collection).
- supports authored side stories and scenario prompts through bounded dialogue templates: Shared Story Decisions (quest_coop_story).
- stores approved community works and their creators with explicit visibility: The Grand Collection Index (archive_catalogue).
### 12 · Incoming system relationships
- Visual Comfort Settings (trust_contrast_motion) can use this system because it validates player-generated material before public distribution.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A versioned quest graph with author attribution.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive quest workshop view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the creator_quest_editor rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for quest workshop: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It expands narrative content without allowing arbitrary scripts or untrusted code execution.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# creator_vault_builder — Vault Layout Workshop
> Node 3 of 10 in **Player Creation and Community Content** · World Loom identity `creator_vault_builder` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Creators can arrange supported chamber types, clue placements and published reward definitions into a testable Vault draft.
- Why this belongs in Moonbyte: It makes challenge design extensible while keeping room logic within approved contracts.
- Distinctive interaction: Select chambers, wire legal exits, run solution validation, test the complete route and submit the blueprint.
- Successful completion produces: A Vault draft with solvability evidence.
### 02 · Purpose and boundaries
- Product role: Creation tools use constrained templates, provenance, moderation and reproducible tests. Original work can be shared, but no user submission silently becomes trusted game logic.
- This system is the canonical owner of vault layout workshop behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Player Creation and Community Content; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes challenge design extensible while keeping room logic within approved contracts.
- Stable destination ID: creator_vault_builder; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .creator vault builder and /creator vault builder are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select chambers and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to vault layout workshop.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Creators can arrange supported chamber types, clue placements and published reward definitions into a testable Vault draft.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select chambers, wire legal exits, run solution validation, test the complete route and submit the blueprint.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Vault Layout Workshop is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A Vault draft with solvability evidence.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- applies reporting, review, permissions and privacy policy to player-submitted material: Language and Locale Registry (trust_localisation).
- allows approved puzzle packs and variants to appear in the curated game library: Asynchronous Challenge Cards (arcade_async_challenges).
- supports authored side stories and scenario prompts through bounded dialogue templates: The Chapter Chronicle (quest_chronicle).
- stores approved community works and their creators with explicit visibility: The Field Bestiary (archive_bestiary).
### 12 · Incoming system relationships
- Language and Locale Registry (trust_localisation) can use this system because it validates player-generated material before public distribution.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A Vault draft with solvability evidence.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive vault layout workshop view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the creator_vault_builder rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for vault layout workshop: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes challenge design extensible while keeping room logic within approved contracts.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# creator_rule_templates — Game Rule Templates
> Node 4 of 10 in **Player Creation and Community Content** · World Loom identity `creator_rule_templates` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A template library exposes safe configuration knobs for supported game variants without allowing players to rewrite authoritative code.
- Why this belongs in Moonbyte: It permits variation while keeping the Rust engine's rules and permissions in control.
- Distinctive interaction: Choose a supported template, edit bounded options, run compatibility checks and publish a private playtest.
- Successful completion produces: A schema-validated variant definition.
### 02 · Purpose and boundaries
- Product role: Creation tools use constrained templates, provenance, moderation and reproducible tests. Original work can be shared, but no user submission silently becomes trusted game logic.
- This system is the canonical owner of game rule templates behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Player Creation and Community Content; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It permits variation while keeping the Rust engine's rules and permissions in control.
- Stable destination ID: creator_rule_templates; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .creator rule templates and /creator rule templates are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a supported template and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to game rule templates.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A template library exposes safe configuration knobs for supported game variants without allowing players to rewrite authoritative code.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a supported template, edit bounded options, run compatibility checks and publish a private playtest.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Game Rule Templates is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A schema-validated variant definition.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- applies reporting, review, permissions and privacy policy to player-submitted material: Text-First Play Mode (trust_text_fallback).
- allows approved puzzle packs and variants to appear in the curated game library: Friendly Arcade League (arcade_league).
- supports authored side stories and scenario prompts through bounded dialogue templates: Story Remix Paths (quest_remix).
- stores approved community works and their creators with explicit visibility: The Relic Archive (archive_relics).
### 12 · Incoming system relationships
- Text-First Play Mode (trust_text_fallback) can use this system because it validates player-generated material before public distribution.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A schema-validated variant definition.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive game rule templates view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the creator_rule_templates rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for game rule templates: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It permits variation while keeping the Rust engine's rules and permissions in control.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# creator_deck_builder — Original Card Set Builder
> Node 5 of 10 in **Player Creation and Community Content** · World Loom identity `creator_deck_builder` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A card-set editor creates original thematic collections with fixed sources and non-random acquisition rules.
- Why this belongs in Moonbyte: It adds collection identity without cloning protected card games or using paid pack mechanics.
- Distinctive interaction: Choose approved card layouts, author original descriptions, define guaranteed unlock sources and submit the set.
- Successful completion produces: A content pack with provenance and deterministic unlock conditions.
### 02 · Purpose and boundaries
- Product role: Creation tools use constrained templates, provenance, moderation and reproducible tests. Original work can be shared, but no user submission silently becomes trusted game logic.
- This system is the canonical owner of original card set builder behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Player Creation and Community Content; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It adds collection identity without cloning protected card games or using paid pack mechanics.
- Stable destination ID: creator_deck_builder; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .creator deck builder and /creator deck builder are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose approved card layouts and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to original card set builder.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A card-set editor creates original thematic collections with fixed sources and non-random acquisition rules.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose approved card layouts, author original descriptions, define guaranteed unlock sources and submit the set.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Original Card Set Builder is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A content pack with provenance and deterministic unlock conditions.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- applies reporting, review, permissions and privacy policy to player-submitted material: Moderator Console (trust_moderator_console).
- allows approved puzzle packs and variants to appear in the curated game library: Replay and Review Desk (arcade_replays).
- supports authored side stories and scenario prompts through bounded dialogue templates: The Quest Board (quest_board).
- stores approved community works and their creators with explicit visibility: The Illustrated Compendium (archive_cards).
### 12 · Incoming system relationships
- Moderator Console (trust_moderator_console) can use this system because it validates player-generated material before public distribution.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A content pack with provenance and deterministic unlock conditions.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive original card set builder view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the creator_deck_builder rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for original card set builder: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It adds collection identity without cloning protected card games or using paid pack mechanics.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# creator_pixel_editor — Pixel Sticker Workshop
> Node 6 of 10 in **Player Creation and Community Content** · World Loom identity `creator_pixel_editor` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A bounded pixel editor lets users design small original stickers or room decorations using a limited palette and grid.
- Why this belongs in Moonbyte: It gives the visual world a community art pipeline with predictable output constraints.
- Distinctive interaction: Select a canvas size, draw with accessible controls, validate the palette and export a moderated draft.
- Successful completion produces: An original raster asset with dimensions and attribution.
### 02 · Purpose and boundaries
- Product role: Creation tools use constrained templates, provenance, moderation and reproducible tests. Original work can be shared, but no user submission silently becomes trusted game logic.
- This system is the canonical owner of pixel sticker workshop behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Player Creation and Community Content; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives the visual world a community art pipeline with predictable output constraints.
- Stable destination ID: creator_pixel_editor; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .creator pixel editor and /creator pixel editor are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select a canvas size and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to pixel sticker workshop.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A bounded pixel editor lets users design small original stickers or room decorations using a limited palette and grid.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select a canvas size, draw with accessible controls, validate the palette and export a moderated draft.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Pixel Sticker Workshop is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: An original raster asset with dimensions and attribution.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- applies reporting, review, permissions and privacy policy to player-submitted material: Report and Review Pipeline (trust_reports).
- allows approved puzzle packs and variants to appear in the curated game library: Arcade Control Profiles (arcade_accessibility).
- supports authored side stories and scenario prompts through bounded dialogue templates: Chapter Arcs (quest_arcs).
- stores approved community works and their creators with explicit visibility: The Museum Hall (archive_museum).
### 12 · Incoming system relationships
- Report and Review Pipeline (trust_reports) can use this system because it validates player-generated material before public distribution.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: An original raster asset with dimensions and attribution.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive pixel sticker workshop view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the creator_pixel_editor rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for pixel sticker workshop: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives the visual world a community art pipeline with predictable output constraints.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# creator_playtest — Community Playtest Queue
> Node 7 of 10 in **Player Creation and Community Content** · World Loom identity `creator_playtest` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Creators can invite opt-in testers to a private draft session and collect structured feedback without public ranking.
- Why this belongs in Moonbyte: It improves content quality and tests edge cases before publication.
- Distinctive interaction: Submit a draft, set the tester scope, run a bounded playtest, record feedback and revise the version.
- Successful completion produces: A private test report and draft revision history.
### 02 · Purpose and boundaries
- Product role: Creation tools use constrained templates, provenance, moderation and reproducible tests. Original work can be shared, but no user submission silently becomes trusted game logic.
- This system is the canonical owner of community playtest queue behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Player Creation and Community Content; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It improves content quality and tests edge cases before publication.
- Stable destination ID: creator_playtest; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .creator playtest and /creator playtest are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Submit a draft and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to community playtest queue.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Creators can invite opt-in testers to a private draft session and collect structured feedback without public ranking.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Submit a draft, set the tester scope, run a bounded playtest, record feedback and revise the version.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Community Playtest Queue is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A private test report and draft revision history.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- applies reporting, review, permissions and privacy policy to player-submitted material: Privacy and Data Center (trust_privacy_center).
- allows approved puzzle packs and variants to appear in the curated game library: Curated Game Routes (arcade_playlists).
- supports authored side stories and scenario prompts through bounded dialogue templates: Choice and Consequence Graph (quest_branches).
- stores approved community works and their creators with explicit visibility: Collection Sets (archive_sets).
### 12 · Incoming system relationships
- Privacy and Data Center (trust_privacy_center) can use this system because it validates player-generated material before public distribution.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A private test report and draft revision history.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive community playtest queue view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the creator_playtest rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for community playtest queue: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It improves content quality and tests edge cases before publication.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# creator_review — Content Review Pipeline
> Node 8 of 10 in **Player Creation and Community Content** · World Loom identity `creator_review` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Every public creation passes automated validation and a human review path appropriate to its content and visibility.
- Why this belongs in Moonbyte: It protects players from spam, copied material and unsafe submissions.
- Distinctive interaction: Check metadata, run validators, review reports, approve or request edits and record the decision.
- Successful completion produces: A moderation state, reviewer reason and publication gate.
### 02 · Purpose and boundaries
- Product role: Creation tools use constrained templates, provenance, moderation and reproducible tests. Original work can be shared, but no user submission silently becomes trusted game logic.
- This system is the canonical owner of content review pipeline behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Player Creation and Community Content; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It protects players from spam, copied material and unsafe submissions.
- Stable destination ID: creator_review; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .creator review and /creator review are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Check metadata and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to content review pipeline.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Every public creation passes automated validation and a human review path appropriate to its content and visibility.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Check metadata, run validators, review reports, approve or request edits and record the decision.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Content Review Pipeline is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A moderation state, reviewer reason and publication gate.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- applies reporting, review, permissions and privacy policy to player-submitted material: Quiet Notification Rules (trust_notification_rules).
- allows approved puzzle packs and variants to appear in the curated game library: The Pocket Arcade (arcade_library).
- supports authored side stories and scenario prompts through bounded dialogue templates: The Investigation Desk (quest_mysteries).
- stores approved community works and their creators with explicit visibility: The Cosmetic Gallery (archive_cosmetics).
### 12 · Incoming system relationships
- Quiet Notification Rules (trust_notification_rules) can use this system because it validates player-generated material before public distribution.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A moderation state, reviewer reason and publication gate.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive content review pipeline view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the creator_review rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for content review pipeline: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It protects players from spam, copied material and unsafe submissions.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# creator_attribution — Creator Credit Ledger
> Node 9 of 10 in **Player Creation and Community Content** · World Loom identity `creator_attribution` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Approved content includes original author credit, version, source assets and permitted reuse terms.
- Why this belongs in Moonbyte: It builds trust and encourages original work without confusing ownership.
- Distinctive interaction: Inspect a content package, verify its author and assets, accept the listed use terms and publish with attribution.
- Successful completion produces: A provenance record and a clear rights statement.
### 02 · Purpose and boundaries
- Product role: Creation tools use constrained templates, provenance, moderation and reproducible tests. Original work can be shared, but no user submission silently becomes trusted game logic.
- This system is the canonical owner of creator credit ledger behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Player Creation and Community Content; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It builds trust and encourages original work without confusing ownership.
- Stable destination ID: creator_attribution; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .creator attribution and /creator attribution are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect a content package and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to creator credit ledger.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Approved content includes original author credit, version, source assets and permitted reuse terms.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect a content package, verify its author and assets, accept the listed use terms and publish with attribution.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Creator Credit Ledger is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A provenance record and a clear rights statement.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- applies reporting, review, permissions and privacy policy to player-submitted material: Session Reliability Desk (trust_reliability).
- allows approved puzzle packs and variants to appear in the curated game library: Optional Daily Logic Desk (arcade_daily_logic).
- supports authored side stories and scenario prompts through bounded dialogue templates: Dialogue Routes (quest_dialogue).
- stores approved community works and their creators with explicit visibility: The Living Lore Index (archive_lore).
### 12 · Incoming system relationships
- Session Reliability Desk (trust_reliability) can use this system because it validates player-generated material before public distribution.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A provenance record and a clear rights statement.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive creator credit ledger view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the creator_attribution rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for creator credit ledger: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It builds trust and encourages original work without confusing ownership.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# creator_publishing — Community Pack Publishing
> Node 10 of 10 in **Player Creation and Community Content** · World Loom identity `creator_publishing` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Approved puzzles, stories, decorations and event kits can be grouped into discoverable packs with explicit compatibility.
- Why this belongs in Moonbyte: It makes good community content reusable across servers without bypassing moderation.
- Distinctive interaction: Select approved entries, declare dependencies, preview the pack, publish a version and track corrections.
- Successful completion produces: A versioned content pack with safe dependency metadata.
### 02 · Purpose and boundaries
- Product role: Creation tools use constrained templates, provenance, moderation and reproducible tests. Original work can be shared, but no user submission silently becomes trusted game logic.
- This system is the canonical owner of community pack publishing behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Player Creation and Community Content; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes good community content reusable across servers without bypassing moderation.
- Stable destination ID: creator_publishing; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .creator publishing and /creator publishing are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select approved entries and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to community pack publishing.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Approved puzzles, stories, decorations and event kits can be grouped into discoverable packs with explicit compatibility.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select approved entries, declare dependencies, preview the pack, publish a version and track corrections.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Community Pack Publishing is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned content pack with safe dependency metadata.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- applies reporting, review, permissions and privacy policy to player-submitted material: World Content Policy (trust_content_policy).
- allows approved puzzle packs and variants to appear in the curated game library: The Practice School (arcade_skill_school).
- supports authored side stories and scenario prompts through bounded dialogue templates: Faction Storylines (quest_factions).
- stores approved community works and their creators with explicit visibility: Collection Milestones (archive_milestones).
### 12 · Incoming system relationships
- World Content Policy (trust_content_policy) can use this system because it validates player-generated material before public distribution.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A versioned content pack with safe dependency metadata.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive community pack publishing view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the creator_publishing rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for community pack publishing: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes good community content reusable across servers without bypassing moderation.; it must remain satisfying in a bounded session and support a natural stopping point.
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

## Domain 18 — Fair Play, Competition and Verification

Fairness provides deterministic rules, auditable outcomes and honest scoreboards. It never converts chance into tradable value or a wager.

**Domain dependencies:** Parties, Lobbies and Multiplayer Rules (`multiplayer`) · The Bazaar and Exchange (`bazaar`) · Arcade and Original Mini-Games (`arcade`) · Vaults, Dungeons and Puzzle Spaces (`vault`)

# fairness_rulebooks — Published Rulebooks
> Node 1 of 10 in **Fair Play, Competition and Verification** · World Loom identity `fairness_rulebooks` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Every competitive game has a public rules version, legal-action definition, scoring model and tie resolution.
- Why this belongs in Moonbyte: It lets players understand what counts before competing and prevents silent changes.
- Distinctive interaction: Open a game card, inspect the current rules version, compare revisions and start only after the terms are clear.
- Successful completion produces: A versioned rulebook linked to sessions and replays.
### 02 · Purpose and boundaries
- Product role: Fairness provides deterministic rules, auditable outcomes and honest scoreboards. It never converts chance into tradable value or a wager.
- This system is the canonical owner of published rulebooks behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Fair Play, Competition and Verification; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It lets players understand what counts before competing and prevents silent changes.
- Stable destination ID: fairness_rulebooks; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fairness rulebooks and /fairness rulebooks are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open a game card and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to published rulebooks.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Every competitive game has a public rules version, legal-action definition, scoring model and tie resolution.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open a game card, inspect the current rules version, compare revisions and start only after the terms are clear.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Published Rulebooks is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned rulebook linked to sessions and replays.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- ensures turn order, scoring and participant contributions are reproducible: Universal Lobby Contract (multiplayer_lobby).
- uses atomic ledgers and receipts for every trade and escrow transition: The Wanted Board (bazaar_wanted_board).
- allows players to reproduce relevant game results and puzzle score calculations: Friendly Arcade League (arcade_league).
- keeps challenge generation and fixed rewards auditable: Verified Puzzle Registry (vault_solution_registry).
### 12 · Incoming system relationships
- Player Stallfronts (bazaar_stallfronts) can use this system because it uses atomic escrow, append-only receipts and deterministic ledger rules.
- Logic Key Mechanisms (vault_logic_keys) can use this system because it uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed.
- The Party Game Shelf (arcade_party_collection) can use this system because it documents random inputs and competitive scoring where verification is useful.
- Friendly Tournament Brackets (multiplayer_brackets) can use this system because it uses documented rules and verifiable transcripts for supported competitive outcomes.
### 13 · Reward and economy contract
- Reward policy: A versioned rulebook linked to sessions and replays.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive published rulebooks view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fairness_rulebooks rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for published rulebooks: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It lets players understand what counts before competing and prevents silent changes.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fairness_commit_reveal — Commit and Reveal Service
> Node 2 of 10 in **Fair Play, Competition and Verification** · World Loom identity `fairness_commit_reveal` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Eligible randomised score-only sessions can publish a SHA-512 commitment before play and reveal the seed and transcript afterward.
- Why this belongs in Moonbyte: It enables independent verification while keeping cryptographic hashing distinct from entropy generation.
- Distinctive interaction: Generate OS-backed secret entropy, publish commitment, freeze inputs, derive result with unbiased mapping and reveal transcript.
- Successful completion produces: A verifiable transcript with documented derivation and no economy payout.
### 02 · Purpose and boundaries
- Product role: Fairness provides deterministic rules, auditable outcomes and honest scoreboards. It never converts chance into tradable value or a wager.
- This system is the canonical owner of commit and reveal service behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Fair Play, Competition and Verification; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It enables independent verification while keeping cryptographic hashing distinct from entropy generation.
- Stable destination ID: fairness_commit_reveal; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fairness commit reveal and /fairness commit reveal are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Generate OS-backed secret entropy and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to commit and reveal service.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Eligible randomised score-only sessions can publish a SHA-512 commitment before play and reveal the seed and transcript afterward.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Generate OS-backed secret entropy, publish commitment, freeze inputs, derive result with unbiased mapping and reveal transcript.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Commit and Reveal Service is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A verifiable transcript with documented derivation and no economy payout.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- ensures turn order, scoring and participant contributions are reproducible: Party Formation Desk (multiplayer_party).
- uses atomic ledgers and receipts for every trade and escrow transition: Crafting Work Orders (bazaar_work_orders).
- allows players to reproduce relevant game results and puzzle score calculations: Replay and Review Desk (arcade_replays).
- keeps challenge generation and fixed rewards auditable: Vault Blueprint Library (vault_blueprints).
### 12 · Incoming system relationships
- Fixed-Price Listings (bazaar_fixed_listings) can use this system because it uses atomic escrow, append-only receipts and deterministic ledger rules.
- Relic Galleries (vault_relic_rooms) can use this system because it uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed.
- Asynchronous Challenge Cards (arcade_async_challenges) can use this system because it documents random inputs and competitive scoring where verification is useful.
- Shared Cooperative Boards (multiplayer_coop_boards) can use this system because it uses documented rules and verifiable transcripts for supported competitive outcomes.
### 13 · Reward and economy contract
- Reward policy: A verifiable transcript with documented derivation and no economy payout.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive commit and reveal service view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fairness_commit_reveal rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for commit and reveal service: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It enables independent verification while keeping cryptographic hashing distinct from entropy generation.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fairness_scoring — Scoring Contract Registry
> Node 3 of 10 in **Fair Play, Competition and Verification** · World Loom identity `fairness_scoring` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Each score metric declares its inputs, range, aggregation, tie handling and supported rules versions.
- Why this belongs in Moonbyte: It prevents leaderboards from comparing unlike sessions or hidden score changes.
- Distinctive interaction: Resolve a match, calculate from canonical events, apply the declared tie rule and save the score evidence.
- Successful completion produces: A deterministic score record and rules reference.
### 02 · Purpose and boundaries
- Product role: Fairness provides deterministic rules, auditable outcomes and honest scoreboards. It never converts chance into tradable value or a wager.
- This system is the canonical owner of scoring contract registry behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Fair Play, Competition and Verification; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It prevents leaderboards from comparing unlike sessions or hidden score changes.
- Stable destination ID: fairness_scoring; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fairness scoring and /fairness scoring are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Resolve a match and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to scoring contract registry.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Each score metric declares its inputs, range, aggregation, tie handling and supported rules versions.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Resolve a match, calculate from canonical events, apply the declared tie rule and save the score evidence.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Scoring Contract Registry is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A deterministic score record and rules reference.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- ensures turn order, scoring and participant contributions are reproducible: Friend-First Match Finder (multiplayer_matchmaking).
- uses atomic ledgers and receipts for every trade and escrow transition: The Price Reference Ledger (bazaar_price_ledger).
- allows players to reproduce relevant game results and puzzle score calculations: Arcade Control Profiles (arcade_accessibility).
- keeps challenge generation and fixed rewards auditable: Puzzle Chambers (vault_chambers).
### 12 · Incoming system relationships
- The Barter Desk (bazaar_barter_desk) can use this system because it uses atomic escrow, append-only receipts and deterministic ledger rules.
- Mastery Trial Rooms (vault_mastery_trials) can use this system because it uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed.
- Friendly Arcade League (arcade_league) can use this system because it documents random inputs and competitive scoring where verification is useful.
- Reconnect and Rejoin (multiplayer_reconnect) can use this system because it uses documented rules and verifiable transcripts for supported competitive outcomes.
### 13 · Reward and economy contract
- Reward policy: A deterministic score record and rules reference.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive scoring contract registry view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fairness_scoring rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for scoring contract registry: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It prevents leaderboards from comparing unlike sessions or hidden score changes.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fairness_ratings — Optional Skill Rating
> Node 4 of 10 in **Fair Play, Competition and Verification** · World Loom identity `fairness_ratings` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A rating model can estimate competitive skill for compatible game modes while exposing uncertainty and allowing players to opt out.
- Why this belongs in Moonbyte: It creates fairer match groupings without presenting rating as personal worth or a universal ranking.
- Distinctive interaction: Play eligible matches, update using a documented model, inspect uncertainty and hide or reset the visible rating where supported.
- Successful completion produces: A bounded rating record and model version.
### 02 · Purpose and boundaries
- Product role: Fairness provides deterministic rules, auditable outcomes and honest scoreboards. It never converts chance into tradable value or a wager.
- This system is the canonical owner of optional skill rating behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Fair Play, Competition and Verification; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates fairer match groupings without presenting rating as personal worth or a universal ranking.
- Stable destination ID: fairness_ratings; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fairness ratings and /fairness ratings are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Play eligible matches and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to optional skill rating.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A rating model can estimate competitive skill for compatible game modes while exposing uncertainty and allowing players to opt out.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Play eligible matches, update using a documented model, inspect uncertainty and hide or reset the visible rating where supported.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Optional Skill Rating is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A bounded rating record and model version.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- ensures turn order, scoring and participant contributions are reproducible: Shared Turn Engine (multiplayer_turn_engine).
- uses atomic ledgers and receipts for every trade and escrow transition: The Escrow Vault (bazaar_escrow_vault).
- allows players to reproduce relevant game results and puzzle score calculations: Curated Game Routes (arcade_playlists).
- keeps challenge generation and fixed rewards auditable: Logic Key Mechanisms (vault_logic_keys).
### 12 · Incoming system relationships
- The Wanted Board (bazaar_wanted_board) can use this system because it uses atomic escrow, append-only receipts and deterministic ledger rules.
- The Grand Mechanism (vault_boss_logic) can use this system because it uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed.
- Replay and Review Desk (arcade_replays) can use this system because it documents random inputs and competitive scoring where verification is useful.
- Table Votes and Host Controls (multiplayer_table_votes) can use this system because it uses documented rules and verifiable transcripts for supported competitive outcomes.
### 13 · Reward and economy contract
- Reward policy: A bounded rating record and model version.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive optional skill rating view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fairness_ratings rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for optional skill rating: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates fairer match groupings without presenting rating as personal worth or a universal ranking.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fairness_match_review — Match Review Transcript
> Node 5 of 10 in **Fair Play, Competition and Verification** · World Loom identity `fairness_match_review` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Participants can inspect a redacted timeline of accepted actions, phase changes, score calculations and disconnect handling.
- Why this belongs in Moonbyte: It gives players evidence when a result is questioned and supports learning.
- Distinctive interaction: Open a match, choose public-safe details, compare events with the rulebook and request review if inconsistent.
- Successful completion produces: A replayable audit timeline with private information filtered.
### 02 · Purpose and boundaries
- Product role: Fairness provides deterministic rules, auditable outcomes and honest scoreboards. It never converts chance into tradable value or a wager.
- This system is the canonical owner of match review transcript behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Fair Play, Competition and Verification; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives players evidence when a result is questioned and supports learning.
- Stable destination ID: fairness_match_review; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fairness match review and /fairness match review are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open a match and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to match review transcript.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Participants can inspect a redacted timeline of accepted actions, phase changes, score calculations and disconnect handling.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open a match, choose public-safe details, compare events with the rulebook and request review if inconsistent.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Match Review Transcript is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A replayable audit timeline with private information filtered.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- ensures turn order, scoring and participant contributions are reproducible: Simultaneous Choice Windows (multiplayer_simultaneous).
- uses atomic ledgers and receipts for every trade and escrow transition: Item Provenance Cards (bazaar_provenance).
- allows players to reproduce relevant game results and puzzle score calculations: The Pocket Arcade (arcade_library).
- keeps challenge generation and fixed rewards auditable: Relic Galleries (vault_relic_rooms).
### 12 · Incoming system relationships
- Crafting Work Orders (bazaar_work_orders) can use this system because it uses atomic escrow, append-only receipts and deterministic ledger rules.
- Split-Path Chambers (vault_parallel_rooms) can use this system because it uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed.
- Arcade Control Profiles (arcade_accessibility) can use this system because it documents random inputs and competitive scoring where verification is useful.
- Universal Lobby Contract (multiplayer_lobby) can use this system because it uses documented rules and verifiable transcripts for supported competitive outcomes.
### 13 · Reward and economy contract
- Reward policy: A replayable audit timeline with private information filtered.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive match review transcript view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fairness_match_review rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for match review transcript: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives players evidence when a result is questioned and supports learning.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fairness_anti_abuse — Integrity Guardrails
> Node 6 of 10 in **Fair Play, Competition and Verification** · World Loom identity `fairness_anti_abuse` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Rate limits, action idempotency, anomaly flags and transaction constraints prevent duplicate rewards and exploit loops.
- Why this belongs in Moonbyte: It protects player trust without invasive behavioural profiling.
- Distinctive interaction: Validate a request, check only necessary counters, reject unsafe duplicates and record a minimal reason code.
- Successful completion produces: A bounded integrity event and safe rejection response.
### 02 · Purpose and boundaries
- Product role: Fairness provides deterministic rules, auditable outcomes and honest scoreboards. It never converts chance into tradable value or a wager.
- This system is the canonical owner of integrity guardrails behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Fair Play, Competition and Verification; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It protects player trust without invasive behavioural profiling.
- Stable destination ID: fairness_anti_abuse; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fairness anti abuse and /fairness anti abuse are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Validate a request and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to integrity guardrails.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Rate limits, action idempotency, anomaly flags and transaction constraints prevent duplicate rewards and exploit loops.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Validate a request, check only necessary counters, reject unsafe duplicates and record a minimal reason code.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Integrity Guardrails is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A bounded integrity event and safe rejection response.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- ensures turn order, scoring and participant contributions are reproducible: Spectator Gallery (multiplayer_spectators).
- uses atomic ledgers and receipts for every trade and escrow transition: The Delivery Board (bazaar_delivery_board).
- allows players to reproduce relevant game results and puzzle score calculations: Optional Daily Logic Desk (arcade_daily_logic).
- keeps challenge generation and fixed rewards auditable: Mastery Trial Rooms (vault_mastery_trials).
### 12 · Incoming system relationships
- The Price Reference Ledger (bazaar_price_ledger) can use this system because it uses atomic escrow, append-only receipts and deterministic ledger rules.
- Vault Save Stones (vault_checkpoints) can use this system because it uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed.
- Curated Game Routes (arcade_playlists) can use this system because it documents random inputs and competitive scoring where verification is useful.
- Party Formation Desk (multiplayer_party) can use this system because it uses documented rules and verifiable transcripts for supported competitive outcomes.
### 13 · Reward and economy contract
- Reward policy: A bounded integrity event and safe rejection response.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive integrity guardrails view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fairness_anti_abuse rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for integrity guardrails: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It protects player trust without invasive behavioural profiling.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fairness_tournament — Tournament Rules Desk
> Node 7 of 10 in **Fair Play, Competition and Verification** · World Loom identity `fairness_tournament` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Tournament organisers configure bracket size, match format, dispute window and cosmetic recognition under one published policy.
- Why this belongs in Moonbyte: It supports structured play without entry fees, cash prizes, stakes or pooled currency.
- Distinctive interaction: Publish rules, register teams, schedule matches, resolve rounds and archive the final bracket.
- Successful completion produces: A deterministic tournament record with no wager or prize-pool data model.
### 02 · Purpose and boundaries
- Product role: Fairness provides deterministic rules, auditable outcomes and honest scoreboards. It never converts chance into tradable value or a wager.
- This system is the canonical owner of tournament rules desk behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Fair Play, Competition and Verification; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports structured play without entry fees, cash prizes, stakes or pooled currency.
- Stable destination ID: fairness_tournament; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fairness tournament and /fairness tournament are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Publish rules and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to tournament rules desk.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Tournament organisers configure bracket size, match format, dispute window and cosmetic recognition under one published policy.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Publish rules, register teams, schedule matches, resolve rounds and archive the final bracket.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Tournament Rules Desk is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A deterministic tournament record with no wager or prize-pool data model.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- ensures turn order, scoring and participant contributions are reproducible: Friendly Tournament Brackets (multiplayer_brackets).
- uses atomic ledgers and receipts for every trade and escrow transition: The Trade Review Desk (bazaar_review_court).
- allows players to reproduce relevant game results and puzzle score calculations: The Practice School (arcade_skill_school).
- keeps challenge generation and fixed rewards auditable: The Grand Mechanism (vault_boss_logic).
### 12 · Incoming system relationships
- The Escrow Vault (bazaar_escrow_vault) can use this system because it uses atomic escrow, append-only receipts and deterministic ledger rules.
- Fair Difficulty Profiles (vault_difficulty) can use this system because it uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed.
- The Pocket Arcade (arcade_library) can use this system because it documents random inputs and competitive scoring where verification is useful.
- Friend-First Match Finder (multiplayer_matchmaking) can use this system because it uses documented rules and verifiable transcripts for supported competitive outcomes.
### 13 · Reward and economy contract
- Reward policy: A deterministic tournament record with no wager or prize-pool data model.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive tournament rules desk view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fairness_tournament rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for tournament rules desk: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports structured play without entry fees, cash prizes, stakes or pooled currency.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fairness_leaderboard — Leaderboard Integrity
> Node 8 of 10 in **Fair Play, Competition and Verification** · World Loom identity `fairness_leaderboard` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A leaderboard stores score provenance, rules version, eligible mode and tie policy alongside each displayed result.
- Why this belongs in Moonbyte: It makes rankings explainable and robust against forged or incomparable submissions.
- Distinctive interaction: Select a scope, filter compatible results, inspect the rules and report a specific record for review.
- Successful completion produces: A rules-consistent leaderboard snapshot.
### 02 · Purpose and boundaries
- Product role: Fairness provides deterministic rules, auditable outcomes and honest scoreboards. It never converts chance into tradable value or a wager.
- This system is the canonical owner of leaderboard integrity behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Fair Play, Competition and Verification; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes rankings explainable and robust against forged or incomparable submissions.
- Stable destination ID: fairness_leaderboard; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fairness leaderboard and /fairness leaderboard are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select a scope and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to leaderboard integrity.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A leaderboard stores score provenance, rules version, eligible mode and tie policy alongside each displayed result.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select a scope, filter compatible results, inspect the rules and report a specific record for review.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Leaderboard Integrity is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A rules-consistent leaderboard snapshot.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- ensures turn order, scoring and participant contributions are reproducible: Shared Cooperative Boards (multiplayer_coop_boards).
- uses atomic ledgers and receipts for every trade and escrow transition: Player Stallfronts (bazaar_stallfronts).
- allows players to reproduce relevant game results and puzzle score calculations: Puzzle Generator and Validator (arcade_puzzle_factory).
- keeps challenge generation and fixed rewards auditable: Split-Path Chambers (vault_parallel_rooms).
### 12 · Incoming system relationships
- Item Provenance Cards (bazaar_provenance) can use this system because it uses atomic escrow, append-only receipts and deterministic ledger rules.
- Verified Puzzle Registry (vault_solution_registry) can use this system because it uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed.
- Optional Daily Logic Desk (arcade_daily_logic) can use this system because it documents random inputs and competitive scoring where verification is useful.
- Shared Turn Engine (multiplayer_turn_engine) can use this system because it uses documented rules and verifiable transcripts for supported competitive outcomes.
### 13 · Reward and economy contract
- Reward policy: A rules-consistent leaderboard snapshot.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive leaderboard integrity view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fairness_leaderboard rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for leaderboard integrity: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes rankings explainable and robust against forged or incomparable submissions.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fairness_access — Equal Conditions Profiles
> Node 9 of 10 in **Fair Play, Competition and Verification** · World Loom identity `fairness_access` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Supported competitive games can constrain optional assists to equivalent modes or report them as separate categories.
- Why this belongs in Moonbyte: It keeps comparisons fair while preserving accessibility options for everyone.
- Distinctive interaction: Choose a mode, inspect allowed assists, confirm the profile and compare only against the same rules category.
- Successful completion produces: A mode classification and accessible comparison contract.
### 02 · Purpose and boundaries
- Product role: Fairness provides deterministic rules, auditable outcomes and honest scoreboards. It never converts chance into tradable value or a wager.
- This system is the canonical owner of equal conditions profiles behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Fair Play, Competition and Verification; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It keeps comparisons fair while preserving accessibility options for everyone.
- Stable destination ID: fairness_access; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fairness access and /fairness access are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a mode and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to equal conditions profiles.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Supported competitive games can constrain optional assists to equivalent modes or report them as separate categories.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a mode, inspect allowed assists, confirm the profile and compare only against the same rules category.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Equal Conditions Profiles is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A mode classification and accessible comparison contract.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- ensures turn order, scoring and participant contributions are reproducible: Reconnect and Rejoin (multiplayer_reconnect).
- uses atomic ledgers and receipts for every trade and escrow transition: Fixed-Price Listings (bazaar_fixed_listings).
- allows players to reproduce relevant game results and puzzle score calculations: The Party Game Shelf (arcade_party_collection).
- keeps challenge generation and fixed rewards auditable: Vault Save Stones (vault_checkpoints).
### 12 · Incoming system relationships
- The Delivery Board (bazaar_delivery_board) can use this system because it uses atomic escrow, append-only receipts and deterministic ledger rules.
- Vault Blueprint Library (vault_blueprints) can use this system because it uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed.
- The Practice School (arcade_skill_school) can use this system because it documents random inputs and competitive scoring where verification is useful.
- Simultaneous Choice Windows (multiplayer_simultaneous) can use this system because it uses documented rules and verifiable transcripts for supported competitive outcomes.
### 13 · Reward and economy contract
- Reward policy: A mode classification and accessible comparison contract.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive equal conditions profiles view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fairness_access rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for equal conditions profiles: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It keeps comparisons fair while preserving accessibility options for everyone.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fairness_appeals — Result Appeal Desk
> Node 10 of 10 in **Fair Play, Competition and Verification** · World Loom identity `fairness_appeals` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A participant can request a review of a specific match result with a reason code and linked evidence.
- Why this belongs in Moonbyte: It adds accountability without automatically overturning a result through popularity or pressure.
- Distinctive interaction: Select a result, identify the disputed rule, submit evidence, receive a review state and inspect the decision.
- Successful completion produces: A private appeal record and reasoned resolution.
### 02 · Purpose and boundaries
- Product role: Fairness provides deterministic rules, auditable outcomes and honest scoreboards. It never converts chance into tradable value or a wager.
- This system is the canonical owner of result appeal desk behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Fair Play, Competition and Verification; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It adds accountability without automatically overturning a result through popularity or pressure.
- Stable destination ID: fairness_appeals; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fairness appeals and /fairness appeals are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select a result and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to result appeal desk.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A participant can request a review of a specific match result with a reason code and linked evidence.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select a result, identify the disputed rule, submit evidence, receive a review state and inspect the decision.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Result Appeal Desk is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A private appeal record and reasoned resolution.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- ensures turn order, scoring and participant contributions are reproducible: Table Votes and Host Controls (multiplayer_table_votes).
- uses atomic ledgers and receipts for every trade and escrow transition: The Barter Desk (bazaar_barter_desk).
- allows players to reproduce relevant game results and puzzle score calculations: Asynchronous Challenge Cards (arcade_async_challenges).
- keeps challenge generation and fixed rewards auditable: Fair Difficulty Profiles (vault_difficulty).
### 12 · Incoming system relationships
- The Trade Review Desk (bazaar_review_court) can use this system because it uses atomic escrow, append-only receipts and deterministic ledger rules.
- Puzzle Chambers (vault_chambers) can use this system because it uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed.
- Puzzle Generator and Validator (arcade_puzzle_factory) can use this system because it documents random inputs and competitive scoring where verification is useful.
- Spectator Gallery (multiplayer_spectators) can use this system because it uses documented rules and verifiable transcripts for supported competitive outcomes.
### 13 · Reward and economy contract
- Reward policy: A private appeal record and reasoned resolution.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive result appeal desk view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fairness_appeals rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for result appeal desk: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It adds accountability without automatically overturning a result through popularity or pressure.; it must remain satisfying in a bounded session and support a natural stopping point.
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

## Domain 19 — Friends, Community Bonds and Social Play

Social systems help people cooperate, create and remember shared moments. They remain consent-based, scoped and easy to mute or leave.

**Domain dependencies:** The Hearth and Tavern (`hearth`) · Guilds and Long-Form Cooperation (`guild`) · Parties, Lobbies and Multiplayer Rules (`multiplayer`) · Festivals, Chapters and World Events (`festival`)

# fellowship_friends — Friend Circle
> Node 1 of 10 in **Friends, Community Bonds and Social Play** · World Loom identity `fellowship_friends` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can maintain a private list of known Moonbyte friends with clear invitation, block and removal controls.
- Why this belongs in Moonbyte: It supports repeat cooperation without a public social graph or unsolicited contact surface.
- Distinctive interaction: Send an invitation through an allowed Discord context, accept or decline, choose sharing scope and revoke anytime.
- Successful completion produces: A minimal friendship relation with explicit visibility settings.
### 02 · Purpose and boundaries
- Product role: Social systems help people cooperate, create and remember shared moments. They remain consent-based, scoped and easy to mute or leave.
- This system is the canonical owner of friend circle behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Friends, Community Bonds and Social Play; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports repeat cooperation without a public social graph or unsolicited contact surface.
- Stable destination ID: fellowship_friends; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fellowship friends and /fellowship friends are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Send an invitation through an allowed Discord context and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to friend circle.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can maintain a private list of known Moonbyte friends with clear invitation, block and removal controls.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Send an invitation through an allowed Discord context, accept or decline, choose sharing scope and revoke anytime.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Friend Circle is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A minimal friendship relation with explicit visibility settings.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses the Tavern as a familiar shared meeting point: The Lantern and Ladle (hearth_main_room).
- lets friends form or join structured long-running groups: The Great Works Board (guild_projects).
- uses the universal lobby, turn and reconnect contracts: Friendly Tournament Brackets (multiplayer_brackets).
- connects shared milestones to optional community events: Moonbyte Anniversary (festival_anniversary).
### 12 · Incoming system relationships
- The Lantern and Ladle (hearth_main_room) can use this system because it gives friends and parties a shared meeting point without requiring a public conversation.
- The Practice School (arcade_skill_school) can use this system because it supports friend challenges and shared play with clear exit and privacy controls.
- Simultaneous Choice Windows (multiplayer_simultaneous) can use this system because it provides friend invitations, opt-in groups and cooperative roles.
- Equipment Ledger (character_equipment) can use this system because it gives parties complementary utility without requiring one optimal composition.
### 13 · Reward and economy contract
- Reward policy: A minimal friendship relation with explicit visibility settings.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive friend circle view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fellowship_friends rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for friend circle: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports repeat cooperation without a public social graph or unsolicited contact surface.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fellowship_roles — Cooperative Role Planner
> Node 2 of 10 in **Friends, Community Bonds and Social Play** · World Loom identity `fellowship_roles` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A party can assign optional roles such as clue keeper, map reader, builder and coordinator before a shared session.
- Why this belongs in Moonbyte: It helps groups divide work and makes quieter contributions visible.
- Distinctive interaction: Inspect the activity, choose a role, review its actions, change before launch and contribute during the session.
- Successful completion produces: A party role map linked to the session.
### 02 · Purpose and boundaries
- Product role: Social systems help people cooperate, create and remember shared moments. They remain consent-based, scoped and easy to mute or leave.
- This system is the canonical owner of cooperative role planner behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Friends, Community Bonds and Social Play; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It helps groups divide work and makes quieter contributions visible.
- Stable destination ID: fellowship_roles; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fellowship roles and /fellowship roles are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect the activity and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to cooperative role planner.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A party can assign optional roles such as clue keeper, map reader, builder and coordinator before a shared session.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect the activity, choose a role, review its actions, change before launch and contribute during the session.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Cooperative Role Planner is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A party role map linked to the session.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses the Tavern as a familiar shared meeting point: The Notice Wall (hearth_notice_wall).
- lets friends form or join structured long-running groups: Guild Story Contracts (guild_quests).
- uses the universal lobby, turn and reconnect contracts: Shared Cooperative Boards (multiplayer_coop_boards).
- connects shared milestones to optional community events: World Fairs (festival_world_fairs).
### 12 · Incoming system relationships
- The Notice Wall (hearth_notice_wall) can use this system because it gives friends and parties a shared meeting point without requiring a public conversation.
- Puzzle Generator and Validator (arcade_puzzle_factory) can use this system because it supports friend challenges and shared play with clear exit and privacy controls.
- Spectator Gallery (multiplayer_spectators) can use this system because it provides friend invitations, opt-in groups and cooperative roles.
- Titles and Identity Marks (character_titles) can use this system because it gives parties complementary utility without requiring one optimal composition.
### 13 · Reward and economy contract
- Reward policy: A party role map linked to the session.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive cooperative role planner view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fellowship_roles rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for cooperative role planner: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It helps groups divide work and makes quieter contributions visible.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fellowship_mentor — Opt-In Mentor Sessions
> Node 3 of 10 in **Friends, Community Bonds and Social Play** · World Loom identity `fellowship_mentor` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Experienced players can host short lessons for a chosen game with an explicit topic and no follower or popularity reward.
- Why this belongs in Moonbyte: It supports learning while avoiding pressure, unsolicited messaging and hierarchy abuse.
- Distinctive interaction: Publish a lesson slot, allow interested players to opt in, run the practice session and close invitations.
- Successful completion produces: A lesson record and structured feedback with blocking/reporting controls.
### 02 · Purpose and boundaries
- Product role: Social systems help people cooperate, create and remember shared moments. They remain consent-based, scoped and easy to mute or leave.
- This system is the canonical owner of opt-in mentor sessions behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Friends, Community Bonds and Social Play; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports learning while avoiding pressure, unsolicited messaging and hierarchy abuse.
- Stable destination ID: fellowship_mentor; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fellowship mentor and /fellowship mentor are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Publish a lesson slot and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to opt-in mentor sessions.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Experienced players can host short lessons for a chosen game with an explicit topic and no follower or popularity reward.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Publish a lesson slot, allow interested players to opt in, run the practice session and close invitations.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Opt-In Mentor Sessions is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A lesson record and structured feedback with blocking/reporting controls.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses the Tavern as a familiar shared meeting point: The Rumour Circle (hearth_rumour_circle).
- lets friends form or join structured long-running groups: The Guild Vault (guild_vault).
- uses the universal lobby, turn and reconnect contracts: Reconnect and Rejoin (multiplayer_reconnect).
- connects shared milestones to optional community events: Seasonal Story Chapters (festival_chapters).
### 12 · Incoming system relationships
- The Rumour Circle (hearth_rumour_circle) can use this system because it gives friends and parties a shared meeting point without requiring a public conversation.
- The Party Game Shelf (arcade_party_collection) can use this system because it supports friend challenges and shared play with clear exit and privacy controls.
- Friendly Tournament Brackets (multiplayer_brackets) can use this system because it provides friend invitations, opt-in groups and cooperative roles.
- Pixel Persona Studio (character_appearance) can use this system because it gives parties complementary utility without requiring one optimal composition.
### 13 · Reward and economy contract
- Reward policy: A lesson record and structured feedback with blocking/reporting controls.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive opt-in mentor sessions view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fellowship_mentor rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for opt-in mentor sessions: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports learning while avoiding pressure, unsolicited messaging and hierarchy abuse.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fellowship_clubs — Interest Clubs
> Node 4 of 10 in **Friends, Community Bonds and Social Play** · World Loom identity `fellowship_clubs` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can join small opt-in clubs around puzzles, cartography, crafting, stories or particular games.
- Why this belongs in Moonbyte: It creates social discovery beyond a single server while keeping membership visible only as chosen.
- Distinctive interaction: Browse club rules, preview its scope, request membership, join and leave from one control.
- Successful completion produces: A club membership with moderation and privacy settings.
### 02 · Purpose and boundaries
- Product role: Social systems help people cooperate, create and remember shared moments. They remain consent-based, scoped and easy to mute or leave.
- This system is the canonical owner of interest clubs behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Friends, Community Bonds and Social Play; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates social discovery beyond a single server while keeping membership visible only as chosen.
- Stable destination ID: fellowship_clubs; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fellowship clubs and /fellowship clubs are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Browse club rules and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to interest clubs.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can join small opt-in clubs around puzzles, cartography, crafting, stories or particular games.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Browse club rules, preview its scope, request membership, join and leave from one control.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Interest Clubs is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A club membership with moderation and privacy settings.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses the Tavern as a familiar shared meeting point: The Common Tables (hearth_table_games).
- lets friends form or join structured long-running groups: The Guild Cartography Room (guild_map).
- uses the universal lobby, turn and reconnect contracts: Table Votes and Host Controls (multiplayer_table_votes).
- connects shared milestones to optional community events: Server Celebration Kits (festival_server_events).
### 12 · Incoming system relationships
- The Common Tables (hearth_table_games) can use this system because it gives friends and parties a shared meeting point without requiring a public conversation.
- Asynchronous Challenge Cards (arcade_async_challenges) can use this system because it supports friend challenges and shared play with clear exit and privacy controls.
- Shared Cooperative Boards (multiplayer_coop_boards) can use this system because it provides friend invitations, opt-in groups and cooperative roles.
- Non-Combat Talents (character_utility) can use this system because it gives parties complementary utility without requiring one optimal composition.
### 13 · Reward and economy contract
- Reward policy: A club membership with moderation and privacy settings.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive interest clubs view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fellowship_clubs rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for interest clubs: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates social discovery beyond a single server while keeping membership visible only as chosen.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fellowship_gifting — Thoughtful Gift Exchange
> Node 5 of 10 in **Friends, Community Bonds and Social Play** · World Loom identity `fellowship_gifting` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players may send eligible account-safe cosmetics or low-impact crafted items under strict frequency and provenance rules.
- Why this belongs in Moonbyte: It supports generosity without currency farming or a cash-equivalent gift market.
- Distinctive interaction: Choose an eligible gift, inspect recipient and item, confirm once and provide a receipt to both parties.
- Successful completion produces: A one-way gift record with duplicate and self-gift protections.
### 02 · Purpose and boundaries
- Product role: Social systems help people cooperate, create and remember shared moments. They remain consent-based, scoped and easy to mute or leave.
- This system is the canonical owner of thoughtful gift exchange behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Friends, Community Bonds and Social Play; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports generosity without currency farming or a cash-equivalent gift market.
- Stable destination ID: fellowship_gifting; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fellowship gifting and /fellowship gifting are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose an eligible gift and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to thoughtful gift exchange.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players may send eligible account-safe cosmetics or low-impact crafted items under strict frequency and provenance rules.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose an eligible gift, inspect recipient and item, confirm once and provide a receipt to both parties.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Thoughtful Gift Exchange is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A one-way gift record with duplicate and self-gift protections.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses the Tavern as a familiar shared meeting point: The Host's Bench (hearth_hosts_bench).
- lets friends form or join structured long-running groups: Guild Neighbourhoods (guild_alliances).
- uses the universal lobby, turn and reconnect contracts: Universal Lobby Contract (multiplayer_lobby).
- connects shared milestones to optional community events: World Restoration Projects (festival_world_projects).
### 12 · Incoming system relationships
- The Host's Bench (hearth_hosts_bench) can use this system because it gives friends and parties a shared meeting point without requiring a public conversation.
- Friendly Arcade League (arcade_league) can use this system because it supports friend challenges and shared play with clear exit and privacy controls.
- Reconnect and Rejoin (multiplayer_reconnect) can use this system because it provides friend invitations, opt-in groups and cooperative roles.
- Player Character Sheet (character_identity) can use this system because it gives parties complementary utility without requiring one optimal composition.
### 13 · Reward and economy contract
- Reward policy: A one-way gift record with duplicate and self-gift protections.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive thoughtful gift exchange view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fellowship_gifting rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for thoughtful gift exchange: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports generosity without currency farming or a cash-equivalent gift market.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fellowship_mailbox — Community Message Board
> Node 6 of 10 in **Friends, Community Bonds and Social Play** · World Loom identity `fellowship_mailbox` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A bounded in-world mailbox carries structured invitations, trade notices and shared activity recaps instead of unrestricted direct messaging.
- Why this belongs in Moonbyte: It keeps social interactions useful and reduces abuse surfaces.
- Distinctive interaction: Open a notice, choose its structured action, respond through the original feature and archive it.
- Successful completion produces: A typed notification record with expiry and mute controls.
### 02 · Purpose and boundaries
- Product role: Social systems help people cooperate, create and remember shared moments. They remain consent-based, scoped and easy to mute or leave.
- This system is the canonical owner of community message board behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Friends, Community Bonds and Social Play; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It keeps social interactions useful and reduces abuse surfaces.
- Stable destination ID: fellowship_mailbox; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fellowship mailbox and /fellowship mailbox are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open a notice and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to community message board.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A bounded in-world mailbox carries structured invitations, trade notices and shared activity recaps instead of unrestricted direct messaging.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open a notice, choose its structured action, respond through the original feature and archive it.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Community Message Board is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A typed notification record with expiry and mute controls.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses the Tavern as a familiar shared meeting point: The Story Stage (hearth_bard_stage).
- lets friends form or join structured long-running groups: Guild Trust Record (guild_reputation).
- uses the universal lobby, turn and reconnect contracts: Party Formation Desk (multiplayer_party).
- connects shared milestones to optional community events: Puzzle Rotation Archive (festival_rotating_puzzles).
### 12 · Incoming system relationships
- The Story Stage (hearth_bard_stage) can use this system because it gives friends and parties a shared meeting point without requiring a public conversation.
- Replay and Review Desk (arcade_replays) can use this system because it supports friend challenges and shared play with clear exit and privacy controls.
- Table Votes and Host Controls (multiplayer_table_votes) can use this system because it provides friend invitations, opt-in groups and cooperative roles.
- Archetype Codex (character_archetypes) can use this system because it gives parties complementary utility without requiring one optimal composition.
### 13 · Reward and economy contract
- Reward policy: A typed notification record with expiry and mute controls.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive community message board view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fellowship_mailbox rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for community message board: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It keeps social interactions useful and reduces abuse surfaces.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fellowship_commendations — Positive Contribution Notes
> Node 7 of 10 in **Friends, Community Bonds and Social Play** · World Loom identity `fellowship_commendations` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can recognise a specific helpful action such as explaining a puzzle or coordinating a group, subject to rate limits.
- Why this belongs in Moonbyte: It reinforces useful behaviour without converting reactions into tradable currency or popularity scores.
- Distinctive interaction: Select a verified session, choose a specific note, preview public visibility and submit or cancel.
- Successful completion produces: A privacy-controlled commendation linked to an evidence event.
### 02 · Purpose and boundaries
- Product role: Social systems help people cooperate, create and remember shared moments. They remain consent-based, scoped and easy to mute or leave.
- This system is the canonical owner of positive contribution notes behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Friends, Community Bonds and Social Play; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It reinforces useful behaviour without converting reactions into tradable currency or popularity scores.
- Stable destination ID: fellowship_commendations; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fellowship commendations and /fellowship commendations are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select a verified session and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to positive contribution notes.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can recognise a specific helpful action such as explaining a puzzle or coordinating a group, subject to rate limits.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select a verified session, choose a specific note, preview public visibility and submit or cancel.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Positive Contribution Notes is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A privacy-controlled commendation linked to an evidence event.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses the Tavern as a familiar shared meeting point: The Hearth Kitchen (hearth_kitchen).
- lets friends form or join structured long-running groups: The Guild Chronicle (guild_archive).
- uses the universal lobby, turn and reconnect contracts: Friend-First Match Finder (multiplayer_matchmaking).
- connects shared milestones to optional community events: The Celebration Wall (festival_celebration_wall).
### 12 · Incoming system relationships
- The Hearth Kitchen (hearth_kitchen) can use this system because it gives friends and parties a shared meeting point without requiring a public conversation.
- Arcade Control Profiles (arcade_accessibility) can use this system because it supports friend challenges and shared play with clear exit and privacy controls.
- Universal Lobby Contract (multiplayer_lobby) can use this system because it provides friend invitations, opt-in groups and cooperative roles.
- Skill Path Map (character_skill_paths) can use this system because it gives parties complementary utility without requiring one optimal composition.
### 13 · Reward and economy contract
- Reward policy: A privacy-controlled commendation linked to an evidence event.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive positive contribution notes view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fellowship_commendations rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for positive contribution notes: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It reinforces useful behaviour without converting reactions into tradable currency or popularity scores.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fellowship_memories — Shared Session Memories
> Node 8 of 10 in **Friends, Community Bonds and Social Play** · World Loom identity `fellowship_memories` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: After a group activity, participants can save a recap containing allowed events, screenshots or authored notes where rights permit.
- Why this belongs in Moonbyte: It gives multiplayer sessions a social afterlife and links to the guild chronicle.
- Distinctive interaction: Finish a session, choose shareable moments, approve visibility and save a group memory.
- Successful completion produces: A shared memory record with participant controls.
### 02 · Purpose and boundaries
- Product role: Social systems help people cooperate, create and remember shared moments. They remain consent-based, scoped and easy to mute or leave.
- This system is the canonical owner of shared session memories behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Friends, Community Bonds and Social Play; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives multiplayer sessions a social afterlife and links to the guild chronicle.
- Stable destination ID: fellowship_memories; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fellowship memories and /fellowship memories are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Finish a session and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to shared session memories.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: After a group activity, participants can save a recap containing allowed events, screenshots or authored notes where rights permit.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Finish a session, choose shareable moments, approve visibility and save a group memory.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Shared Session Memories is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A shared memory record with participant controls.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses the Tavern as a familiar shared meeting point: The Regulars' Circle (hearth_regulars).
- lets friends form or join structured long-running groups: Guild Charters (guild_charters).
- uses the universal lobby, turn and reconnect contracts: Shared Turn Engine (multiplayer_turn_engine).
- connects shared milestones to optional community events: Neighbourhood Festival Circuit (festival_cross_server).
### 12 · Incoming system relationships
- The Regulars' Circle (hearth_regulars) can use this system because it gives friends and parties a shared meeting point without requiring a public conversation.
- Curated Game Routes (arcade_playlists) can use this system because it supports friend challenges and shared play with clear exit and privacy controls.
- Party Formation Desk (multiplayer_party) can use this system because it provides friend invitations, opt-in groups and cooperative roles.
- Adventure Loadouts (character_loadouts) can use this system because it gives parties complementary utility without requiring one optimal composition.
### 13 · Reward and economy contract
- Reward policy: A shared memory record with participant controls.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive shared session memories view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fellowship_memories rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for shared session memories: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives multiplayer sessions a social afterlife and links to the guild chronicle.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fellowship_discovery — Friend Discovery Routes
> Node 9 of 10 in **Friends, Community Bonds and Social Play** · World Loom identity `fellowship_discovery` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can discover new games through a friend's opt-in public activity card rather than tracking private presence.
- Why this belongs in Moonbyte: It makes recommendations social while preserving the choice not to broadcast play.
- Distinctive interaction: Open a shared card, inspect the activity, start privately or request an invite and hide future shares if desired.
- Successful completion produces: A non-invasive shared recommendation event.
### 02 · Purpose and boundaries
- Product role: Social systems help people cooperate, create and remember shared moments. They remain consent-based, scoped and easy to mute or leave.
- This system is the canonical owner of friend discovery routes behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Friends, Community Bonds and Social Play; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes recommendations social while preserving the choice not to broadcast play.
- Stable destination ID: fellowship_discovery; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fellowship discovery and /fellowship discovery are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open a shared card and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to friend discovery routes.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can discover new games through a friend's opt-in public activity card rather than tracking private presence.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open a shared card, inspect the activity, start privately or request an invite and hide future shares if desired.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Friend Discovery Routes is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A non-invasive shared recommendation event.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses the Tavern as a familiar shared meeting point: The Common Room Project (hearth_renovation).
- lets friends form or join structured long-running groups: The Guild Hall (guild_hall).
- uses the universal lobby, turn and reconnect contracts: Simultaneous Choice Windows (multiplayer_simultaneous).
- connects shared milestones to optional community events: Open Archive Routes (festival_return_paths).
### 12 · Incoming system relationships
- The Common Room Project (hearth_renovation) can use this system because it gives friends and parties a shared meeting point without requiring a public conversation.
- The Pocket Arcade (arcade_library) can use this system because it supports friend challenges and shared play with clear exit and privacy controls.
- Friend-First Match Finder (multiplayer_matchmaking) can use this system because it provides friend invitations, opt-in groups and cooperative roles.
- Companion Chronicle (character_companions) can use this system because it gives parties complementary utility without requiring one optimal composition.
### 13 · Reward and economy contract
- Reward policy: A non-invasive shared recommendation event.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive friend discovery routes view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fellowship_discovery rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for friend discovery routes: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes recommendations social while preserving the choice not to broadcast play.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# fellowship_safety — Social Boundaries Panel
> Node 10 of 10 in **Friends, Community Bonds and Social Play** · World Loom identity `fellowship_safety` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A player can control invites, visits, friend requests, mentions and shared-content visibility from one consistent settings area.
- Why this belongs in Moonbyte: It makes safety controls discoverable across every social system.
- Distinctive interaction: Open boundaries, inspect defaults, choose audience and notification rules, block or revoke and review the change.
- Successful completion produces: A central preference set checked by social services.
### 02 · Purpose and boundaries
- Product role: Social systems help people cooperate, create and remember shared moments. They remain consent-based, scoped and easy to mute or leave.
- This system is the canonical owner of social boundaries panel behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Friends, Community Bonds and Social Play; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes safety controls discoverable across every social system.
- Stable destination ID: fellowship_safety; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .fellowship safety and /fellowship safety are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open boundaries and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to social boundaries panel.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A player can control invites, visits, friend requests, mentions and shared-content visibility from one consistent settings area.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open boundaries, inspect defaults, choose audience and notification rules, block or revoke and review the change.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Social Boundaries Panel is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A central preference set checked by social services.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses the Tavern as a familiar shared meeting point: The Tavern Wayfinder (hearth_routes).
- lets friends form or join structured long-running groups: Guild Role Charter (guild_roles).
- uses the universal lobby, turn and reconnect contracts: Spectator Gallery (multiplayer_spectators).
- connects shared milestones to optional community events: Festival Chronicle (festival_chronicles).
### 12 · Incoming system relationships
- The Tavern Wayfinder (hearth_routes) can use this system because it gives friends and parties a shared meeting point without requiring a public conversation.
- Optional Daily Logic Desk (arcade_daily_logic) can use this system because it supports friend challenges and shared play with clear exit and privacy controls.
- Shared Turn Engine (multiplayer_turn_engine) can use this system because it provides friend invitations, opt-in groups and cooperative roles.
- Role Synergy Board (character_party_synergy) can use this system because it gives parties complementary utility without requiring one optimal composition.
### 13 · Reward and economy contract
- Reward policy: A central preference set checked by social services.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive social boundaries panel view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the fellowship_safety rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for social boundaries panel: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes safety controls discoverable across every social system.; it must remain satisfying in a bounded session and support a natural stopping point.
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

## Domain 20 — Accessibility, Moderation, Privacy and Operations

Trust systems are cross-cutting foundations: accessible controls, safe content, privacy boundaries, reliable delivery and transparent operations apply to every game.

**Domain dependencies:** Server Culture and Community Layer (`server`) · Player Creation and Community Content (`creator`) · Fair Play, Competition and Verification (`fairness`) · The Hearth and Tavern (`hearth`)

# trust_control_profiles — Universal Control Profiles
> Node 1 of 10 in **Accessibility, Moderation, Privacy and Operations** · World Loom identity `trust_control_profiles` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A shared preference layer standardises keyboard, Discord button, text-only and extended-time controls across supported games.
- Why this belongs in Moonbyte: It prevents each game from inventing different accessibility behaviour.
- Distinctive interaction: Choose a profile, inspect compatible features, preview controls and apply preferences before sessions.
- Successful completion produces: A validated player control profile and game compatibility map.
### 02 · Purpose and boundaries
- Product role: Trust systems are cross-cutting foundations: accessible controls, safe content, privacy boundaries, reliable delivery and transparent operations apply to every game.
- This system is the canonical owner of universal control profiles behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Accessibility, Moderation, Privacy and Operations; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It prevents each game from inventing different accessibility behaviour.
- Stable destination ID: trust_control_profiles; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .trust control profiles and /trust control profiles are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a profile and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to universal control profiles.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A shared preference layer standardises keyboard, Discord button, text-only and extended-time controls across supported games.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a profile, inspect compatible features, preview controls and apply preferences before sessions.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Universal Control Profiles is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A validated player control profile and game compatibility map.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- enforces guild configuration, channel scope and moderator permissions: Server World Identity (server_identity).
- validates player-generated material before public distribution: Game Rule Templates (creator_rule_templates).
- provides integrity, appeal and result-verification processes: Tournament Rules Desk (fairness_tournament).
- exposes privacy, accessibility and notification settings from the central hub: The Tavern Wayfinder (hearth_routes).
### 12 · Incoming system relationships
- Server World Identity (server_identity) can use this system because it uses moderation, privacy and rate-limit policies for server-owned features.
- Vault Layout Workshop (creator_vault_builder) can use this system because it applies reporting, review, permissions and privacy policy to player-submitted material.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A validated player control profile and game compatibility map.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive universal control profiles view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the trust_control_profiles rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for universal control profiles: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It prevents each game from inventing different accessibility behaviour.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# trust_contrast_motion — Visual Comfort Settings
> Node 2 of 10 in **Accessibility, Moderation, Privacy and Operations** · World Loom identity `trust_contrast_motion` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can select high contrast, reduced motion, simpler animation and readable text density for supported views.
- Why this belongs in Moonbyte: It makes the full-colour pixel-art world usable across different devices and sensitivities.
- Distinctive interaction: Open display settings, preview each option, apply a combination and retain the preference across surfaces.
- Successful completion produces: A display preference set shared by Discord renderers and the React Activity.
### 02 · Purpose and boundaries
- Product role: Trust systems are cross-cutting foundations: accessible controls, safe content, privacy boundaries, reliable delivery and transparent operations apply to every game.
- This system is the canonical owner of visual comfort settings behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Accessibility, Moderation, Privacy and Operations; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes the full-colour pixel-art world usable across different devices and sensitivities.
- Stable destination ID: trust_contrast_motion; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .trust contrast motion and /trust contrast motion are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open display settings and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to visual comfort settings.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can select high contrast, reduced motion, simpler animation and readable text density for supported views.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open display settings, preview each option, apply a combination and retain the preference across surfaces.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Visual Comfort Settings is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A display preference set shared by Discord renderers and the React Activity.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- enforces guild configuration, channel scope and moderator permissions: Community Calendar (server_calendar).
- validates player-generated material before public distribution: Original Card Set Builder (creator_deck_builder).
- provides integrity, appeal and result-verification processes: Leaderboard Integrity (fairness_leaderboard).
- exposes privacy, accessibility and notification settings from the central hub: The Lantern and Ladle (hearth_main_room).
### 12 · Incoming system relationships
- Community Calendar (server_calendar) can use this system because it uses moderation, privacy and rate-limit policies for server-owned features.
- Game Rule Templates (creator_rule_templates) can use this system because it applies reporting, review, permissions and privacy policy to player-submitted material.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A display preference set shared by Discord renderers and the React Activity.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive visual comfort settings view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the trust_contrast_motion rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for visual comfort settings: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes the full-colour pixel-art world usable across different devices and sensitivities.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# trust_localisation — Language and Locale Registry
> Node 3 of 10 in **Accessibility, Moderation, Privacy and Operations** · World Loom identity `trust_localisation` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Text content can carry locale identifiers, fallbacks and review status so new languages do not silently ship as machine-quality text.
- Why this belongs in Moonbyte: It lets the world grow internationally while keeping rules and legal actions unambiguous.
- Distinctive interaction: Choose a locale, inspect translation coverage, use reviewed strings and report a missing or unclear phrase.
- Successful completion produces: A locale preference and versioned translation catalogue.
### 02 · Purpose and boundaries
- Product role: Trust systems are cross-cutting foundations: accessible controls, safe content, privacy boundaries, reliable delivery and transparent operations apply to every game.
- This system is the canonical owner of language and locale registry behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Accessibility, Moderation, Privacy and Operations; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It lets the world grow internationally while keeping rules and legal actions unambiguous.
- Stable destination ID: trust_localisation; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .trust localisation and /trust localisation are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a locale and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to language and locale registry.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Text content can carry locale identifiers, fallbacks and review status so new languages do not silently ship as machine-quality text.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a locale, inspect translation coverage, use reviewed strings and report a missing or unclear phrase.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Language and Locale Registry is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A locale preference and versioned translation catalogue.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- enforces guild configuration, channel scope and moderator permissions: Channel Wayfinding (server_channels).
- validates player-generated material before public distribution: Pixel Sticker Workshop (creator_pixel_editor).
- provides integrity, appeal and result-verification processes: Equal Conditions Profiles (fairness_access).
- exposes privacy, accessibility and notification settings from the central hub: The Notice Wall (hearth_notice_wall).
### 12 · Incoming system relationships
- Channel Wayfinding (server_channels) can use this system because it uses moderation, privacy and rate-limit policies for server-owned features.
- Original Card Set Builder (creator_deck_builder) can use this system because it applies reporting, review, permissions and privacy policy to player-submitted material.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A locale preference and versioned translation catalogue.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive language and locale registry view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the trust_localisation rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for language and locale registry: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It lets the world grow internationally while keeping rules and legal actions unambiguous.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# trust_text_fallback — Text-First Play Mode
> Node 4 of 10 in **Accessibility, Moderation, Privacy and Operations** · World Loom identity `trust_text_fallback` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Every visual board and important state has a compact text representation with labels for actions and current status.
- Why this belongs in Moonbyte: It ensures that the game remains playable when images fail or a player prefers text.
- Distinctive interaction: Enable text mode, open a game board, read the state, submit the same legal action and view the result.
- Successful completion produces: A text view model generated from the same authoritative state.
### 02 · Purpose and boundaries
- Product role: Trust systems are cross-cutting foundations: accessible controls, safe content, privacy boundaries, reliable delivery and transparent operations apply to every game.
- This system is the canonical owner of text-first play mode behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Accessibility, Moderation, Privacy and Operations; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It ensures that the game remains playable when images fail or a player prefers text.
- Stable destination ID: trust_text_fallback; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .trust text fallback and /trust text fallback are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Enable text mode and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to text-first play mode.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Every visual board and important state has a compact text representation with labels for actions and current status.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Enable text mode, open a game board, read the state, submit the same legal action and view the result.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Text-First Play Mode is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A text view model generated from the same authoritative state.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- enforces guild configuration, channel scope and moderator permissions: Server Challenge Board (server_challenges).
- validates player-generated material before public distribution: Community Playtest Queue (creator_playtest).
- provides integrity, appeal and result-verification processes: Result Appeal Desk (fairness_appeals).
- exposes privacy, accessibility and notification settings from the central hub: The Rumour Circle (hearth_rumour_circle).
### 12 · Incoming system relationships
- Server Challenge Board (server_challenges) can use this system because it uses moderation, privacy and rate-limit policies for server-owned features.
- Pixel Sticker Workshop (creator_pixel_editor) can use this system because it applies reporting, review, permissions and privacy policy to player-submitted material.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A text view model generated from the same authoritative state.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive text-first play mode view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the trust_text_fallback rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for text-first play mode: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It ensures that the game remains playable when images fail or a player prefers text.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# trust_moderator_console — Moderator Console
> Node 5 of 10 in **Accessibility, Moderation, Privacy and Operations** · World Loom identity `trust_moderator_console` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A least-privilege moderation surface handles reports, content review, session safety and server settings without exposing private inventory.
- Why this belongs in Moonbyte: It gives moderators enough information to protect a server without granting database-wide access.
- Distinctive interaction: Open a scoped queue, inspect minimal evidence, choose a bounded action, record the reason and close the case.
- Successful completion produces: An auditable moderation action with role and scope checks.
### 02 · Purpose and boundaries
- Product role: Trust systems are cross-cutting foundations: accessible controls, safe content, privacy boundaries, reliable delivery and transparent operations apply to every game.
- This system is the canonical owner of moderator console behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Accessibility, Moderation, Privacy and Operations; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives moderators enough information to protect a server without granting database-wide access.
- Stable destination ID: trust_moderator_console; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .trust moderator console and /trust moderator console are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open a scoped queue and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to moderator console.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A least-privilege moderation surface handles reports, content review, session safety and server settings without exposing private inventory.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open a scoped queue, inspect minimal evidence, choose a bounded action, record the reason and close the case.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Moderator Console is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: An auditable moderation action with role and scope checks.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- enforces guild configuration, channel scope and moderator permissions: The Community Museum (server_museum).
- validates player-generated material before public distribution: Content Review Pipeline (creator_review).
- provides integrity, appeal and result-verification processes: Published Rulebooks (fairness_rulebooks).
- exposes privacy, accessibility and notification settings from the central hub: The Common Tables (hearth_table_games).
### 12 · Incoming system relationships
- The Community Museum (server_museum) can use this system because it uses moderation, privacy and rate-limit policies for server-owned features.
- Community Playtest Queue (creator_playtest) can use this system because it applies reporting, review, permissions and privacy policy to player-submitted material.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: An auditable moderation action with role and scope checks.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive moderator console view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the trust_moderator_console rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for moderator console: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives moderators enough information to protect a server without granting database-wide access.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# trust_reports — Report and Review Pipeline
> Node 6 of 10 in **Accessibility, Moderation, Privacy and Operations** · World Loom identity `trust_reports` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can report public content, trade issues and social behaviour through structured categories and clear status updates.
- Why this belongs in Moonbyte: It creates a consistent path to help while avoiding public pile-ons or unbounded evidence collection.
- Distinctive interaction: Choose the offending object, select a reason, submit privately, check case status and appeal where supported.
- Successful completion produces: A private report record with evidence retention limits.
### 02 · Purpose and boundaries
- Product role: Trust systems are cross-cutting foundations: accessible controls, safe content, privacy boundaries, reliable delivery and transparent operations apply to every game.
- This system is the canonical owner of report and review pipeline behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Accessibility, Moderation, Privacy and Operations; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates a consistent path to help while avoiding public pile-ons or unbounded evidence collection.
- Stable destination ID: trust_reports; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .trust reports and /trust reports are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose the offending object and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to report and review pipeline.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can report public content, trade issues and social behaviour through structured categories and clear status updates.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose the offending object, select a reason, submit privately, check case status and appeal where supported.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Report and Review Pipeline is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A private report record with evidence retention limits.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- enforces guild configuration, channel scope and moderator permissions: Neighbourhood Goals (server_goals).
- validates player-generated material before public distribution: Creator Credit Ledger (creator_attribution).
- provides integrity, appeal and result-verification processes: Commit and Reveal Service (fairness_commit_reveal).
- exposes privacy, accessibility and notification settings from the central hub: The Host's Bench (hearth_hosts_bench).
### 12 · Incoming system relationships
- Neighbourhood Goals (server_goals) can use this system because it uses moderation, privacy and rate-limit policies for server-owned features.
- Content Review Pipeline (creator_review) can use this system because it applies reporting, review, permissions and privacy policy to player-submitted material.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A private report record with evidence retention limits.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive report and review pipeline view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the trust_reports rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for report and review pipeline: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates a consistent path to help while avoiding public pile-ons or unbounded evidence collection.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# trust_privacy_center — Privacy and Data Center
> Node 7 of 10 in **Accessibility, Moderation, Privacy and Operations** · World Loom identity `trust_privacy_center` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A central privacy command and Activity page show stored categories, purpose, export and deletion status honestly.
- Why this belongs in Moonbyte: It makes privacy a usable product feature rather than a paragraph hidden in documentation.
- Distinctive interaction: Inspect data inventory, request export, review deletion scope, confirm as the owner and receive a truthful result.
- Successful completion produces: A privacy request state and complete repository coverage map.
### 02 · Purpose and boundaries
- Product role: Trust systems are cross-cutting foundations: accessible controls, safe content, privacy boundaries, reliable delivery and transparent operations apply to every game.
- This system is the canonical owner of privacy and data center behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Accessibility, Moderation, Privacy and Operations; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes privacy a usable product feature rather than a paragraph hidden in documentation.
- Stable destination ID: trust_privacy_center; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .trust privacy center and /trust privacy center are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect data inventory and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to privacy and data center.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A central privacy command and Activity page show stored categories, purpose, export and deletion status honestly.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect data inventory, request export, review deletion scope, confirm as the owner and receive a truthful result.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Privacy and Data Center is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A privacy request state and complete repository coverage map.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- enforces guild configuration, channel scope and moderator permissions: Scoped Scoreboards (server_leaderboards).
- validates player-generated material before public distribution: Community Pack Publishing (creator_publishing).
- provides integrity, appeal and result-verification processes: Scoring Contract Registry (fairness_scoring).
- exposes privacy, accessibility and notification settings from the central hub: The Story Stage (hearth_bard_stage).
### 12 · Incoming system relationships
- Scoped Scoreboards (server_leaderboards) can use this system because it uses moderation, privacy and rate-limit policies for server-owned features.
- Creator Credit Ledger (creator_attribution) can use this system because it applies reporting, review, permissions and privacy policy to player-submitted material.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A privacy request state and complete repository coverage map.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive privacy and data center view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the trust_privacy_center rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for privacy and data center: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes privacy a usable product feature rather than a paragraph hidden in documentation.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# trust_notification_rules — Quiet Notification Rules
> Node 8 of 10 in **Accessibility, Moderation, Privacy and Operations** · World Loom identity `trust_notification_rules` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A preference engine applies opt-in topics, quiet hours, frequency caps and immediate mute across reminders and digests.
- Why this belongs in Moonbyte: It keeps Discord useful without nagging, forced attendance or hidden notification defaults.
- Distinctive interaction: Choose topics and timing, preview the effect, pause or mute globally and change settings later.
- Successful completion produces: A versioned notification policy checked before every delivery.
### 02 · Purpose and boundaries
- Product role: Trust systems are cross-cutting foundations: accessible controls, safe content, privacy boundaries, reliable delivery and transparent operations apply to every game.
- This system is the canonical owner of quiet notification rules behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Accessibility, Moderation, Privacy and Operations; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It keeps Discord useful without nagging, forced attendance or hidden notification defaults.
- Stable destination ID: trust_notification_rules; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .trust notification rules and /trust notification rules are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose topics and timing and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to quiet notification rules.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A preference engine applies opt-in topics, quiet hours, frequency caps and immediate mute across reminders and digests.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose topics and timing, preview the effect, pause or mute globally and change settings later.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Quiet Notification Rules is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned notification policy checked before every delivery.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- enforces guild configuration, channel scope and moderator permissions: Server Steward Desk (server_moderation).
- validates player-generated material before public distribution: Puzzle Studio (creator_puzzle_studio).
- provides integrity, appeal and result-verification processes: Optional Skill Rating (fairness_ratings).
- exposes privacy, accessibility and notification settings from the central hub: The Hearth Kitchen (hearth_kitchen).
### 12 · Incoming system relationships
- Server Steward Desk (server_moderation) can use this system because it uses moderation, privacy and rate-limit policies for server-owned features.
- Community Pack Publishing (creator_publishing) can use this system because it applies reporting, review, permissions and privacy policy to player-submitted material.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A versioned notification policy checked before every delivery.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive quiet notification rules view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the trust_notification_rules rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for quiet notification rules: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It keeps Discord useful without nagging, forced attendance or hidden notification defaults.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# trust_reliability — Session Reliability Desk
> Node 9 of 10 in **Accessibility, Moderation, Privacy and Operations** · World Loom identity `trust_reliability` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Operational state tracks gateway health, session recovery, SQLite contention, render failures and retry outcomes without collecting message content.
- Why this belongs in Moonbyte: It prevents infrastructure failures from becoming player progress loss.
- Distinctive interaction: Detect a fault, preserve durable state, retry idempotently, notify only when useful and expose a safe status.
- Successful completion produces: A bounded health snapshot and recovery event stream.
### 02 · Purpose and boundaries
- Product role: Trust systems are cross-cutting foundations: accessible controls, safe content, privacy boundaries, reliable delivery and transparent operations apply to every game.
- This system is the canonical owner of session reliability desk behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Accessibility, Moderation, Privacy and Operations; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It prevents infrastructure failures from becoming player progress loss.
- Stable destination ID: trust_reliability; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .trust reliability and /trust reliability are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Detect a fault and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to session reliability desk.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Operational state tracks gateway health, session recovery, SQLite contention, render failures and retry outcomes without collecting message content.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Detect a fault, preserve durable state, retry idempotently, notify only when useful and expose a safe status.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Session Reliability Desk is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A bounded health snapshot and recovery event stream.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- enforces guild configuration, channel scope and moderator permissions: Server Data Boundary (server_privacy).
- validates player-generated material before public distribution: Quest Workshop (creator_quest_editor).
- provides integrity, appeal and result-verification processes: Match Review Transcript (fairness_match_review).
- exposes privacy, accessibility and notification settings from the central hub: The Regulars' Circle (hearth_regulars).
### 12 · Incoming system relationships
- Server Data Boundary (server_privacy) can use this system because it uses moderation, privacy and rate-limit policies for server-owned features.
- Puzzle Studio (creator_puzzle_studio) can use this system because it applies reporting, review, permissions and privacy policy to player-submitted material.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A bounded health snapshot and recovery event stream.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive session reliability desk view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the trust_reliability rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for session reliability desk: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It prevents infrastructure failures from becoming player progress loss.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# trust_content_policy — World Content Policy
> Node 10 of 10 in **Accessibility, Moderation, Privacy and Operations** · World Loom identity `trust_content_policy` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A shared policy defines age-appropriate text, original asset rights, user-content review and disallowed content across all modules.
- Why this belongs in Moonbyte: It lets a large content catalogue grow without inconsistent moderation decisions.
- Distinctive interaction: Check a content source, validate text and asset rights, apply review status and block unapproved public publication.
- Successful completion produces: A policy version and content eligibility result.
### 02 · Purpose and boundaries
- Product role: Trust systems are cross-cutting foundations: accessible controls, safe content, privacy boundaries, reliable delivery and transparent operations apply to every game.
- This system is the canonical owner of world content policy behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Accessibility, Moderation, Privacy and Operations; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It lets a large content catalogue grow without inconsistent moderation decisions.
- Stable destination ID: trust_content_policy; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .trust content policy and /trust content policy are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Check a content source and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to world content policy.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A shared policy defines age-appropriate text, original asset rights, user-content review and disallowed content across all modules.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Check a content source, validate text and asset rights, apply review status and block unapproved public publication.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: World Content Policy is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A policy version and content eligibility result.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- enforces guild configuration, channel scope and moderator permissions: The Quiet Community Digest (server_digest).
- validates player-generated material before public distribution: Vault Layout Workshop (creator_vault_builder).
- provides integrity, appeal and result-verification processes: Integrity Guardrails (fairness_anti_abuse).
- exposes privacy, accessibility and notification settings from the central hub: The Common Room Project (hearth_renovation).
### 12 · Incoming system relationships
- The Quiet Community Digest (server_digest) can use this system because it uses moderation, privacy and rate-limit policies for server-owned features.
- Quest Workshop (creator_quest_editor) can use this system because it applies reporting, review, permissions and privacy policy to player-submitted material.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A policy version and content eligibility result.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive world content policy view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the trust_content_policy rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for world content policy: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It lets a large content catalogue grow without inconsistent moderation decisions.; it must remain satisfying in a bounded session and support a natural stopping point.
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
