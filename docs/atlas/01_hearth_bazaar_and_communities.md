# Volume I — Hearth, Bazaar and Communities

> Moonbyte World Loom · 40 connected system proposals · This is a design specification, not a claim of implementation.

Each card is a distinct proposal with a stable graph identity, a player loop, state and ownership boundaries, Discord and Activity surfaces, privacy rules, acceptance tests and explicit cross-system links.

## Domain 1 — The Hearth and Tavern

The Tavern is Moonbyte's social home: the default place to discover activities, form parties, meet authored characters, review stories and step into the wider world.

**Domain dependencies:** The Bazaar and Exchange (`bazaar`) · Quests, Mystery and Story (`quest`) · Arcade and Original Mini-Games (`arcade`) · Friends, Community Bonds and Social Play (`fellowship`)

# hearth_main_room — The Lantern and Ladle
> Node 1 of 10 in **The Hearth and Tavern** · World Loom identity `hearth_main_room` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A persistent, recognisable social home where every player can see what is available without searching a command catalogue.
- Why this belongs in Moonbyte: It replaces disconnected entry commands with one navigable place and establishes the tone of the world.
- Distinctive interaction: Arrive, inspect the room, choose a table or notice, invite a party, then leave or continue into a destination.
- Successful completion produces: A clear next-action menu, a resumable home location and a shared visual identity.
### 02 · Purpose and boundaries
- Product role: The Tavern is Moonbyte's social home: the default place to discover activities, form parties, meet authored characters, review stories and step into the wider world.
- This system is the canonical owner of the lantern and ladle behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Hearth and Tavern; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It replaces disconnected entry commands with one navigable place and establishes the tone of the world.
- Stable destination ID: hearth_main_room; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .hearth main room and /hearth main room are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Arrive and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the lantern and ladle.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A persistent, recognisable social home where every player can see what is available without searching a command catalogue.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Arrive, inspect the room, choose a table or notice, invite a party, then leave or continue into a destination.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Lantern and Ladle is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A clear next-action menu, a resumable home location and a shared visual identity.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- turns player needs and discovered demand into transparent Bazaar browsing: Player Stallfronts (bazaar_stallfronts).
- converts rumours, NPC conversations and notice-board entries into optional quest starts: The Investigation Desk (quest_mysteries).
- provides immediate short-session games between longer adventures: Friendly Arcade League (arcade_league).
- gives friends and parties a shared meeting point without requiring a public conversation: Social Boundaries Panel (fellowship_safety).
### 12 · Incoming system relationships
- Player Stallfronts (bazaar_stallfronts) can use this system because it makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands.
- Choice and Consequence Graph (quest_branches) can use this system because it surfaces rumours and story contracts in a calm central hub.
- The Party Game Shelf (arcade_party_collection) can use this system because it lets players start a short game from a Tavern table without navigating a separate service.
- Neighbourhood Festival Circuit (festival_cross_server) can use this system because it surfaces a festival calmly through the Tavern and player-selected digests.
### 13 · Reward and economy contract
- Reward policy: A clear next-action menu, a resumable home location and a shared visual identity.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the lantern and ladle view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the hearth_main_room rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the lantern and ladle: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It replaces disconnected entry commands with one navigable place and establishes the tone of the world.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# hearth_notice_wall — The Notice Wall
> Node 2 of 10 in **The Hearth and Tavern** · World Loom identity `hearth_notice_wall` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A readable wall that gathers quests, guild requests, player commissions, community projects and server announcements into one filtered board.
- Why this belongs in Moonbyte: It prevents important activities from being buried in chat and makes the world feel connected without constant notifications.
- Distinctive interaction: Open the wall, filter by category and time expectation, inspect terms, pin an activity to a personal route, then accept or dismiss it.
- Successful completion produces: A selected activity and a saved route; dismissing an entry never penalises the player.
### 02 · Purpose and boundaries
- Product role: The Tavern is Moonbyte's social home: the default place to discover activities, form parties, meet authored characters, review stories and step into the wider world.
- This system is the canonical owner of the notice wall behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Hearth and Tavern; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It prevents important activities from being buried in chat and makes the world feel connected without constant notifications.
- Stable destination ID: hearth_notice_wall; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .hearth notice wall and /hearth notice wall are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open the wall and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the notice wall.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A readable wall that gathers quests, guild requests, player commissions, community projects and server announcements into one filtered board.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open the wall, filter by category and time expectation, inspect terms, pin an activity to a personal route, then accept or dismiss it.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Notice Wall is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A selected activity and a saved route; dismissing an entry never penalises the player.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- turns player needs and discovered demand into transparent Bazaar browsing: Fixed-Price Listings (bazaar_fixed_listings).
- converts rumours, NPC conversations and notice-board entries into optional quest starts: Dialogue Routes (quest_dialogue).
- provides immediate short-session games between longer adventures: Replay and Review Desk (arcade_replays).
- gives friends and parties a shared meeting point without requiring a public conversation: Friend Circle (fellowship_friends).
### 12 · Incoming system relationships
- Fixed-Price Listings (bazaar_fixed_listings) can use this system because it makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands.
- The Investigation Desk (quest_mysteries) can use this system because it surfaces rumours and story contracts in a calm central hub.
- Asynchronous Challenge Cards (arcade_async_challenges) can use this system because it lets players start a short game from a Tavern table without navigating a separate service.
- Open Archive Routes (festival_return_paths) can use this system because it surfaces a festival calmly through the Tavern and player-selected digests.
### 13 · Reward and economy contract
- Reward policy: A selected activity and a saved route; dismissing an entry never penalises the player.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the notice wall view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the hearth_notice_wall rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the notice wall: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It prevents important activities from being buried in chat and makes the world feel connected without constant notifications.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# hearth_rumour_circle — The Rumour Circle
> Node 3 of 10 in **The Hearth and Tavern** · World Loom identity `hearth_rumour_circle` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A narrative discovery surface where authored rumours hint at places, NPCs, puzzles and world history rather than hiding random prizes.
- Why this belongs in Moonbyte: It links Tavern social play to exploration and gives players reasons to compare clues and form hypotheses.
- Distinctive interaction: Hear a rumour, inspect its evidence level, ask a follow-up, compare it with the Atlas, then choose whether to investigate.
- Successful completion produces: Clue references, world knowledge flags and optional quest leads.
### 02 · Purpose and boundaries
- Product role: The Tavern is Moonbyte's social home: the default place to discover activities, form parties, meet authored characters, review stories and step into the wider world.
- This system is the canonical owner of the rumour circle behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Hearth and Tavern; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It links Tavern social play to exploration and gives players reasons to compare clues and form hypotheses.
- Stable destination ID: hearth_rumour_circle; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .hearth rumour circle and /hearth rumour circle are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Hear a rumour and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the rumour circle.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A narrative discovery surface where authored rumours hint at places, NPCs, puzzles and world history rather than hiding random prizes.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Hear a rumour, inspect its evidence level, ask a follow-up, compare it with the Atlas, then choose whether to investigate.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Rumour Circle is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: Clue references, world knowledge flags and optional quest leads.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- turns player needs and discovered demand into transparent Bazaar browsing: The Barter Desk (bazaar_barter_desk).
- converts rumours, NPC conversations and notice-board entries into optional quest starts: Faction Storylines (quest_factions).
- provides immediate short-session games between longer adventures: Arcade Control Profiles (arcade_accessibility).
- gives friends and parties a shared meeting point without requiring a public conversation: Cooperative Role Planner (fellowship_roles).
### 12 · Incoming system relationships
- The Barter Desk (bazaar_barter_desk) can use this system because it makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands.
- Dialogue Routes (quest_dialogue) can use this system because it surfaces rumours and story contracts in a calm central hub.
- Friendly Arcade League (arcade_league) can use this system because it lets players start a short game from a Tavern table without navigating a separate service.
- Festival Chronicle (festival_chronicles) can use this system because it surfaces a festival calmly through the Tavern and player-selected digests.
### 13 · Reward and economy contract
- Reward policy: Clue references, world knowledge flags and optional quest leads.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the rumour circle view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the hearth_rumour_circle rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the rumour circle: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It links Tavern social play to exploration and gives players reasons to compare clues and form hypotheses.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# hearth_table_games — The Common Tables
> Node 4 of 10 in **The Hearth and Tavern** · World Loom identity `hearth_table_games` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A set of named tables for quick solo, cooperative and party games with clear session lengths and player limits.
- Why this belongs in Moonbyte: It gives the social hub a low-friction arcade entrance while preserving the separate game-session contract.
- Distinctive interaction: Choose a table, preview rules, invite or queue with friends, play a bounded round, then review the result.
- Successful completion produces: A completed mini-game session, personal record and optional progression event.
### 02 · Purpose and boundaries
- Product role: The Tavern is Moonbyte's social home: the default place to discover activities, form parties, meet authored characters, review stories and step into the wider world.
- This system is the canonical owner of the common tables behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Hearth and Tavern; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives the social hub a low-friction arcade entrance while preserving the separate game-session contract.
- Stable destination ID: hearth_table_games; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .hearth table games and /hearth table games are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a table and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the common tables.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A set of named tables for quick solo, cooperative and party games with clear session lengths and player limits.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a table, preview rules, invite or queue with friends, play a bounded round, then review the result.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Common Tables is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A completed mini-game session, personal record and optional progression event.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- turns player needs and discovered demand into transparent Bazaar browsing: The Wanted Board (bazaar_wanted_board).
- converts rumours, NPC conversations and notice-board entries into optional quest starts: Local Tales (quest_side_stories).
- provides immediate short-session games between longer adventures: Curated Game Routes (arcade_playlists).
- gives friends and parties a shared meeting point without requiring a public conversation: Opt-In Mentor Sessions (fellowship_mentor).
### 12 · Incoming system relationships
- The Wanted Board (bazaar_wanted_board) can use this system because it makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands.
- Faction Storylines (quest_factions) can use this system because it surfaces rumours and story contracts in a calm central hub.
- Replay and Review Desk (arcade_replays) can use this system because it lets players start a short game from a Tavern table without navigating a separate service.
- Moonbyte Anniversary (festival_anniversary) can use this system because it surfaces a festival calmly through the Tavern and player-selected digests.
### 13 · Reward and economy contract
- Reward policy: A completed mini-game session, personal record and optional progression event.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the common tables view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the hearth_table_games rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the common tables: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives the social hub a low-friction arcade entrance while preserving the separate game-session contract.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# hearth_hosts_bench — The Host's Bench
> Node 5 of 10 in **The Hearth and Tavern** · World Loom identity `hearth_hosts_bench` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A lobby desk for creating a hosted activity, choosing visibility, setting seat limits and managing invitations.
- Why this belongs in Moonbyte: It unifies lobbies across games instead of implementing a different invitation pattern in every module.
- Distinctive interaction: Select an activity, set a public or private room, preview permissions, invite participants, ready the group and launch.
- Successful completion produces: A validated lobby with host authority, join history and a recoverable session identity.
### 02 · Purpose and boundaries
- Product role: The Tavern is Moonbyte's social home: the default place to discover activities, form parties, meet authored characters, review stories and step into the wider world.
- This system is the canonical owner of the host's bench behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Hearth and Tavern; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It unifies lobbies across games instead of implementing a different invitation pattern in every module.
- Stable destination ID: hearth_hosts_bench; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .hearth hosts bench and /hearth hosts bench are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select an activity and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the host's bench.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A lobby desk for creating a hosted activity, choosing visibility, setting seat limits and managing invitations.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select an activity, set a public or private room, preview permissions, invite participants, ready the group and launch.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Host's Bench is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A validated lobby with host authority, join history and a recoverable session identity.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- turns player needs and discovered demand into transparent Bazaar browsing: Crafting Work Orders (bazaar_work_orders).
- converts rumours, NPC conversations and notice-board entries into optional quest starts: Shared Story Decisions (quest_coop_story).
- provides immediate short-session games between longer adventures: The Pocket Arcade (arcade_library).
- gives friends and parties a shared meeting point without requiring a public conversation: Interest Clubs (fellowship_clubs).
### 12 · Incoming system relationships
- Crafting Work Orders (bazaar_work_orders) can use this system because it makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands.
- Local Tales (quest_side_stories) can use this system because it surfaces rumours and story contracts in a calm central hub.
- Arcade Control Profiles (arcade_accessibility) can use this system because it lets players start a short game from a Tavern table without navigating a separate service.
- World Fairs (festival_world_fairs) can use this system because it surfaces a festival calmly through the Tavern and player-selected digests.
### 13 · Reward and economy contract
- Reward policy: A validated lobby with host authority, join history and a recoverable session identity.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the host's bench view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the hearth_hosts_bench rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the host's bench: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It unifies lobbies across games instead of implementing a different invitation pattern in every module.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# hearth_bard_stage — The Story Stage
> Node 6 of 10 in **The Hearth and Tavern** · World Loom identity `hearth_bard_stage` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A moderated space for original player-written tales, riddles, recaps and short performances connected to places in the world.
- Why this belongs in Moonbyte: It creates community expression without copying protected songs, characters or third-party stories.
- Distinctive interaction: Choose a prompt, compose original text, preview formatting, submit for display, receive reactions and archive the work.
- Successful completion produces: Attributed creative entries, reviewed display status and links to the world nodes they reference.
### 02 · Purpose and boundaries
- Product role: The Tavern is Moonbyte's social home: the default place to discover activities, form parties, meet authored characters, review stories and step into the wider world.
- This system is the canonical owner of the story stage behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Hearth and Tavern; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates community expression without copying protected songs, characters or third-party stories.
- Stable destination ID: hearth_bard_stage; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .hearth bard stage and /hearth bard stage are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a prompt and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the story stage.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A moderated space for original player-written tales, riddles, recaps and short performances connected to places in the world.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a prompt, compose original text, preview formatting, submit for display, receive reactions and archive the work.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Story Stage is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: Attributed creative entries, reviewed display status and links to the world nodes they reference.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- turns player needs and discovered demand into transparent Bazaar browsing: The Price Reference Ledger (bazaar_price_ledger).
- converts rumours, NPC conversations and notice-board entries into optional quest starts: The Chapter Chronicle (quest_chronicle).
- provides immediate short-session games between longer adventures: Optional Daily Logic Desk (arcade_daily_logic).
- gives friends and parties a shared meeting point without requiring a public conversation: Thoughtful Gift Exchange (fellowship_gifting).
### 12 · Incoming system relationships
- The Price Reference Ledger (bazaar_price_ledger) can use this system because it makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands.
- Shared Story Decisions (quest_coop_story) can use this system because it surfaces rumours and story contracts in a calm central hub.
- Curated Game Routes (arcade_playlists) can use this system because it lets players start a short game from a Tavern table without navigating a separate service.
- Seasonal Story Chapters (festival_chapters) can use this system because it surfaces a festival calmly through the Tavern and player-selected digests.
### 13 · Reward and economy contract
- Reward policy: Attributed creative entries, reviewed display status and links to the world nodes they reference.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the story stage view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the hearth_bard_stage rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the story stage: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates community expression without copying protected songs, characters or third-party stories.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# hearth_kitchen — The Hearth Kitchen
> Node 7 of 10 in **The Hearth and Tavern** · World Loom identity `hearth_kitchen` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A cooperative recipe and cooking station where ingredients earned through play become known dishes, buffs for puzzles or decorative table items.
- Why this belongs in Moonbyte: It turns gathering and crafting into a social activity and gives the Tavern a useful sink for non-premium resources.
- Distinctive interaction: Inspect recipes, contribute known ingredients, prepare a deterministic result, share servings and save the recipe.
- Successful completion produces: A crafted dish with published effects, ingredient provenance and an optional shared-session benefit.
### 02 · Purpose and boundaries
- Product role: The Tavern is Moonbyte's social home: the default place to discover activities, form parties, meet authored characters, review stories and step into the wider world.
- This system is the canonical owner of the hearth kitchen behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Hearth and Tavern; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It turns gathering and crafting into a social activity and gives the Tavern a useful sink for non-premium resources.
- Stable destination ID: hearth_kitchen; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .hearth kitchen and /hearth kitchen are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect recipes and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the hearth kitchen.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A cooperative recipe and cooking station where ingredients earned through play become known dishes, buffs for puzzles or decorative table items.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect recipes, contribute known ingredients, prepare a deterministic result, share servings and save the recipe.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Hearth Kitchen is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A crafted dish with published effects, ingredient provenance and an optional shared-session benefit.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- turns player needs and discovered demand into transparent Bazaar browsing: The Escrow Vault (bazaar_escrow_vault).
- converts rumours, NPC conversations and notice-board entries into optional quest starts: Story Remix Paths (quest_remix).
- provides immediate short-session games between longer adventures: The Practice School (arcade_skill_school).
- gives friends and parties a shared meeting point without requiring a public conversation: Community Message Board (fellowship_mailbox).
### 12 · Incoming system relationships
- The Escrow Vault (bazaar_escrow_vault) can use this system because it makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands.
- The Chapter Chronicle (quest_chronicle) can use this system because it surfaces rumours and story contracts in a calm central hub.
- The Pocket Arcade (arcade_library) can use this system because it lets players start a short game from a Tavern table without navigating a separate service.
- Server Celebration Kits (festival_server_events) can use this system because it surfaces a festival calmly through the Tavern and player-selected digests.
### 13 · Reward and economy contract
- Reward policy: A crafted dish with published effects, ingredient provenance and an optional shared-session benefit.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the hearth kitchen view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the hearth_kitchen rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the hearth kitchen: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It turns gathering and crafting into a social activity and gives the Tavern a useful sink for non-premium resources.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# hearth_regulars — The Regulars' Circle
> Node 8 of 10 in **The Hearth and Tavern** · World Loom identity `hearth_regulars` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A roster of authored Tavern NPCs whose dialogue reflects world events, completed chapters and the player's chosen interests.
- Why this belongs in Moonbyte: It makes narrative state visible through a warm hub while avoiding randomised affection or manipulative character timers.
- Distinctive interaction: Meet a regular, select a dialogue topic, receive a clue or story response, unlock a journal note and return later.
- Successful completion produces: Dialogue flags, character lore entries and contextual leads.
### 02 · Purpose and boundaries
- Product role: The Tavern is Moonbyte's social home: the default place to discover activities, form parties, meet authored characters, review stories and step into the wider world.
- This system is the canonical owner of the regulars' circle behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Hearth and Tavern; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes narrative state visible through a warm hub while avoiding randomised affection or manipulative character timers.
- Stable destination ID: hearth_regulars; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .hearth regulars and /hearth regulars are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Meet a regular and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the regulars' circle.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A roster of authored Tavern NPCs whose dialogue reflects world events, completed chapters and the player's chosen interests.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Meet a regular, select a dialogue topic, receive a clue or story response, unlock a journal note and return later.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Regulars' Circle is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: Dialogue flags, character lore entries and contextual leads.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- turns player needs and discovered demand into transparent Bazaar browsing: Item Provenance Cards (bazaar_provenance).
- converts rumours, NPC conversations and notice-board entries into optional quest starts: The Quest Board (quest_board).
- provides immediate short-session games between longer adventures: Puzzle Generator and Validator (arcade_puzzle_factory).
- gives friends and parties a shared meeting point without requiring a public conversation: Positive Contribution Notes (fellowship_commendations).
### 12 · Incoming system relationships
- Item Provenance Cards (bazaar_provenance) can use this system because it makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands.
- Story Remix Paths (quest_remix) can use this system because it surfaces rumours and story contracts in a calm central hub.
- Optional Daily Logic Desk (arcade_daily_logic) can use this system because it lets players start a short game from a Tavern table without navigating a separate service.
- World Restoration Projects (festival_world_projects) can use this system because it surfaces a festival calmly through the Tavern and player-selected digests.
### 13 · Reward and economy contract
- Reward policy: Dialogue flags, character lore entries and contextual leads.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the regulars' circle view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the hearth_regulars rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the regulars' circle: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes narrative state visible through a warm hub while avoiding randomised affection or manipulative character timers.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# hearth_renovation — The Common Room Project
> Node 9 of 10 in **The Hearth and Tavern** · World Loom identity `hearth_renovation` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A cooperative decoration plan that lets a server vote on and contribute to permanent Tavern improvements.
- Why this belongs in Moonbyte: It creates a shared goal whose output is visible to everyone and does not require daily check-ins.
- Distinctive interaction: Review a published renovation plan, choose a contribution, add earned materials, inspect progress and vote on a completed design.
- Successful completion produces: A permanent room upgrade, contribution record and new optional interaction point.
### 02 · Purpose and boundaries
- Product role: The Tavern is Moonbyte's social home: the default place to discover activities, form parties, meet authored characters, review stories and step into the wider world.
- This system is the canonical owner of the common room project behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Hearth and Tavern; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates a shared goal whose output is visible to everyone and does not require daily check-ins.
- Stable destination ID: hearth_renovation; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .hearth renovation and /hearth renovation are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Review a published renovation plan and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the common room project.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A cooperative decoration plan that lets a server vote on and contribute to permanent Tavern improvements.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Review a published renovation plan, choose a contribution, add earned materials, inspect progress and vote on a completed design.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Common Room Project is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A permanent room upgrade, contribution record and new optional interaction point.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- turns player needs and discovered demand into transparent Bazaar browsing: The Delivery Board (bazaar_delivery_board).
- converts rumours, NPC conversations and notice-board entries into optional quest starts: Chapter Arcs (quest_arcs).
- provides immediate short-session games between longer adventures: The Party Game Shelf (arcade_party_collection).
- gives friends and parties a shared meeting point without requiring a public conversation: Shared Session Memories (fellowship_memories).
### 12 · Incoming system relationships
- The Delivery Board (bazaar_delivery_board) can use this system because it makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands.
- The Quest Board (quest_board) can use this system because it surfaces rumours and story contracts in a calm central hub.
- The Practice School (arcade_skill_school) can use this system because it lets players start a short game from a Tavern table without navigating a separate service.
- Puzzle Rotation Archive (festival_rotating_puzzles) can use this system because it surfaces a festival calmly through the Tavern and player-selected digests.
### 13 · Reward and economy contract
- Reward policy: A permanent room upgrade, contribution record and new optional interaction point.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the common room project view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the hearth_renovation rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the common room project: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates a shared goal whose output is visible to everyone and does not require daily check-ins.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# hearth_routes — The Tavern Wayfinder
> Node 10 of 10 in **The Hearth and Tavern** · World Loom identity `hearth_routes` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A contextual navigation panel that remembers a player's preferred destinations and shows useful next steps from the hub.
- Why this belongs in Moonbyte: It reduces command friction and lets a very large game remain understandable from Discord.
- Distinctive interaction: Open the wayfinder, choose a destination, inspect its prerequisites, pin or launch it, then return to the same hub context.
- Successful completion produces: Saved navigation preferences and consistent deep links into the shared world graph.
### 02 · Purpose and boundaries
- Product role: The Tavern is Moonbyte's social home: the default place to discover activities, form parties, meet authored characters, review stories and step into the wider world.
- This system is the canonical owner of the tavern wayfinder behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Hearth and Tavern; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It reduces command friction and lets a very large game remain understandable from Discord.
- Stable destination ID: hearth_routes; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .hearth routes and /hearth routes are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open the wayfinder and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the tavern wayfinder.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A contextual navigation panel that remembers a player's preferred destinations and shows useful next steps from the hub.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open the wayfinder, choose a destination, inspect its prerequisites, pin or launch it, then return to the same hub context.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Tavern Wayfinder is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: Saved navigation preferences and consistent deep links into the shared world graph.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- turns player needs and discovered demand into transparent Bazaar browsing: The Trade Review Desk (bazaar_review_court).
- converts rumours, NPC conversations and notice-board entries into optional quest starts: Choice and Consequence Graph (quest_branches).
- provides immediate short-session games between longer adventures: Asynchronous Challenge Cards (arcade_async_challenges).
- gives friends and parties a shared meeting point without requiring a public conversation: Friend Discovery Routes (fellowship_discovery).
### 12 · Incoming system relationships
- The Trade Review Desk (bazaar_review_court) can use this system because it makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands.
- Chapter Arcs (quest_arcs) can use this system because it surfaces rumours and story contracts in a calm central hub.
- Puzzle Generator and Validator (arcade_puzzle_factory) can use this system because it lets players start a short game from a Tavern table without navigating a separate service.
- The Celebration Wall (festival_celebration_wall) can use this system because it surfaces a festival calmly through the Tavern and player-selected digests.
### 13 · Reward and economy contract
- Reward policy: Saved navigation preferences and consistent deep links into the shared world graph.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the tavern wayfinder view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the hearth_routes rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the tavern wayfinder: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It reduces command friction and lets a very large game remain understandable from Discord.; it must remain satisfying in a bounded session and support a natural stopping point.
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

## Domain 2 — The Bazaar and Exchange

The Bazaar circulates earned goods through understandable fixed-price listings, barter and contracts. It is a utility for players, never a speculative or wagering market.

**Domain dependencies:** The Hearth and Tavern (`hearth`) · Gathering, Crafting and Workshops (`craft`) · Collections, Bestiary and Museum (`archive`) · Fair Play, Competition and Verification (`fairness`)

# bazaar_stallfronts — Player Stallfronts
> Node 1 of 10 in **The Bazaar and Exchange** · World Loom identity `bazaar_stallfronts` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Each player can operate a small, customisable stall that presents eligible goods, fixed prices and a short description.
- Why this belongs in Moonbyte: It gives trading a social identity while keeping the transaction model simple and inspectable.
- Distinctive interaction: Create a stall, choose a permitted theme, select eligible stock, publish listings, answer requests and manage completed orders.
- Successful completion produces: A discoverable storefront, active listing set and stall history.
### 02 · Purpose and boundaries
- Product role: The Bazaar circulates earned goods through understandable fixed-price listings, barter and contracts. It is a utility for players, never a speculative or wagering market.
- This system is the canonical owner of player stallfronts behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Bazaar and Exchange; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives trading a social identity while keeping the transaction model simple and inspectable.
- Stable destination ID: bazaar_stallfronts; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .bazaar stallfronts and /bazaar stallfronts are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Create a stall and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to player stallfronts.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Each player can operate a small, customisable stall that presents eligible goods, fixed prices and a short description.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Create a stall, choose a permitted theme, select eligible stock, publish listings, answer requests and manage completed orders.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Player Stallfronts is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A discoverable storefront, active listing set and stall history.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands: The Lantern and Ladle (hearth_main_room).
- connects player demand to published recipes, ingredient lists and crafting work orders: Known Quality Grades (craft_quality).
- uses canonical item definitions and collection rules to determine what can be listed: The Cosmetic Gallery (archive_cosmetics).
- uses atomic escrow, append-only receipts and deterministic ledger rules: Result Appeal Desk (fairness_appeals).
### 12 · Incoming system relationships
- The Lantern and Ladle (hearth_main_room) can use this system because it turns player needs and discovered demand into transparent Bazaar browsing.
- The Maker's Workshop (craft_workshop) can use this system because it fulfils wanted-board requests and work orders through known recipe outputs.
- Match Review Transcript (fairness_match_review) can use this system because it uses atomic ledgers and receipts for every trade and escrow transition.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A discoverable storefront, active listing set and stall history.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive player stallfronts view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the bazaar_stallfronts rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for player stallfronts: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives trading a social identity while keeping the transaction model simple and inspectable.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# bazaar_fixed_listings — Fixed-Price Listings
> Node 2 of 10 in **The Bazaar and Exchange** · World Loom identity `bazaar_fixed_listings` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Every listing states one exact item, quantity, total price, expiry and seller before a buyer commits.
- Why this belongs in Moonbyte: It removes hidden pricing, bid pressure and speculative auction behaviour from the economy.
- Distinctive interaction: Inspect the item, check seller and total, confirm purchase, receive a receipt and see the inventory update.
- Successful completion produces: An atomic trade record and a deterministic transfer of item and Moonbits.
### 02 · Purpose and boundaries
- Product role: The Bazaar circulates earned goods through understandable fixed-price listings, barter and contracts. It is a utility for players, never a speculative or wagering market.
- This system is the canonical owner of fixed-price listings behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Bazaar and Exchange; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It removes hidden pricing, bid pressure and speculative auction behaviour from the economy.
- Stable destination ID: bazaar_fixed_listings; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .bazaar fixed listings and /bazaar fixed listings are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect the item and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to fixed-price listings.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Every listing states one exact item, quantity, total price, expiry and seller before a buyer commits.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect the item, check seller and total, confirm purchase, receive a receipt and see the inventory update.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Fixed-Price Listings is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: An atomic trade record and a deterministic transfer of item and Moonbits.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands: The Notice Wall (hearth_notice_wall).
- connects player demand to published recipes, ingredient lists and crafting work orders: Commission Bench (craft_commissions).
- uses canonical item definitions and collection rules to determine what can be listed: The Living Lore Index (archive_lore).
- uses atomic escrow, append-only receipts and deterministic ledger rules: Published Rulebooks (fairness_rulebooks).
### 12 · Incoming system relationships
- The Notice Wall (hearth_notice_wall) can use this system because it turns player needs and discovered demand into transparent Bazaar browsing.
- Known Quality Grades (craft_quality) can use this system because it fulfils wanted-board requests and work orders through known recipe outputs.
- Integrity Guardrails (fairness_anti_abuse) can use this system because it uses atomic ledgers and receipts for every trade and escrow transition.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: An atomic trade record and a deterministic transfer of item and Moonbits.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive fixed-price listings view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the bazaar_fixed_listings rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for fixed-price listings: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It removes hidden pricing, bid pressure and speculative auction behaviour from the economy.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# bazaar_barter_desk — The Barter Desk
> Node 3 of 10 in **The Bazaar and Exchange** · World Loom identity `bazaar_barter_desk` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can propose a transparent exchange of named items and optional Moonbits with both sides seeing the complete offer.
- Why this belongs in Moonbyte: It supports flexible trading without hidden valuations or gambling-like bid mechanics.
- Distinctive interaction: Select owned eligible items, build an offer, send it privately, inspect the counter-offer and confirm both sides.
- Successful completion produces: A bilateral trade proposal with expiry, escrow and a signed-in-system receipt.
### 02 · Purpose and boundaries
- Product role: The Bazaar circulates earned goods through understandable fixed-price listings, barter and contracts. It is a utility for players, never a speculative or wagering market.
- This system is the canonical owner of the barter desk behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Bazaar and Exchange; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports flexible trading without hidden valuations or gambling-like bid mechanics.
- Stable destination ID: bazaar_barter_desk; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .bazaar barter desk and /bazaar barter desk are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select owned eligible items and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the barter desk.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can propose a transparent exchange of named items and optional Moonbits with both sides seeing the complete offer.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select owned eligible items, build an offer, send it privately, inspect the counter-offer and confirm both sides.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Barter Desk is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A bilateral trade proposal with expiry, escrow and a signed-in-system receipt.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands: The Rumour Circle (hearth_rumour_circle).
- connects player demand to published recipes, ingredient lists and crafting work orders: Material Reclaim (craft_recycling).
- uses canonical item definitions and collection rules to determine what can be listed: Collection Milestones (archive_milestones).
- uses atomic escrow, append-only receipts and deterministic ledger rules: Commit and Reveal Service (fairness_commit_reveal).
### 12 · Incoming system relationships
- The Rumour Circle (hearth_rumour_circle) can use this system because it turns player needs and discovered demand into transparent Bazaar browsing.
- Commission Bench (craft_commissions) can use this system because it fulfils wanted-board requests and work orders through known recipe outputs.
- Tournament Rules Desk (fairness_tournament) can use this system because it uses atomic ledgers and receipts for every trade and escrow transition.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A bilateral trade proposal with expiry, escrow and a signed-in-system receipt.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the barter desk view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the bazaar_barter_desk rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the barter desk: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports flexible trading without hidden valuations or gambling-like bid mechanics.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# bazaar_wanted_board — The Wanted Board
> Node 4 of 10 in **The Bazaar and Exchange** · World Loom identity `bazaar_wanted_board` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Buyers post specific requests with desired quantities, accepted variants and a maximum disclosed price.
- Why this belongs in Moonbyte: It makes the Bazaar demand-led and connects collectors, crafters and explorers.
- Distinctive interaction: Create a request, publish constraints, receive matching offers, compare provenance and accept one complete offer.
- Successful completion produces: A fulfilled request, request status and a traceable connection to the source activity.
### 02 · Purpose and boundaries
- Product role: The Bazaar circulates earned goods through understandable fixed-price listings, barter and contracts. It is a utility for players, never a speculative or wagering market.
- This system is the canonical owner of the wanted board behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Bazaar and Exchange; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes the Bazaar demand-led and connects collectors, crafters and explorers.
- Stable destination ID: bazaar_wanted_board; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .bazaar wanted board and /bazaar wanted board are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Create a request and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the wanted board.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Buyers post specific requests with desired quantities, accepted variants and a maximum disclosed price.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Create a request, publish constraints, receive matching offers, compare provenance and accept one complete offer.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Wanted Board is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A fulfilled request, request status and a traceable connection to the source activity.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands: The Common Tables (hearth_table_games).
- connects player demand to published recipes, ingredient lists and crafting work orders: Workshop Repair (craft_repair).
- uses canonical item definitions and collection rules to determine what can be listed: Collector's Provenance View (archive_provenance).
- uses atomic escrow, append-only receipts and deterministic ledger rules: Scoring Contract Registry (fairness_scoring).
### 12 · Incoming system relationships
- The Common Tables (hearth_table_games) can use this system because it turns player needs and discovered demand into transparent Bazaar browsing.
- Material Reclaim (craft_recycling) can use this system because it fulfils wanted-board requests and work orders through known recipe outputs.
- Leaderboard Integrity (fairness_leaderboard) can use this system because it uses atomic ledgers and receipts for every trade and escrow transition.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A fulfilled request, request status and a traceable connection to the source activity.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the wanted board view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the bazaar_wanted_board rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the wanted board: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes the Bazaar demand-led and connects collectors, crafters and explorers.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# bazaar_work_orders — Crafting Work Orders
> Node 5 of 10 in **The Bazaar and Exchange** · World Loom identity `bazaar_work_orders` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A buyer commissions a known recipe from a crafter using published inputs, fixed compensation and a delivery deadline chosen by agreement.
- Why this belongs in Moonbyte: It turns crafting into a community service rather than a random loot chase.
- Distinctive interaction: Choose a recipe, agree inputs and price, reserve materials, craft the item, deliver and release escrow.
- Successful completion produces: A completed commission, recipe version, material provenance and clear settlement.
### 02 · Purpose and boundaries
- Product role: The Bazaar circulates earned goods through understandable fixed-price listings, barter and contracts. It is a utility for players, never a speculative or wagering market.
- This system is the canonical owner of crafting work orders behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Bazaar and Exchange; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It turns crafting into a community service rather than a random loot chase.
- Stable destination ID: bazaar_work_orders; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .bazaar work orders and /bazaar work orders are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a recipe and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to crafting work orders.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A buyer commissions a known recipe from a crafter using published inputs, fixed compensation and a delivery deadline chosen by agreement.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a recipe, agree inputs and price, reserve materials, craft the item, deliver and release escrow.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Crafting Work Orders is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A completed commission, recipe version, material provenance and clear settlement.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands: The Host's Bench (hearth_hosts_bench).
- connects player demand to published recipes, ingredient lists and crafting work orders: Pattern and Set Assembly (craft_sets).
- uses canonical item definitions and collection rules to determine what can be listed: The Grand Collection Index (archive_catalogue).
- uses atomic escrow, append-only receipts and deterministic ledger rules: Optional Skill Rating (fairness_ratings).
### 12 · Incoming system relationships
- The Host's Bench (hearth_hosts_bench) can use this system because it turns player needs and discovered demand into transparent Bazaar browsing.
- Workshop Repair (craft_repair) can use this system because it fulfils wanted-board requests and work orders through known recipe outputs.
- Equal Conditions Profiles (fairness_access) can use this system because it uses atomic ledgers and receipts for every trade and escrow transition.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A completed commission, recipe version, material provenance and clear settlement.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive crafting work orders view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the bazaar_work_orders rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for crafting work orders: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It turns crafting into a community service rather than a random loot chase.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# bazaar_price_ledger — The Price Reference Ledger
> Node 6 of 10 in **The Bazaar and Exchange** · World Loom identity `bazaar_price_ledger` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A non-speculative reference view summarises recent completed fixed-price trades and marks small samples clearly.
- Why this belongs in Moonbyte: It helps players understand ordinary prices without presenting items as investments.
- Distinctive interaction: Inspect a stable item definition, view completed-trade bands, check sample size and return to the actual listing.
- Successful completion produces: Aggregate reference information with privacy-safe counts and explicit uncertainty.
### 02 · Purpose and boundaries
- Product role: The Bazaar circulates earned goods through understandable fixed-price listings, barter and contracts. It is a utility for players, never a speculative or wagering market.
- This system is the canonical owner of the price reference ledger behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Bazaar and Exchange; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It helps players understand ordinary prices without presenting items as investments.
- Stable destination ID: bazaar_price_ledger; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .bazaar price ledger and /bazaar price ledger are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect a stable item definition and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the price reference ledger.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A non-speculative reference view summarises recent completed fixed-price trades and marks small samples clearly.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect a stable item definition, view completed-trade bands, check sample size and return to the actual listing.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Price Reference Ledger is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: Aggregate reference information with privacy-safe counts and explicit uncertainty.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands: The Story Stage (hearth_bard_stage).
- connects player demand to published recipes, ingredient lists and crafting work orders: Guild Workshop (craft_shared_bench).
- uses canonical item definitions and collection rules to determine what can be listed: The Field Bestiary (archive_bestiary).
- uses atomic escrow, append-only receipts and deterministic ledger rules: Match Review Transcript (fairness_match_review).
### 12 · Incoming system relationships
- The Story Stage (hearth_bard_stage) can use this system because it turns player needs and discovered demand into transparent Bazaar browsing.
- Pattern and Set Assembly (craft_sets) can use this system because it fulfils wanted-board requests and work orders through known recipe outputs.
- Result Appeal Desk (fairness_appeals) can use this system because it uses atomic ledgers and receipts for every trade and escrow transition.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: Aggregate reference information with privacy-safe counts and explicit uncertainty.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the price reference ledger view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the bazaar_price_ledger rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the price reference ledger: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It helps players understand ordinary prices without presenting items as investments.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# bazaar_escrow_vault — The Escrow Vault
> Node 7 of 10 in **The Bazaar and Exchange** · World Loom identity `bazaar_escrow_vault` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A transaction state machine reserves goods and currency during a trade, then commits or refunds everything exactly once.
- Why this belongs in Moonbyte: It protects both parties from duplicate clicks, disconnects and concurrent purchases.
- Distinctive interaction: Create an offer, reserve the exact assets, confirm both parties, atomically commit or expire and refund.
- Successful completion produces: A recoverable escrow record and an immutable trade receipt.
### 02 · Purpose and boundaries
- Product role: The Bazaar circulates earned goods through understandable fixed-price listings, barter and contracts. It is a utility for players, never a speculative or wagering market.
- This system is the canonical owner of the escrow vault behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Bazaar and Exchange; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It protects both parties from duplicate clicks, disconnects and concurrent purchases.
- Stable destination ID: bazaar_escrow_vault; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .bazaar escrow vault and /bazaar escrow vault are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Create an offer and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the escrow vault.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A transaction state machine reserves goods and currency during a trade, then commits or refunds everything exactly once.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Create an offer, reserve the exact assets, confirm both parties, atomically commit or expire and refund.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Escrow Vault is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A recoverable escrow record and an immutable trade receipt.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands: The Hearth Kitchen (hearth_kitchen).
- connects player demand to published recipes, ingredient lists and crafting work orders: Recipe Research (craft_discovery).
- uses canonical item definitions and collection rules to determine what can be listed: The Relic Archive (archive_relics).
- uses atomic escrow, append-only receipts and deterministic ledger rules: Integrity Guardrails (fairness_anti_abuse).
### 12 · Incoming system relationships
- The Hearth Kitchen (hearth_kitchen) can use this system because it turns player needs and discovered demand into transparent Bazaar browsing.
- Guild Workshop (craft_shared_bench) can use this system because it fulfils wanted-board requests and work orders through known recipe outputs.
- Published Rulebooks (fairness_rulebooks) can use this system because it uses atomic ledgers and receipts for every trade and escrow transition.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A recoverable escrow record and an immutable trade receipt.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the escrow vault view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the bazaar_escrow_vault rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the escrow vault: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It protects both parties from duplicate clicks, disconnects and concurrent purchases.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# bazaar_provenance — Item Provenance Cards
> Node 8 of 10 in **The Bazaar and Exchange** · World Loom identity `bazaar_provenance` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Each eligible item can show its origin, definition version, craft recipe and earned history without exposing private player data.
- Why this belongs in Moonbyte: It makes ownership trustworthy and allows trade eligibility to be explained.
- Distinctive interaction: Inspect an item card, read its source, check restrictions, view public-safe history and list or keep it.
- Successful completion produces: A provenance chain attached to the stable item identity.
### 02 · Purpose and boundaries
- Product role: The Bazaar circulates earned goods through understandable fixed-price listings, barter and contracts. It is a utility for players, never a speculative or wagering market.
- This system is the canonical owner of item provenance cards behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Bazaar and Exchange; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes ownership trustworthy and allows trade eligibility to be explained.
- Stable destination ID: bazaar_provenance; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .bazaar provenance and /bazaar provenance are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect an item card and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to item provenance cards.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Each eligible item can show its origin, definition version, craft recipe and earned history without exposing private player data.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect an item card, read its source, check restrictions, view public-safe history and list or keep it.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Item Provenance Cards is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A provenance chain attached to the stable item identity.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands: The Regulars' Circle (hearth_regulars).
- connects player demand to published recipes, ingredient lists and crafting work orders: Field Gathering (craft_gathering).
- uses canonical item definitions and collection rules to determine what can be listed: The Illustrated Compendium (archive_cards).
- uses atomic escrow, append-only receipts and deterministic ledger rules: Tournament Rules Desk (fairness_tournament).
### 12 · Incoming system relationships
- The Regulars' Circle (hearth_regulars) can use this system because it turns player needs and discovered demand into transparent Bazaar browsing.
- Recipe Research (craft_discovery) can use this system because it fulfils wanted-board requests and work orders through known recipe outputs.
- Commit and Reveal Service (fairness_commit_reveal) can use this system because it uses atomic ledgers and receipts for every trade and escrow transition.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A provenance chain attached to the stable item identity.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive item provenance cards view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the bazaar_provenance rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for item provenance cards: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes ownership trustworthy and allows trade eligibility to be explained.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# bazaar_delivery_board — The Delivery Board
> Node 9 of 10 in **The Bazaar and Exchange** · World Loom identity `bazaar_delivery_board` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A fulfilment queue tracks accepted orders, delivery state, collection points and expiry without requiring players to stay online.
- Why this belongs in Moonbyte: It completes the market loop for commissions and asynchronous trades.
- Distinctive interaction: Accept an order, fulfil the listed conditions, mark ready, let the receiver claim it and archive the receipt.
- Successful completion produces: A deterministic delivery state with a clear pickup or cancellation path.
### 02 · Purpose and boundaries
- Product role: The Bazaar circulates earned goods through understandable fixed-price listings, barter and contracts. It is a utility for players, never a speculative or wagering market.
- This system is the canonical owner of the delivery board behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Bazaar and Exchange; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It completes the market loop for commissions and asynchronous trades.
- Stable destination ID: bazaar_delivery_board; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .bazaar delivery board and /bazaar delivery board are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Accept an order and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the delivery board.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A fulfilment queue tracks accepted orders, delivery state, collection points and expiry without requiring players to stay online.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Accept an order, fulfil the listed conditions, mark ready, let the receiver claim it and archive the receipt.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Delivery Board is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A deterministic delivery state with a clear pickup or cancellation path.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands: The Common Room Project (hearth_renovation).
- connects player demand to published recipes, ingredient lists and crafting work orders: The Recipe Codex (craft_recipe_book).
- uses canonical item definitions and collection rules to determine what can be listed: The Museum Hall (archive_museum).
- uses atomic escrow, append-only receipts and deterministic ledger rules: Leaderboard Integrity (fairness_leaderboard).
### 12 · Incoming system relationships
- The Common Room Project (hearth_renovation) can use this system because it turns player needs and discovered demand into transparent Bazaar browsing.
- Field Gathering (craft_gathering) can use this system because it fulfils wanted-board requests and work orders through known recipe outputs.
- Scoring Contract Registry (fairness_scoring) can use this system because it uses atomic ledgers and receipts for every trade and escrow transition.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A deterministic delivery state with a clear pickup or cancellation path.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the delivery board view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the bazaar_delivery_board rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the delivery board: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It completes the market loop for commissions and asynchronous trades.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# bazaar_review_court — The Trade Review Desk
> Node 10 of 10 in **The Bazaar and Exchange** · World Loom identity `bazaar_review_court` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A bounded dispute workflow lets users report an incorrect fulfilment and gives moderators an evidence-only review view.
- Why this belongs in Moonbyte: It provides recourse without making informal moderator judgement part of normal transaction execution.
- Distinctive interaction: Open a receipt, choose a factual issue, attach allowed evidence references, receive a review state and see the resolution.
- Successful completion produces: A private case record, moderator outcome and audited correction if one is justified.
### 02 · Purpose and boundaries
- Product role: The Bazaar circulates earned goods through understandable fixed-price listings, barter and contracts. It is a utility for players, never a speculative or wagering market.
- This system is the canonical owner of the trade review desk behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: The Bazaar and Exchange; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It provides recourse without making informal moderator judgement part of normal transaction execution.
- Stable destination ID: bazaar_review_court; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .bazaar review court and /bazaar review court are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open a receipt and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the trade review desk.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A bounded dispute workflow lets users report an incorrect fulfilment and gives moderators an evidence-only review view.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open a receipt, choose a factual issue, attach allowed evidence references, receive a review state and see the resolution.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Trade Review Desk is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A private case record, moderator outcome and audited correction if one is justified.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- makes stalls and requests discoverable from the Tavern rather than requiring users to memorise commands: The Tavern Wayfinder (hearth_routes).
- connects player demand to published recipes, ingredient lists and crafting work orders: The Maker's Workshop (craft_workshop).
- uses canonical item definitions and collection rules to determine what can be listed: Collection Sets (archive_sets).
- uses atomic escrow, append-only receipts and deterministic ledger rules: Equal Conditions Profiles (fairness_access).
### 12 · Incoming system relationships
- The Tavern Wayfinder (hearth_routes) can use this system because it turns player needs and discovered demand into transparent Bazaar browsing.
- The Recipe Codex (craft_recipe_book) can use this system because it fulfils wanted-board requests and work orders through known recipe outputs.
- Optional Skill Rating (fairness_ratings) can use this system because it uses atomic ledgers and receipts for every trade and escrow transition.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A private case record, moderator outcome and audited correction if one is justified.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the trade review desk view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the bazaar_review_court rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the trade review desk: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It provides recourse without making informal moderator judgement part of normal transaction execution.; it must remain satisfying in a bounded session and support a natural stopping point.
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

## Domain 3 — Guilds and Long-Form Cooperation

Guilds coordinate shared adventures, persistent projects and optional social identity while individual ownership and server boundaries remain explicit.

**Domain dependencies:** Server Culture and Community Layer (`server`) · Expeditions and Exploration Sessions (`expedition`) · Vaults, Dungeons and Puzzle Spaces (`vault`) · Progression, Mastery and Recognition (`mastery`)

# guild_charters — Guild Charters
> Node 1 of 10 in **Guilds and Long-Form Cooperation** · World Loom identity `guild_charters` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A guild begins with a clear purpose, membership rules, privacy settings and a visible list of shared goals.
- Why this belongs in Moonbyte: It creates a real community structure rather than a cosmetic tag attached to a leaderboard.
- Distinctive interaction: Draft a charter, preview permissions, invite members, record agreement and publish the guild identity.
- Successful completion produces: A versioned guild charter and a recoverable membership boundary.
### 02 · Purpose and boundaries
- Product role: Guilds coordinate shared adventures, persistent projects and optional social identity while individual ownership and server boundaries remain explicit.
- This system is the canonical owner of guild charters behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Guilds and Long-Form Cooperation; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates a real community structure rather than a cosmetic tag attached to a leaderboard.
- Stable destination ID: guild_charters; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .guild charters and /guild charters are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Draft a charter and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to guild charters.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A guild begins with a clear purpose, membership rules, privacy settings and a visible list of shared goals.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Draft a charter, preview permissions, invite members, record agreement and publish the guild identity.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Guild Charters is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned guild charter and a recoverable membership boundary.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- binds guild identity and moderation to the correct Discord server: Server World Identity (server_identity).
- forms parties for map-based expeditions and cooperative routes: Prepared Supply Kit (expedition_supplies).
- provides the membership and contribution model for Guild Vault challenges: Split-Path Chambers (vault_parallel_rooms).
- turns verified participation and skill accomplishments into transparent guild recognition: Account-Bound Recognition (mastery_account_rewards).
### 12 · Incoming system relationships
- Server World Identity (server_identity) can use this system because it provides the Discord server boundary and permission context for guild features.
- Expedition Roles (expedition_roles) can use this system because it allows guilds to coordinate asynchronous contributions and party membership.
- Mastery Trial Rooms (vault_mastery_trials) can use this system because it hosts shared roles and parallel clue paths for Guild Vaults.
- Friendly Tournament Brackets (multiplayer_brackets) can use this system because it allows guild members to coordinate long-running group activities.
### 13 · Reward and economy contract
- Reward policy: A versioned guild charter and a recoverable membership boundary.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive guild charters view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the guild_charters rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for guild charters: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates a real community structure rather than a cosmetic tag attached to a leaderboard.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# guild_hall — The Guild Hall
> Node 2 of 10 in **Guilds and Long-Form Cooperation** · World Loom identity `guild_hall` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Each guild gets a shared visual hall with its banner, current project, member roles, notices and recent public achievements.
- Why this belongs in Moonbyte: It gives members one coherent place to coordinate without exposing private player inventories.
- Distinctive interaction: Open the hall, inspect its boards, choose a project, view allowed member details and return to a party or world area.
- Successful completion produces: A guild-scoped view model and persistent hall configuration.
### 02 · Purpose and boundaries
- Product role: Guilds coordinate shared adventures, persistent projects and optional social identity while individual ownership and server boundaries remain explicit.
- This system is the canonical owner of the guild hall behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Guilds and Long-Form Cooperation; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives members one coherent place to coordinate without exposing private player inventories.
- Stable destination ID: guild_hall; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .guild hall and /guild hall are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open the hall and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the guild hall.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Each guild gets a shared visual hall with its banner, current project, member roles, notices and recent public achievements.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open the hall, inspect its boards, choose a project, view allowed member details and return to a party or world area.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Guild Hall is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A guild-scoped view model and persistent hall configuration.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- binds guild identity and moderation to the correct Discord server: Community Calendar (server_calendar).
- forms parties for map-based expeditions and cooperative routes: Waystation Checkpoints (expedition_camps).
- provides the membership and contribution model for Guild Vault challenges: Vault Save Stones (vault_checkpoints).
- turns verified participation and skill accomplishments into transparent guild recognition: Experience and Milestone Ledger (mastery_experience).
### 12 · Incoming system relationships
- Community Calendar (server_calendar) can use this system because it provides the Discord server boundary and permission context for guild features.
- Prepared Supply Kit (expedition_supplies) can use this system because it allows guilds to coordinate asynchronous contributions and party membership.
- The Grand Mechanism (vault_boss_logic) can use this system because it hosts shared roles and parallel clue paths for Guild Vaults.
- Shared Cooperative Boards (multiplayer_coop_boards) can use this system because it allows guild members to coordinate long-running group activities.
### 13 · Reward and economy contract
- Reward policy: A guild-scoped view model and persistent hall configuration.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the guild hall view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the guild_hall rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the guild hall: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives members one coherent place to coordinate without exposing private player inventories.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# guild_roles — Guild Role Charter
> Node 3 of 10 in **Guilds and Long-Form Cooperation** · World Loom identity `guild_roles` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Guild leaders can define bounded roles such as organiser, builder, archivist and moderator with specific permissions.
- Why this belongs in Moonbyte: It limits arbitrary power and makes responsibilities understandable to members.
- Distinctive interaction: Review a role template, assign a member, preview permitted actions, audit changes and revoke access when needed.
- Successful completion produces: A least-privilege role assignment and auditable permission history.
### 02 · Purpose and boundaries
- Product role: Guilds coordinate shared adventures, persistent projects and optional social identity while individual ownership and server boundaries remain explicit.
- This system is the canonical owner of guild role charter behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Guilds and Long-Form Cooperation; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It limits arbitrary power and makes responsibilities understandable to members.
- Stable destination ID: guild_roles; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .guild roles and /guild roles are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Review a role template and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to guild role charter.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Guild leaders can define bounded roles such as organiser, builder, archivist and moderator with specific permissions.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Review a role template, assign a member, preview permitted actions, audit changes and revoke access when needed.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Guild Role Charter is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A least-privilege role assignment and auditable permission history.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- binds guild identity and moderation to the correct Discord server: Channel Wayfinding (server_channels).
- forms parties for map-based expeditions and cooperative routes: The Expedition Journal (expedition_journal).
- provides the membership and contribution model for Guild Vault challenges: Fair Difficulty Profiles (vault_difficulty).
- turns verified participation and skill accomplishments into transparent guild recognition: Skill Mastery Tracks (mastery_tracks).
### 12 · Incoming system relationships
- Channel Wayfinding (server_channels) can use this system because it provides the Discord server boundary and permission context for guild features.
- Waystation Checkpoints (expedition_camps) can use this system because it allows guilds to coordinate asynchronous contributions and party membership.
- Split-Path Chambers (vault_parallel_rooms) can use this system because it hosts shared roles and parallel clue paths for Guild Vaults.
- Reconnect and Rejoin (multiplayer_reconnect) can use this system because it allows guild members to coordinate long-running group activities.
### 13 · Reward and economy contract
- Reward policy: A least-privilege role assignment and auditable permission history.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive guild role charter view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the guild_roles rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for guild role charter: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It limits arbitrary power and makes responsibilities understandable to members.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# guild_projects — The Great Works Board
> Node 4 of 10 in **Guilds and Long-Form Cooperation** · World Loom identity `guild_projects` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Guild members jointly complete long-term construction, mapping, archive or puzzle projects with visible contribution categories.
- Why this belongs in Moonbyte: It gives cooperation a meaningful output that persists beyond one match.
- Distinctive interaction: Choose a work, inspect required contributions, complete one task, review team progress and vote on the finished result.
- Successful completion produces: A guild project state, contribution ledger and world-visible completion.
### 02 · Purpose and boundaries
- Product role: Guilds coordinate shared adventures, persistent projects and optional social identity while individual ownership and server boundaries remain explicit.
- This system is the canonical owner of the great works board behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Guilds and Long-Form Cooperation; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives cooperation a meaningful output that persists beyond one match.
- Stable destination ID: guild_projects; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .guild projects and /guild projects are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a work and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the great works board.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Guild members jointly complete long-term construction, mapping, archive or puzzle projects with visible contribution categories.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a work, inspect required contributions, complete one task, review team progress and vote on the finished result.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Great Works Board is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A guild project state, contribution ledger and world-visible completion.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- binds guild identity and moderation to the correct Discord server: Server Challenge Board (server_challenges).
- forms parties for map-based expeditions and cooperative routes: Return and Recovery Routes (expedition_recovery).
- provides the membership and contribution model for Guild Vault challenges: Verified Puzzle Registry (vault_solution_registry).
- turns verified participation and skill accomplishments into transparent guild recognition: Achievement Registry (mastery_achievements).
### 12 · Incoming system relationships
- Server Challenge Board (server_challenges) can use this system because it provides the Discord server boundary and permission context for guild features.
- The Expedition Journal (expedition_journal) can use this system because it allows guilds to coordinate asynchronous contributions and party membership.
- Vault Save Stones (vault_checkpoints) can use this system because it hosts shared roles and parallel clue paths for Guild Vaults.
- Table Votes and Host Controls (multiplayer_table_votes) can use this system because it allows guild members to coordinate long-running group activities.
### 13 · Reward and economy contract
- Reward policy: A guild project state, contribution ledger and world-visible completion.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the great works board view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the guild_projects rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the great works board: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives cooperation a meaningful output that persists beyond one match.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# guild_quests — Guild Story Contracts
> Node 5 of 10 in **Guilds and Long-Form Cooperation** · World Loom identity `guild_quests` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A shared quest line lets a guild divide puzzle, exploration and crafting objectives without forcing everyone to be online together.
- Why this belongs in Moonbyte: It combines individual play sessions into one authored group story.
- Distinctive interaction: Accept the contract, assign optional branches, submit independently earned evidence and resolve the final shared objective.
- Successful completion produces: A quest graph with contribution attribution and idempotent rewards.
### 02 · Purpose and boundaries
- Product role: Guilds coordinate shared adventures, persistent projects and optional social identity while individual ownership and server boundaries remain explicit.
- This system is the canonical owner of guild story contracts behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Guilds and Long-Form Cooperation; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It combines individual play sessions into one authored group story.
- Stable destination ID: guild_quests; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .guild quests and /guild quests are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Accept the contract and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to guild story contracts.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A shared quest line lets a guild divide puzzle, exploration and crafting objectives without forcing everyone to be online together.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Accept the contract, assign optional branches, submit independently earned evidence and resolve the final shared objective.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Guild Story Contracts is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A quest graph with contribution attribution and idempotent rewards.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- binds guild identity and moderation to the correct Discord server: The Community Museum (server_museum).
- forms parties for map-based expeditions and cooperative routes: Explorer Contracts (expedition_contracts).
- provides the membership and contribution model for Guild Vault challenges: Vault Blueprint Library (vault_blueprints).
- turns verified participation and skill accomplishments into transparent guild recognition: Player Honour Record (mastery_reputation).
### 12 · Incoming system relationships
- The Community Museum (server_museum) can use this system because it provides the Discord server boundary and permission context for guild features.
- Return and Recovery Routes (expedition_recovery) can use this system because it allows guilds to coordinate asynchronous contributions and party membership.
- Fair Difficulty Profiles (vault_difficulty) can use this system because it hosts shared roles and parallel clue paths for Guild Vaults.
- Universal Lobby Contract (multiplayer_lobby) can use this system because it allows guild members to coordinate long-running group activities.
### 13 · Reward and economy contract
- Reward policy: A quest graph with contribution attribution and idempotent rewards.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive guild story contracts view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the guild_quests rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for guild story contracts: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It combines individual play sessions into one authored group story.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# guild_vault — The Guild Vault
> Node 6 of 10 in **Guilds and Long-Form Cooperation** · World Loom identity `guild_vault` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A cooperative challenge instance contains parallel clues, shared rooms and role-specific tasks whose rewards are known before entry.
- Why this belongs in Moonbyte: It makes the Vault system a social puzzle rather than a lottery chest.
- Distinctive interaction: Preview the vault, form a party, distribute clues, coordinate actions, save checkpoints and claim the fixed result.
- Successful completion produces: A guild-scoped vault run with validated contribution and fixed rewards.
### 02 · Purpose and boundaries
- Product role: Guilds coordinate shared adventures, persistent projects and optional social identity while individual ownership and server boundaries remain explicit.
- This system is the canonical owner of the guild vault behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Guilds and Long-Form Cooperation; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes the Vault system a social puzzle rather than a lottery chest.
- Stable destination ID: guild_vault; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .guild vault and /guild vault are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Preview the vault and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the guild vault.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A cooperative challenge instance contains parallel clues, shared rooms and role-specific tasks whose rewards are known before entry.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Preview the vault, form a party, distribute clues, coordinate actions, save checkpoints and claim the fixed result.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Guild Vault is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A guild-scoped vault run with validated contribution and fixed rewards.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- binds guild identity and moderation to the correct Discord server: Neighbourhood Goals (server_goals).
- forms parties for map-based expeditions and cooperative routes: Discovery Scorecard (expedition_score).
- provides the membership and contribution model for Guild Vault challenges: Puzzle Chambers (vault_chambers).
- turns verified participation and skill accomplishments into transparent guild recognition: Personal Milestone Contracts (mastery_contracts).
### 12 · Incoming system relationships
- Neighbourhood Goals (server_goals) can use this system because it provides the Discord server boundary and permission context for guild features.
- Explorer Contracts (expedition_contracts) can use this system because it allows guilds to coordinate asynchronous contributions and party membership.
- Verified Puzzle Registry (vault_solution_registry) can use this system because it hosts shared roles and parallel clue paths for Guild Vaults.
- Party Formation Desk (multiplayer_party) can use this system because it allows guild members to coordinate long-running group activities.
### 13 · Reward and economy contract
- Reward policy: A guild-scoped vault run with validated contribution and fixed rewards.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the guild vault view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the guild_vault rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the guild vault: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes the Vault system a social puzzle rather than a lottery chest.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# guild_map — The Guild Cartography Room
> Node 7 of 10 in **Guilds and Long-Form Cooperation** · World Loom identity `guild_map` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Members can annotate a shared map with verified discoveries, routes and authored notes subject to moderation.
- Why this belongs in Moonbyte: It makes exploration useful to the whole group while separating verified facts from speculation.
- Distinctive interaction: Open a region, add a private draft note, attach an evidence reference, submit it, then publish after review.
- Successful completion produces: A shared map layer with revision history and contributor attribution.
### 02 · Purpose and boundaries
- Product role: Guilds coordinate shared adventures, persistent projects and optional social identity while individual ownership and server boundaries remain explicit.
- This system is the canonical owner of the guild cartography room behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Guilds and Long-Form Cooperation; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes exploration useful to the whole group while separating verified facts from speculation.
- Stable destination ID: guild_map; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .guild map and /guild map are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open a region and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the guild cartography room.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Members can annotate a shared map with verified discoveries, routes and authored notes subject to moderation.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open a region, add a private draft note, attach an evidence reference, submit it, then publish after review.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Guild Cartography Room is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A shared map layer with revision history and contributor attribution.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- binds guild identity and moderation to the correct Discord server: Scoped Scoreboards (server_leaderboards).
- forms parties for map-based expeditions and cooperative routes: Expedition Route Archive (expedition_replay).
- provides the membership and contribution model for Guild Vault challenges: Logic Key Mechanisms (vault_logic_keys).
- turns verified participation and skill accomplishments into transparent guild recognition: Milestone Route Planner (mastery_routes).
### 12 · Incoming system relationships
- Scoped Scoreboards (server_leaderboards) can use this system because it provides the Discord server boundary and permission context for guild features.
- Discovery Scorecard (expedition_score) can use this system because it allows guilds to coordinate asynchronous contributions and party membership.
- Vault Blueprint Library (vault_blueprints) can use this system because it hosts shared roles and parallel clue paths for Guild Vaults.
- Friend-First Match Finder (multiplayer_matchmaking) can use this system because it allows guild members to coordinate long-running group activities.
### 13 · Reward and economy contract
- Reward policy: A shared map layer with revision history and contributor attribution.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the guild cartography room view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the guild_map rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the guild cartography room: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes exploration useful to the whole group while separating verified facts from speculation.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# guild_alliances — Guild Neighbourhoods
> Node 8 of 10 in **Guilds and Long-Form Cooperation** · World Loom identity `guild_alliances` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Two or more guilds can opt into a bounded cooperative event or shared puzzle without merging their private data.
- Why this belongs in Moonbyte: It supports cross-server community without building a global unrestricted social graph.
- Distinctive interaction: Invite another guild, review the event's public scope, accept the collaboration, complete shared tasks and close the link.
- Successful completion produces: A time-bounded collaboration contract with explicit visibility and separate ledgers.
### 02 · Purpose and boundaries
- Product role: Guilds coordinate shared adventures, persistent projects and optional social identity while individual ownership and server boundaries remain explicit.
- This system is the canonical owner of guild neighbourhoods behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Guilds and Long-Form Cooperation; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports cross-server community without building a global unrestricted social graph.
- Stable destination ID: guild_alliances; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .guild alliances and /guild alliances are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Invite another guild and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to guild neighbourhoods.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Two or more guilds can opt into a bounded cooperative event or shared puzzle without merging their private data.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Invite another guild, review the event's public scope, accept the collaboration, complete shared tasks and close the link.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Guild Neighbourhoods is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A time-bounded collaboration contract with explicit visibility and separate ledgers.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- binds guild identity and moderation to the correct Discord server: Server Steward Desk (server_moderation).
- forms parties for map-based expeditions and cooperative routes: Expedition Planner (expedition_planner).
- provides the membership and contribution model for Guild Vault challenges: Relic Galleries (vault_relic_rooms).
- turns verified participation and skill accomplishments into transparent guild recognition: Earned Cosmetic Unlocks (mastery_cosmetics).
### 12 · Incoming system relationships
- Server Steward Desk (server_moderation) can use this system because it provides the Discord server boundary and permission context for guild features.
- Expedition Route Archive (expedition_replay) can use this system because it allows guilds to coordinate asynchronous contributions and party membership.
- Puzzle Chambers (vault_chambers) can use this system because it hosts shared roles and parallel clue paths for Guild Vaults.
- Shared Turn Engine (multiplayer_turn_engine) can use this system because it allows guild members to coordinate long-running group activities.
### 13 · Reward and economy contract
- Reward policy: A time-bounded collaboration contract with explicit visibility and separate ledgers.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive guild neighbourhoods view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the guild_alliances rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for guild neighbourhoods: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports cross-server community without building a global unrestricted social graph.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# guild_reputation — Guild Trust Record
> Node 9 of 10 in **Guilds and Long-Form Cooperation** · World Loom identity `guild_reputation` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A transparent record tracks fulfilled commitments, moderation standing and cooperative milestones rather than traded power.
- Why this belongs in Moonbyte: It gives members context for collaboration without creating a pay-to-win ranking.
- Distinctive interaction: Inspect evidence-backed milestones, view the rules, submit a correction request and compare only like-for-like records.
- Successful completion produces: A public-safe reputation summary that cannot be purchased or transferred.
### 02 · Purpose and boundaries
- Product role: Guilds coordinate shared adventures, persistent projects and optional social identity while individual ownership and server boundaries remain explicit.
- This system is the canonical owner of guild trust record behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Guilds and Long-Form Cooperation; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives members context for collaboration without creating a pay-to-win ranking.
- Stable destination ID: guild_reputation; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .guild reputation and /guild reputation are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect evidence-backed milestones and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to guild trust record.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A transparent record tracks fulfilled commitments, moderation standing and cooperative milestones rather than traded power.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect evidence-backed milestones, view the rules, submit a correction request and compare only like-for-like records.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Guild Trust Record is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A public-safe reputation summary that cannot be purchased or transferred.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- binds guild identity and moderation to the correct Discord server: Server Data Boundary (server_privacy).
- forms parties for map-based expeditions and cooperative routes: Route Encounters (expedition_route_events).
- provides the membership and contribution model for Guild Vault challenges: Mastery Trial Rooms (vault_mastery_trials).
- turns verified participation and skill accomplishments into transparent guild recognition: Skill Badge Portfolio (mastery_badges).
### 12 · Incoming system relationships
- Server Data Boundary (server_privacy) can use this system because it provides the Discord server boundary and permission context for guild features.
- Expedition Planner (expedition_planner) can use this system because it allows guilds to coordinate asynchronous contributions and party membership.
- Logic Key Mechanisms (vault_logic_keys) can use this system because it hosts shared roles and parallel clue paths for Guild Vaults.
- Simultaneous Choice Windows (multiplayer_simultaneous) can use this system because it allows guild members to coordinate long-running group activities.
### 13 · Reward and economy contract
- Reward policy: A public-safe reputation summary that cannot be purchased or transferred.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive guild trust record view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the guild_reputation rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for guild trust record: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives members context for collaboration without creating a pay-to-win ranking.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# guild_archive — The Guild Chronicle
> Node 10 of 10 in **Guilds and Long-Form Cooperation** · World Loom identity `guild_archive` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: The guild can preserve completed projects, expedition maps, event memories and authored recaps as a searchable shared archive.
- Why this belongs in Moonbyte: It makes long-form group work matter after the immediate reward is claimed.
- Distinctive interaction: Choose a completed record, approve its display scope, add an original recap and link its map or quest references.
- Successful completion produces: A durable guild chronicle entry with privacy-aware authorship.
### 02 · Purpose and boundaries
- Product role: Guilds coordinate shared adventures, persistent projects and optional social identity while individual ownership and server boundaries remain explicit.
- This system is the canonical owner of the guild chronicle behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Guilds and Long-Form Cooperation; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes long-form group work matter after the immediate reward is claimed.
- Stable destination ID: guild_archive; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .guild archive and /guild archive are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a completed record and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the guild chronicle.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: The guild can preserve completed projects, expedition maps, event memories and authored recaps as a searchable shared archive.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a completed record, approve its display scope, add an original recap and link its map or quest references.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Guild Chronicle is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A durable guild chronicle entry with privacy-aware authorship.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- binds guild identity and moderation to the correct Discord server: The Quiet Community Digest (server_digest).
- forms parties for map-based expeditions and cooperative routes: Expedition Roles (expedition_roles).
- provides the membership and contribution model for Guild Vault challenges: The Grand Mechanism (vault_boss_logic).
- turns verified participation and skill accomplishments into transparent guild recognition: Personal Best Registry (mastery_records).
### 12 · Incoming system relationships
- The Quiet Community Digest (server_digest) can use this system because it provides the Discord server boundary and permission context for guild features.
- Route Encounters (expedition_route_events) can use this system because it allows guilds to coordinate asynchronous contributions and party membership.
- Relic Galleries (vault_relic_rooms) can use this system because it hosts shared roles and parallel clue paths for Guild Vaults.
- Spectator Gallery (multiplayer_spectators) can use this system because it allows guild members to coordinate long-running group activities.
### 13 · Reward and economy contract
- Reward policy: A durable guild chronicle entry with privacy-aware authorship.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the guild chronicle view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the guild_archive rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the guild chronicle: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes long-form group work matter after the immediate reward is claimed.; it must remain satisfying in a bounded session and support a natural stopping point.
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

## Domain 4 — Server Culture and Community Layer

Every Discord guild can shape its own Moonbyte experience through scoped configuration, community content and moderation, while shared player identity remains separate.

**Domain dependencies:** Guilds and Long-Form Cooperation (`guild`) · Festivals, Chapters and World Events (`festival`) · Accessibility, Moderation, Privacy and Operations (`trust`) · Arcade and Original Mini-Games (`arcade`)

# server_identity — Server World Identity
> Node 1 of 10 in **Server Culture and Community Layer** · World Loom identity `server_identity` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A guild can choose its own nameplate, colour palette from approved art, welcome scene and enabled game categories.
- Why this belongs in Moonbyte: It makes different servers feel distinct without forking the global rules or item definitions.
- Distinctive interaction: Authorised admin opens server settings, previews a theme, chooses enabled content and publishes the configuration.
- Successful completion produces: A guild-scoped configuration version and presentation theme.
### 02 · Purpose and boundaries
- Product role: Every Discord guild can shape its own Moonbyte experience through scoped configuration, community content and moderation, while shared player identity remains separate.
- This system is the canonical owner of server world identity behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Server Culture and Community Layer; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes different servers feel distinct without forking the global rules or item definitions.
- Stable destination ID: server_identity; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .server identity and /server identity are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Authorised admin opens server settings and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to server world identity.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A guild can choose its own nameplate, colour palette from approved art, welcome scene and enabled game categories.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Authorised admin opens server settings, previews a theme, chooses enabled content and publishes the configuration.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Server World Identity is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A guild-scoped configuration version and presentation theme.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides the Discord server boundary and permission context for guild features: Guild Charters (guild_charters).
- turns server calendars into optional community events and celebrations: World Restoration Projects (festival_world_projects).
- uses moderation, privacy and rate-limit policies for server-owned features: Privacy and Data Center (trust_privacy_center).
- lets administrators choose which approved game categories appear in the server library: Curated Game Routes (arcade_playlists).
### 12 · Incoming system relationships
- Guild Charters (guild_charters) can use this system because it binds guild identity and moderation to the correct Discord server.
- Server Celebration Kits (festival_server_events) can use this system because it uses community calendars and guild preferences for event planning.
- Moderator Console (trust_moderator_console) can use this system because it enforces guild configuration, channel scope and moderator permissions.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A guild-scoped configuration version and presentation theme.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive server world identity view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the server_identity rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for server world identity: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes different servers feel distinct without forking the global rules or item definitions.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# server_calendar — Community Calendar
> Node 2 of 10 in **Server Culture and Community Layer** · World Loom identity `server_calendar` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A calendar lists opt-in sessions, guild projects, puzzle nights and authored festivals with clear time zones and RSVP controls.
- Why this belongs in Moonbyte: It coordinates real people without turning attendance into a reward obligation.
- Distinctive interaction: Browse the calendar, inspect duration and access, RSVP, receive only opted-in reminders and cancel freely.
- Successful completion produces: A scheduled activity, RSVP list and bounded notification plan.
### 02 · Purpose and boundaries
- Product role: Every Discord guild can shape its own Moonbyte experience through scoped configuration, community content and moderation, while shared player identity remains separate.
- This system is the canonical owner of community calendar behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Server Culture and Community Layer; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It coordinates real people without turning attendance into a reward obligation.
- Stable destination ID: server_calendar; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .server calendar and /server calendar are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Browse the calendar and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to community calendar.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A calendar lists opt-in sessions, guild projects, puzzle nights and authored festivals with clear time zones and RSVP controls.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Browse the calendar, inspect duration and access, RSVP, receive only opted-in reminders and cancel freely.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Community Calendar is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A scheduled activity, RSVP list and bounded notification plan.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides the Discord server boundary and permission context for guild features: The Guild Hall (guild_hall).
- turns server calendars into optional community events and celebrations: Puzzle Rotation Archive (festival_rotating_puzzles).
- uses moderation, privacy and rate-limit policies for server-owned features: Quiet Notification Rules (trust_notification_rules).
- lets administrators choose which approved game categories appear in the server library: The Pocket Arcade (arcade_library).
### 12 · Incoming system relationships
- The Guild Hall (guild_hall) can use this system because it binds guild identity and moderation to the correct Discord server.
- World Restoration Projects (festival_world_projects) can use this system because it uses community calendars and guild preferences for event planning.
- Report and Review Pipeline (trust_reports) can use this system because it enforces guild configuration, channel scope and moderator permissions.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A scheduled activity, RSVP list and bounded notification plan.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive community calendar view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the server_calendar rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for community calendar: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It coordinates real people without turning attendance into a reward obligation.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# server_channels — Channel Wayfinding
> Node 3 of 10 in **Server Culture and Community Layer** · World Loom identity `server_channels` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Admins can map activity types to appropriate channels and set whether boards are public, private or ephemeral.
- Why this belongs in Moonbyte: It keeps public games tidy and prevents personal information from spilling into public channels.
- Distinctive interaction: Choose a category, select permitted channels, preview message visibility and save a versioned route.
- Successful completion produces: A channel policy that the bot checks before publishing.
### 02 · Purpose and boundaries
- Product role: Every Discord guild can shape its own Moonbyte experience through scoped configuration, community content and moderation, while shared player identity remains separate.
- This system is the canonical owner of channel wayfinding behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Server Culture and Community Layer; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It keeps public games tidy and prevents personal information from spilling into public channels.
- Stable destination ID: server_channels; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .server channels and /server channels are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a category and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to channel wayfinding.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Admins can map activity types to appropriate channels and set whether boards are public, private or ephemeral.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a category, select permitted channels, preview message visibility and save a versioned route.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Channel Wayfinding is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A channel policy that the bot checks before publishing.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides the Discord server boundary and permission context for guild features: Guild Role Charter (guild_roles).
- turns server calendars into optional community events and celebrations: The Celebration Wall (festival_celebration_wall).
- uses moderation, privacy and rate-limit policies for server-owned features: Session Reliability Desk (trust_reliability).
- lets administrators choose which approved game categories appear in the server library: Optional Daily Logic Desk (arcade_daily_logic).
### 12 · Incoming system relationships
- Guild Role Charter (guild_roles) can use this system because it binds guild identity and moderation to the correct Discord server.
- Puzzle Rotation Archive (festival_rotating_puzzles) can use this system because it uses community calendars and guild preferences for event planning.
- Privacy and Data Center (trust_privacy_center) can use this system because it enforces guild configuration, channel scope and moderator permissions.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A channel policy that the bot checks before publishing.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive channel wayfinding view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the server_channels rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for channel wayfinding: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It keeps public games tidy and prevents personal information from spilling into public channels.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# server_challenges — Server Challenge Board
> Node 4 of 10 in **Server Culture and Community Layer** · World Loom identity `server_challenges` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A server can host optional shared challenges with one clear objective, one end condition and a published contribution rule.
- Why this belongs in Moonbyte: It creates community play without competitive pressure or punitive streaks.
- Distinctive interaction: Read the challenge, join explicitly, contribute a valid action, inspect shared progress and leave at any time.
- Successful completion produces: A guild-scoped challenge state and non-random, published completion result.
### 02 · Purpose and boundaries
- Product role: Every Discord guild can shape its own Moonbyte experience through scoped configuration, community content and moderation, while shared player identity remains separate.
- This system is the canonical owner of server challenge board behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Server Culture and Community Layer; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates community play without competitive pressure or punitive streaks.
- Stable destination ID: server_challenges; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .server challenges and /server challenges are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Read the challenge and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to server challenge board.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A server can host optional shared challenges with one clear objective, one end condition and a published contribution rule.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Read the challenge, join explicitly, contribute a valid action, inspect shared progress and leave at any time.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Server Challenge Board is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A guild-scoped challenge state and non-random, published completion result.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides the Discord server boundary and permission context for guild features: The Great Works Board (guild_projects).
- turns server calendars into optional community events and celebrations: Neighbourhood Festival Circuit (festival_cross_server).
- uses moderation, privacy and rate-limit policies for server-owned features: World Content Policy (trust_content_policy).
- lets administrators choose which approved game categories appear in the server library: The Practice School (arcade_skill_school).
### 12 · Incoming system relationships
- The Great Works Board (guild_projects) can use this system because it binds guild identity and moderation to the correct Discord server.
- The Celebration Wall (festival_celebration_wall) can use this system because it uses community calendars and guild preferences for event planning.
- Quiet Notification Rules (trust_notification_rules) can use this system because it enforces guild configuration, channel scope and moderator permissions.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A guild-scoped challenge state and non-random, published completion result.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive server challenge board view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the server_challenges rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for server challenge board: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates community play without competitive pressure or punitive streaks.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# server_museum — The Community Museum
> Node 5 of 10 in **Server Culture and Community Layer** · World Loom identity `server_museum` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A server may curate approved public artwork, puzzle solutions, map discoveries and event recaps into a browsable gallery.
- Why this belongs in Moonbyte: It celebrates community creation while keeping publication permission explicit.
- Distinctive interaction: Submit an entry, choose its visibility, pass review, place it in a gallery and allow the creator to request removal.
- Successful completion produces: A moderated gallery item with attribution and revision state.
### 02 · Purpose and boundaries
- Product role: Every Discord guild can shape its own Moonbyte experience through scoped configuration, community content and moderation, while shared player identity remains separate.
- This system is the canonical owner of the community museum behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Server Culture and Community Layer; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It celebrates community creation while keeping publication permission explicit.
- Stable destination ID: server_museum; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .server museum and /server museum are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Submit an entry and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the community museum.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A server may curate approved public artwork, puzzle solutions, map discoveries and event recaps into a browsable gallery.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Submit an entry, choose its visibility, pass review, place it in a gallery and allow the creator to request removal.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Community Museum is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A moderated gallery item with attribution and revision state.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides the Discord server boundary and permission context for guild features: Guild Story Contracts (guild_quests).
- turns server calendars into optional community events and celebrations: Open Archive Routes (festival_return_paths).
- uses moderation, privacy and rate-limit policies for server-owned features: Universal Control Profiles (trust_control_profiles).
- lets administrators choose which approved game categories appear in the server library: Puzzle Generator and Validator (arcade_puzzle_factory).
### 12 · Incoming system relationships
- Guild Story Contracts (guild_quests) can use this system because it binds guild identity and moderation to the correct Discord server.
- Neighbourhood Festival Circuit (festival_cross_server) can use this system because it uses community calendars and guild preferences for event planning.
- Session Reliability Desk (trust_reliability) can use this system because it enforces guild configuration, channel scope and moderator permissions.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A moderated gallery item with attribution and revision state.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the community museum view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the server_museum rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the community museum: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It celebrates community creation while keeping publication permission explicit.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# server_goals — Neighbourhood Goals
> Node 6 of 10 in **Server Culture and Community Layer** · World Loom identity `server_goals` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Members contribute to a server-wide build or discovery milestone that unlocks a permanent shared decoration or archive entry.
- Why this belongs in Moonbyte: It turns cooperation into a visible result instead of a temporary ranking race.
- Distinctive interaction: Inspect the goal, choose one contribution type, submit evidence, review milestones and celebrate completion.
- Successful completion produces: A permanent shared outcome with no decay if the server is quiet.
### 02 · Purpose and boundaries
- Product role: Every Discord guild can shape its own Moonbyte experience through scoped configuration, community content and moderation, while shared player identity remains separate.
- This system is the canonical owner of neighbourhood goals behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Server Culture and Community Layer; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It turns cooperation into a visible result instead of a temporary ranking race.
- Stable destination ID: server_goals; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .server goals and /server goals are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect the goal and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to neighbourhood goals.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Members contribute to a server-wide build or discovery milestone that unlocks a permanent shared decoration or archive entry.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect the goal, choose one contribution type, submit evidence, review milestones and celebrate completion.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Neighbourhood Goals is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A permanent shared outcome with no decay if the server is quiet.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides the Discord server boundary and permission context for guild features: The Guild Vault (guild_vault).
- turns server calendars into optional community events and celebrations: Festival Chronicle (festival_chronicles).
- uses moderation, privacy and rate-limit policies for server-owned features: Visual Comfort Settings (trust_contrast_motion).
- lets administrators choose which approved game categories appear in the server library: The Party Game Shelf (arcade_party_collection).
### 12 · Incoming system relationships
- The Guild Vault (guild_vault) can use this system because it binds guild identity and moderation to the correct Discord server.
- Open Archive Routes (festival_return_paths) can use this system because it uses community calendars and guild preferences for event planning.
- World Content Policy (trust_content_policy) can use this system because it enforces guild configuration, channel scope and moderator permissions.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A permanent shared outcome with no decay if the server is quiet.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive neighbourhood goals view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the server_goals rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for neighbourhood goals: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It turns cooperation into a visible result instead of a temporary ranking race.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# server_leaderboards — Scoped Scoreboards
> Node 7 of 10 in **Server Culture and Community Layer** · World Loom identity `server_leaderboards` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Administrators can enable opt-in leaderboards for supported games with clear windows, tie rules and privacy-safe display names.
- Why this belongs in Moonbyte: It supports friendly competition without turning every activity into a global status contest.
- Distinctive interaction: Choose a game and scope, read scoring rules, opt in, inspect records and hide personal display from the board.
- Successful completion produces: A guild-scoped, rules-versioned ranking snapshot.
### 02 · Purpose and boundaries
- Product role: Every Discord guild can shape its own Moonbyte experience through scoped configuration, community content and moderation, while shared player identity remains separate.
- This system is the canonical owner of scoped scoreboards behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Server Culture and Community Layer; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports friendly competition without turning every activity into a global status contest.
- Stable destination ID: server_leaderboards; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .server leaderboards and /server leaderboards are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a game and scope and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to scoped scoreboards.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Administrators can enable opt-in leaderboards for supported games with clear windows, tie rules and privacy-safe display names.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a game and scope, read scoring rules, opt in, inspect records and hide personal display from the board.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Scoped Scoreboards is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A guild-scoped, rules-versioned ranking snapshot.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides the Discord server boundary and permission context for guild features: The Guild Cartography Room (guild_map).
- turns server calendars into optional community events and celebrations: Moonbyte Anniversary (festival_anniversary).
- uses moderation, privacy and rate-limit policies for server-owned features: Language and Locale Registry (trust_localisation).
- lets administrators choose which approved game categories appear in the server library: Asynchronous Challenge Cards (arcade_async_challenges).
### 12 · Incoming system relationships
- The Guild Cartography Room (guild_map) can use this system because it binds guild identity and moderation to the correct Discord server.
- Festival Chronicle (festival_chronicles) can use this system because it uses community calendars and guild preferences for event planning.
- Universal Control Profiles (trust_control_profiles) can use this system because it enforces guild configuration, channel scope and moderator permissions.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A guild-scoped, rules-versioned ranking snapshot.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive scoped scoreboards view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the server_leaderboards rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for scoped scoreboards: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports friendly competition without turning every activity into a global status contest.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# server_moderation — Server Steward Desk
> Node 8 of 10 in **Server Culture and Community Layer** · World Loom identity `server_moderation` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Moderators receive tools for reports, content review, channel restrictions and event pauses without access to private player records.
- Why this belongs in Moonbyte: It keeps community safety independent from the game's economic and progression logic.
- Distinctive interaction: Open a report queue, inspect minimum necessary evidence, choose a bounded action and record a reason.
- Successful completion produces: An auditable moderation action with least-privilege access.
### 02 · Purpose and boundaries
- Product role: Every Discord guild can shape its own Moonbyte experience through scoped configuration, community content and moderation, while shared player identity remains separate.
- This system is the canonical owner of server steward desk behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Server Culture and Community Layer; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It keeps community safety independent from the game's economic and progression logic.
- Stable destination ID: server_moderation; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .server moderation and /server moderation are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open a report queue and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to server steward desk.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Moderators receive tools for reports, content review, channel restrictions and event pauses without access to private player records.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open a report queue, inspect minimum necessary evidence, choose a bounded action and record a reason.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Server Steward Desk is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: An auditable moderation action with least-privilege access.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides the Discord server boundary and permission context for guild features: Guild Neighbourhoods (guild_alliances).
- turns server calendars into optional community events and celebrations: World Fairs (festival_world_fairs).
- uses moderation, privacy and rate-limit policies for server-owned features: Text-First Play Mode (trust_text_fallback).
- lets administrators choose which approved game categories appear in the server library: Friendly Arcade League (arcade_league).
### 12 · Incoming system relationships
- Guild Neighbourhoods (guild_alliances) can use this system because it binds guild identity and moderation to the correct Discord server.
- Moonbyte Anniversary (festival_anniversary) can use this system because it uses community calendars and guild preferences for event planning.
- Visual Comfort Settings (trust_contrast_motion) can use this system because it enforces guild configuration, channel scope and moderator permissions.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: An auditable moderation action with least-privilege access.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive server steward desk view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the server_moderation rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for server steward desk: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It keeps community safety independent from the game's economic and progression logic.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# server_privacy — Server Data Boundary
> Node 9 of 10 in **Server Culture and Community Layer** · World Loom identity `server_privacy` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: The server layer documents which settings belong to a guild and which data remains player-owned across all servers.
- Why this belongs in Moonbyte: It prevents accidental cross-guild disclosure and makes privacy controls understandable.
- Distinctive interaction: Inspect data categories, review a guild setting, change a consent or visibility value and verify the scope.
- Successful completion produces: A data-boundary inventory and isolation test contract.
### 02 · Purpose and boundaries
- Product role: Every Discord guild can shape its own Moonbyte experience through scoped configuration, community content and moderation, while shared player identity remains separate.
- This system is the canonical owner of server data boundary behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Server Culture and Community Layer; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It prevents accidental cross-guild disclosure and makes privacy controls understandable.
- Stable destination ID: server_privacy; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .server privacy and /server privacy are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect data categories and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to server data boundary.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: The server layer documents which settings belong to a guild and which data remains player-owned across all servers.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect data categories, review a guild setting, change a consent or visibility value and verify the scope.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Server Data Boundary is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A data-boundary inventory and isolation test contract.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides the Discord server boundary and permission context for guild features: Guild Trust Record (guild_reputation).
- turns server calendars into optional community events and celebrations: Seasonal Story Chapters (festival_chapters).
- uses moderation, privacy and rate-limit policies for server-owned features: Moderator Console (trust_moderator_console).
- lets administrators choose which approved game categories appear in the server library: Replay and Review Desk (arcade_replays).
### 12 · Incoming system relationships
- Guild Trust Record (guild_reputation) can use this system because it binds guild identity and moderation to the correct Discord server.
- World Fairs (festival_world_fairs) can use this system because it uses community calendars and guild preferences for event planning.
- Language and Locale Registry (trust_localisation) can use this system because it enforces guild configuration, channel scope and moderator permissions.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A data-boundary inventory and isolation test contract.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive server data boundary view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the server_privacy rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for server data boundary: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It prevents accidental cross-guild disclosure and makes privacy controls understandable.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# server_digest — The Quiet Community Digest
> Node 10 of 10 in **Server Culture and Community Layer** · World Loom identity `server_digest` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: An opt-in digest summarises important completed projects and upcoming events at a chosen frequency without activity bait.
- Why this belongs in Moonbyte: It keeps members informed without ping floods, urgency language or daily pressure.
- Distinctive interaction: Choose digest topics, frequency and delivery channel, preview content and pause or opt out at any time.
- Successful completion produces: A small scheduled summary generated from public-safe events.
### 02 · Purpose and boundaries
- Product role: Every Discord guild can shape its own Moonbyte experience through scoped configuration, community content and moderation, while shared player identity remains separate.
- This system is the canonical owner of the quiet community digest behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Server Culture and Community Layer; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It keeps members informed without ping floods, urgency language or daily pressure.
- Stable destination ID: server_digest; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .server digest and /server digest are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose digest topics and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the quiet community digest.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: An opt-in digest summarises important completed projects and upcoming events at a chosen frequency without activity bait.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose digest topics, frequency and delivery channel, preview content and pause or opt out at any time.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Quiet Community Digest is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A small scheduled summary generated from public-safe events.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides the Discord server boundary and permission context for guild features: The Guild Chronicle (guild_archive).
- turns server calendars into optional community events and celebrations: Server Celebration Kits (festival_server_events).
- uses moderation, privacy and rate-limit policies for server-owned features: Report and Review Pipeline (trust_reports).
- lets administrators choose which approved game categories appear in the server library: Arcade Control Profiles (arcade_accessibility).
### 12 · Incoming system relationships
- The Guild Chronicle (guild_archive) can use this system because it binds guild identity and moderation to the correct Discord server.
- Seasonal Story Chapters (festival_chapters) can use this system because it uses community calendars and guild preferences for event planning.
- Text-First Play Mode (trust_text_fallback) can use this system because it enforces guild configuration, channel scope and moderator permissions.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A small scheduled summary generated from public-safe events.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the quiet community digest view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the server_digest rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the quiet community digest: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It keeps members informed without ping floods, urgency language or daily pressure.; it must remain satisfying in a bounded session and support a natural stopping point.
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
