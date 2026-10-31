# Volume IV — Characters, Mastery and Homesteads

> Moonbyte World Loom · 40 connected system proposals · This is a design specification, not a claim of implementation.

Each card is a distinct proposal with a stable graph identity, a player loop, state and ownership boundaries, Discord and Activity surfaces, privacy rules, acceptance tests and explicit cross-system links.

## Domain 13 — Characters, Companions and Tactical Identity

Characters provide optional strategic identity and story expression. Power is earned predictably, cosmetic choices stay cosmetic and companions are original.

**Domain dependencies:** Progression, Mastery and Recognition (`mastery`) · Friends, Community Bonds and Social Play (`fellowship`) · Gathering, Crafting and Workshops (`craft`) · Quests, Mystery and Story (`quest`)

# character_identity — Player Character Sheet
> Node 1 of 10 in **Characters, Companions and Tactical Identity** · World Loom identity `character_identity` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A character sheet collects a chosen name, visual identity, archetype, equipped options and a concise adventure summary.
- Why this belongs in Moonbyte: It provides a consistent role-playing identity without demanding extensive personal data.
- Distinctive interaction: Create a character, choose permitted options, preview the sheet and save a versioned profile.
- Successful completion produces: A player-owned character record separate from Discord account credentials.
### 02 · Purpose and boundaries
- Product role: Characters provide optional strategic identity and story expression. Power is earned predictably, cosmetic choices stay cosmetic and companions are original.
- This system is the canonical owner of player character sheet behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Characters, Companions and Tactical Identity; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It provides a consistent role-playing identity without demanding extensive personal data.
- Stable destination ID: character_identity; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .character identity and /character identity are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Create a character and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to player character sheet.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A character sheet collects a chosen name, visual identity, archetype, equipped options and a concise adventure summary.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Create a character, choose permitted options, preview the sheet and save a versioned profile.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Player Character Sheet is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A player-owned character record separate from Discord account credentials.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses transparent practice and achievements to unlock documented character options: Experience and Milestone Ledger (mastery_experience).
- gives parties complementary utility without requiring one optimal composition: Interest Clubs (fellowship_clubs).
- uses known recipes and item definitions for non-random equipment: Workshop Repair (craft_repair).
- connects character choices to authored story dialogue and branch flags: Story Remix Paths (quest_remix).
### 12 · Incoming system relationships
- Experience and Milestone Ledger (mastery_experience) can use this system because it provides predictable side-grade character and utility milestones.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A player-owned character record separate from Discord account credentials.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive player character sheet view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the character_identity rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for player character sheet: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It provides a consistent role-playing identity without demanding extensive personal data.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# character_archetypes — Archetype Codex
> Node 2 of 10 in **Characters, Companions and Tactical Identity** · World Loom identity `character_archetypes` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A small set of original archetypes defines different tactical approaches such as Navigator, Maker, Scholar and Guardian.
- Why this belongs in Moonbyte: It gives players meaningful starting choices without creating a strict class hierarchy.
- Distinctive interaction: Compare archetypes, inspect active and passive rules, choose one and test it in practice.
- Successful completion produces: A documented archetype with versioned rules and respec clarity.
### 02 · Purpose and boundaries
- Product role: Characters provide optional strategic identity and story expression. Power is earned predictably, cosmetic choices stay cosmetic and companions are original.
- This system is the canonical owner of archetype codex behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Characters, Companions and Tactical Identity; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives players meaningful starting choices without creating a strict class hierarchy.
- Stable destination ID: character_archetypes; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .character archetypes and /character archetypes are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Compare archetypes and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to archetype codex.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A small set of original archetypes defines different tactical approaches such as Navigator, Maker, Scholar and Guardian.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Compare archetypes, inspect active and passive rules, choose one and test it in practice.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Archetype Codex is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A documented archetype with versioned rules and respec clarity.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses transparent practice and achievements to unlock documented character options: Skill Mastery Tracks (mastery_tracks).
- gives parties complementary utility without requiring one optimal composition: Thoughtful Gift Exchange (fellowship_gifting).
- uses known recipes and item definitions for non-random equipment: Pattern and Set Assembly (craft_sets).
- connects character choices to authored story dialogue and branch flags: The Quest Board (quest_board).
### 12 · Incoming system relationships
- Skill Mastery Tracks (mastery_tracks) can use this system because it provides predictable side-grade character and utility milestones.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A documented archetype with versioned rules and respec clarity.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive archetype codex view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the character_archetypes rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for archetype codex: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives players meaningful starting choices without creating a strict class hierarchy.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# character_skill_paths — Skill Path Map
> Node 3 of 10 in **Characters, Companions and Tactical Identity** · World Loom identity `character_skill_paths` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A branching skill map offers side-grade abilities and utility unlocks with visible prerequisites and no paid shortcuts.
- Why this belongs in Moonbyte: It supports long-term mastery while avoiding a single mandatory build.
- Distinctive interaction: Inspect a path, review all branches, earn a milestone, choose an unlock and test it in a supported game.
- Successful completion produces: A character progression flag and a documented ability selection.
### 02 · Purpose and boundaries
- Product role: Characters provide optional strategic identity and story expression. Power is earned predictably, cosmetic choices stay cosmetic and companions are original.
- This system is the canonical owner of skill path map behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Characters, Companions and Tactical Identity; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports long-term mastery while avoiding a single mandatory build.
- Stable destination ID: character_skill_paths; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .character skill paths and /character skill paths are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect a path and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to skill path map.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A branching skill map offers side-grade abilities and utility unlocks with visible prerequisites and no paid shortcuts.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect a path, review all branches, earn a milestone, choose an unlock and test it in a supported game.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Skill Path Map is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A character progression flag and a documented ability selection.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses transparent practice and achievements to unlock documented character options: Achievement Registry (mastery_achievements).
- gives parties complementary utility without requiring one optimal composition: Community Message Board (fellowship_mailbox).
- uses known recipes and item definitions for non-random equipment: Guild Workshop (craft_shared_bench).
- connects character choices to authored story dialogue and branch flags: Chapter Arcs (quest_arcs).
### 12 · Incoming system relationships
- Achievement Registry (mastery_achievements) can use this system because it provides predictable side-grade character and utility milestones.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A character progression flag and a documented ability selection.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive skill path map view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the character_skill_paths rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for skill path map: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports long-term mastery while avoiding a single mandatory build.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# character_loadouts — Adventure Loadouts
> Node 4 of 10 in **Characters, Companions and Tactical Identity** · World Loom identity `character_loadouts` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can save several named combinations of role, equipment and utility choices for different game categories.
- Why this belongs in Moonbyte: It reduces repeated setup and makes preparation part of strategy.
- Distinctive interaction: Build a loadout, validate its compatibility, save, select before a session and revise between runs.
- Successful completion produces: A versioned preset with no hidden inventory transfer.
### 02 · Purpose and boundaries
- Product role: Characters provide optional strategic identity and story expression. Power is earned predictably, cosmetic choices stay cosmetic and companions are original.
- This system is the canonical owner of adventure loadouts behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Characters, Companions and Tactical Identity; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It reduces repeated setup and makes preparation part of strategy.
- Stable destination ID: character_loadouts; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .character loadouts and /character loadouts are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Build a loadout and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to adventure loadouts.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can save several named combinations of role, equipment and utility choices for different game categories.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Build a loadout, validate its compatibility, save, select before a session and revise between runs.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Adventure Loadouts is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned preset with no hidden inventory transfer.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses transparent practice and achievements to unlock documented character options: Player Honour Record (mastery_reputation).
- gives parties complementary utility without requiring one optimal composition: Positive Contribution Notes (fellowship_commendations).
- uses known recipes and item definitions for non-random equipment: Recipe Research (craft_discovery).
- connects character choices to authored story dialogue and branch flags: Choice and Consequence Graph (quest_branches).
### 12 · Incoming system relationships
- Player Honour Record (mastery_reputation) can use this system because it provides predictable side-grade character and utility milestones.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A versioned preset with no hidden inventory transfer.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive adventure loadouts view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the character_loadouts rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for adventure loadouts: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It reduces repeated setup and makes preparation part of strategy.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# character_companions — Companion Chronicle
> Node 5 of 10 in **Characters, Companions and Tactical Identity** · World Loom identity `character_companions` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Original non-player companions have authored abilities, history and optional support interactions rather than random gacha acquisition.
- Why this belongs in Moonbyte: It creates attachment through story and utility without paid pulls or scarcity pressure.
- Distinctive interaction: Meet through a known story path, review companion rules, choose a support role and save their current assignment.
- Successful completion produces: An account-bound companion record and known unlock source.
### 02 · Purpose and boundaries
- Product role: Characters provide optional strategic identity and story expression. Power is earned predictably, cosmetic choices stay cosmetic and companions are original.
- This system is the canonical owner of companion chronicle behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Characters, Companions and Tactical Identity; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates attachment through story and utility without paid pulls or scarcity pressure.
- Stable destination ID: character_companions; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .character companions and /character companions are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Meet through a known story path and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to companion chronicle.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Original non-player companions have authored abilities, history and optional support interactions rather than random gacha acquisition.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Meet through a known story path, review companion rules, choose a support role and save their current assignment.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Companion Chronicle is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: An account-bound companion record and known unlock source.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses transparent practice and achievements to unlock documented character options: Personal Milestone Contracts (mastery_contracts).
- gives parties complementary utility without requiring one optimal composition: Shared Session Memories (fellowship_memories).
- uses known recipes and item definitions for non-random equipment: Field Gathering (craft_gathering).
- connects character choices to authored story dialogue and branch flags: The Investigation Desk (quest_mysteries).
### 12 · Incoming system relationships
- Personal Milestone Contracts (mastery_contracts) can use this system because it provides predictable side-grade character and utility milestones.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: An account-bound companion record and known unlock source.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive companion chronicle view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the character_companions rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for companion chronicle: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates attachment through story and utility without paid pulls or scarcity pressure.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# character_party_synergy — Role Synergy Board
> Node 6 of 10 in **Characters, Companions and Tactical Identity** · World Loom identity `character_party_synergy` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: The board explains useful combinations of party roles and provides alternative solutions so one role is never mandatory.
- Why this belongs in Moonbyte: It rewards communication and varied approaches rather than enforcing a single build.
- Distinctive interaction: Inspect a challenge, compare role tools, select a combination and resolve the group objective.
- Successful completion produces: A synergy hint set and party composition snapshot.
### 02 · Purpose and boundaries
- Product role: Characters provide optional strategic identity and story expression. Power is earned predictably, cosmetic choices stay cosmetic and companions are original.
- This system is the canonical owner of role synergy board behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Characters, Companions and Tactical Identity; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It rewards communication and varied approaches rather than enforcing a single build.
- Stable destination ID: character_party_synergy; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .character party synergy and /character party synergy are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect a challenge and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to role synergy board.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: The board explains useful combinations of party roles and provides alternative solutions so one role is never mandatory.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect a challenge, compare role tools, select a combination and resolve the group objective.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Role Synergy Board is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A synergy hint set and party composition snapshot.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses transparent practice and achievements to unlock documented character options: Milestone Route Planner (mastery_routes).
- gives parties complementary utility without requiring one optimal composition: Friend Discovery Routes (fellowship_discovery).
- uses known recipes and item definitions for non-random equipment: The Recipe Codex (craft_recipe_book).
- connects character choices to authored story dialogue and branch flags: Dialogue Routes (quest_dialogue).
### 12 · Incoming system relationships
- Milestone Route Planner (mastery_routes) can use this system because it provides predictable side-grade character and utility milestones.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A synergy hint set and party composition snapshot.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive role synergy board view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the character_party_synergy rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for role synergy board: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It rewards communication and varied approaches rather than enforcing a single build.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# character_equipment — Equipment Ledger
> Node 7 of 10 in **Characters, Companions and Tactical Identity** · World Loom identity `character_equipment` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Equipment gives transparent utility effects, provenance and compatibility rules, with no random stat rolls.
- Why this belongs in Moonbyte: It connects crafting and strategy without turning the Bazaar into a power arms race.
- Distinctive interaction: Inspect an item, compare exact effects, equip before a run and track its source.
- Successful completion produces: A character equipment state backed by item definitions.
### 02 · Purpose and boundaries
- Product role: Characters provide optional strategic identity and story expression. Power is earned predictably, cosmetic choices stay cosmetic and companions are original.
- This system is the canonical owner of equipment ledger behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Characters, Companions and Tactical Identity; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It connects crafting and strategy without turning the Bazaar into a power arms race.
- Stable destination ID: character_equipment; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .character equipment and /character equipment are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect an item and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to equipment ledger.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Equipment gives transparent utility effects, provenance and compatibility rules, with no random stat rolls.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect an item, compare exact effects, equip before a run and track its source.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Equipment Ledger is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A character equipment state backed by item definitions.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses transparent practice and achievements to unlock documented character options: Earned Cosmetic Unlocks (mastery_cosmetics).
- gives parties complementary utility without requiring one optimal composition: Social Boundaries Panel (fellowship_safety).
- uses known recipes and item definitions for non-random equipment: The Maker's Workshop (craft_workshop).
- connects character choices to authored story dialogue and branch flags: Faction Storylines (quest_factions).
### 12 · Incoming system relationships
- Earned Cosmetic Unlocks (mastery_cosmetics) can use this system because it provides predictable side-grade character and utility milestones.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A character equipment state backed by item definitions.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive equipment ledger view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the character_equipment rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for equipment ledger: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It connects crafting and strategy without turning the Bazaar into a power arms race.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# character_titles — Titles and Identity Marks
> Node 8 of 10 in **Characters, Companions and Tactical Identity** · World Loom identity `character_titles` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Titles display authored achievements, roles and creative contributions with player-controlled visibility.
- Why this belongs in Moonbyte: It lets players express identity without comparing raw playtime or wealth.
- Distinctive interaction: Browse eligible titles, inspect their source, equip one and hide it whenever desired.
- Successful completion produces: An earned title flag and profile presentation setting.
### 02 · Purpose and boundaries
- Product role: Characters provide optional strategic identity and story expression. Power is earned predictably, cosmetic choices stay cosmetic and companions are original.
- This system is the canonical owner of titles and identity marks behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Characters, Companions and Tactical Identity; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It lets players express identity without comparing raw playtime or wealth.
- Stable destination ID: character_titles; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .character titles and /character titles are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Browse eligible titles and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to titles and identity marks.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Titles display authored achievements, roles and creative contributions with player-controlled visibility.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Browse eligible titles, inspect their source, equip one and hide it whenever desired.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Titles and Identity Marks is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: An earned title flag and profile presentation setting.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses transparent practice and achievements to unlock documented character options: Skill Badge Portfolio (mastery_badges).
- gives parties complementary utility without requiring one optimal composition: Friend Circle (fellowship_friends).
- uses known recipes and item definitions for non-random equipment: Known Quality Grades (craft_quality).
- connects character choices to authored story dialogue and branch flags: Local Tales (quest_side_stories).
### 12 · Incoming system relationships
- Skill Badge Portfolio (mastery_badges) can use this system because it provides predictable side-grade character and utility milestones.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: An earned title flag and profile presentation setting.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive titles and identity marks view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the character_titles rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for titles and identity marks: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It lets players express identity without comparing raw playtime or wealth.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# character_appearance — Pixel Persona Studio
> Node 9 of 10 in **Characters, Companions and Tactical Identity** · World Loom identity `character_appearance` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A bounded creator combines original approved palettes, outfit parts and accessories into a personal pixel character.
- Why this belongs in Moonbyte: It gives the world a visual identity and makes cosmetic creation accessible.
- Distinctive interaction: Choose parts, preview scale and contrast, save a draft and publish only the final selected appearance.
- Successful completion produces: A validated appearance recipe and reusable avatar render input.
### 02 · Purpose and boundaries
- Product role: Characters provide optional strategic identity and story expression. Power is earned predictably, cosmetic choices stay cosmetic and companions are original.
- This system is the canonical owner of pixel persona studio behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Characters, Companions and Tactical Identity; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives the world a visual identity and makes cosmetic creation accessible.
- Stable destination ID: character_appearance; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .character appearance and /character appearance are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose parts and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to pixel persona studio.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A bounded creator combines original approved palettes, outfit parts and accessories into a personal pixel character.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose parts, preview scale and contrast, save a draft and publish only the final selected appearance.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Pixel Persona Studio is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A validated appearance recipe and reusable avatar render input.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses transparent practice and achievements to unlock documented character options: Personal Best Registry (mastery_records).
- gives parties complementary utility without requiring one optimal composition: Cooperative Role Planner (fellowship_roles).
- uses known recipes and item definitions for non-random equipment: Commission Bench (craft_commissions).
- connects character choices to authored story dialogue and branch flags: Shared Story Decisions (quest_coop_story).
### 12 · Incoming system relationships
- Personal Best Registry (mastery_records) can use this system because it provides predictable side-grade character and utility milestones.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A validated appearance recipe and reusable avatar render input.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive pixel persona studio view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the character_appearance rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for pixel persona studio: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives the world a visual identity and makes cosmetic creation accessible.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# character_utility — Non-Combat Talents
> Node 10 of 10 in **Characters, Companions and Tactical Identity** · World Loom identity `character_utility` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Utility talents support navigation, clue interpretation, crafting and social coordination rather than increasing damage or competitive power.
- Why this belongs in Moonbyte: It gives different characters useful identities in non-combat play.
- Distinctive interaction: Choose a talent, inspect its supported activities, use it in a valid context and review its effect.
- Successful completion produces: A character ability event and a transparent rules effect.
### 02 · Purpose and boundaries
- Product role: Characters provide optional strategic identity and story expression. Power is earned predictably, cosmetic choices stay cosmetic and companions are original.
- This system is the canonical owner of non-combat talents behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Characters, Companions and Tactical Identity; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives different characters useful identities in non-combat play.
- Stable destination ID: character_utility; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .character utility and /character utility are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a talent and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to non-combat talents.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Utility talents support navigation, clue interpretation, crafting and social coordination rather than increasing damage or competitive power.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a talent, inspect its supported activities, use it in a valid context and review its effect.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Non-Combat Talents is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A character ability event and a transparent rules effect.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses transparent practice and achievements to unlock documented character options: Account-Bound Recognition (mastery_account_rewards).
- gives parties complementary utility without requiring one optimal composition: Opt-In Mentor Sessions (fellowship_mentor).
- uses known recipes and item definitions for non-random equipment: Material Reclaim (craft_recycling).
- connects character choices to authored story dialogue and branch flags: The Chapter Chronicle (quest_chronicle).
### 12 · Incoming system relationships
- Account-Bound Recognition (mastery_account_rewards) can use this system because it provides predictable side-grade character and utility milestones.
- The Atlas can reference this node through a stable location or content ID.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A character ability event and a transparent rules effect.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive non-combat talents view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the character_utility rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for non-combat talents: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives different characters useful identities in non-combat play.; it must remain satisfying in a bounded session and support a natural stopping point.
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

## Domain 14 — Progression, Mastery and Recognition

Progression recognises real accomplishments using clear thresholds. It never depends on daily attendance, spending or hidden random reward rolls.

**Domain dependencies:** Arcade and Original Mini-Games (`arcade`) · Characters, Companions and Tactical Identity (`character`) · Collections, Bestiary and Museum (`archive`) · Guilds and Long-Form Cooperation (`guild`)

# mastery_experience — Experience and Milestone Ledger
> Node 1 of 10 in **Progression, Mastery and Recognition** · World Loom identity `mastery_experience` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Experience is awarded by documented game actions with reasonable caps and no incentive to remain online indefinitely.
- Why this belongs in Moonbyte: It provides a common sense of progress across mini-games and longer adventures.
- Distinctive interaction: Complete an eligible activity, preview its award, receive one idempotent event and inspect the progress ledger.
- Successful completion produces: A player experience total derived from immutable award records.
### 02 · Purpose and boundaries
- Product role: Progression recognises real accomplishments using clear thresholds. It never depends on daily attendance, spending or hidden random reward rolls.
- This system is the canonical owner of experience and milestone ledger behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Progression, Mastery and Recognition; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It provides a common sense of progress across mini-games and longer adventures.
- Stable destination ID: mastery_experience; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .mastery experience and /mastery experience are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Complete an eligible activity and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to experience and milestone ledger.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Experience is awarded by documented game actions with reasonable caps and no incentive to remain online indefinitely.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Complete an eligible activity, preview its award, receive one idempotent event and inspect the progress ledger.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Experience and Milestone Ledger is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A player experience total derived from immutable award records.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses game results and practice lessons as skill evidence: The Pocket Arcade (arcade_library).
- provides predictable side-grade character and utility milestones: Adventure Loadouts (character_loadouts).
- stores achievements, personal records and earned recognition: The Cosmetic Gallery (archive_cosmetics).
- supports optional guild recognition for contribution and cooperation: The Guild Chronicle (guild_archive).
### 12 · Incoming system relationships
- Guild Charters (guild_charters) can use this system because it turns verified participation and skill accomplishments into transparent guild recognition.
- Choice and Consequence Graph (quest_branches) can use this system because it rewards completed skill and understanding with transparent records.
- The Party Game Shelf (arcade_party_collection) can use this system because it creates personal records, practice paths and transparent skill achievements.
- Equipment Ledger (character_equipment) can use this system because it uses transparent practice and achievements to unlock documented character options.
### 13 · Reward and economy contract
- Reward policy: A player experience total derived from immutable award records.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive experience and milestone ledger view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the mastery_experience rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for experience and milestone ledger: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It provides a common sense of progress across mini-games and longer adventures.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# mastery_tracks — Skill Mastery Tracks
> Node 2 of 10 in **Progression, Mastery and Recognition** · World Loom identity `mastery_tracks` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Each major game category has a mastery track that recognises learning, varied play and improved decisions.
- Why this belongs in Moonbyte: It lets players specialise and see progress without one global level dominating every activity.
- Distinctive interaction: Choose a track, inspect milestones, play a supported activity and claim its known recognition.
- Successful completion produces: A category-specific mastery state and achievement references.
### 02 · Purpose and boundaries
- Product role: Progression recognises real accomplishments using clear thresholds. It never depends on daily attendance, spending or hidden random reward rolls.
- This system is the canonical owner of skill mastery tracks behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Progression, Mastery and Recognition; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It lets players specialise and see progress without one global level dominating every activity.
- Stable destination ID: mastery_tracks; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .mastery tracks and /mastery tracks are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a track and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to skill mastery tracks.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Each major game category has a mastery track that recognises learning, varied play and improved decisions.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a track, inspect milestones, play a supported activity and claim its known recognition.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Skill Mastery Tracks is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A category-specific mastery state and achievement references.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses game results and practice lessons as skill evidence: Optional Daily Logic Desk (arcade_daily_logic).
- provides predictable side-grade character and utility milestones: Companion Chronicle (character_companions).
- stores achievements, personal records and earned recognition: The Living Lore Index (archive_lore).
- supports optional guild recognition for contribution and cooperation: Guild Charters (guild_charters).
### 12 · Incoming system relationships
- The Guild Hall (guild_hall) can use this system because it turns verified participation and skill accomplishments into transparent guild recognition.
- The Investigation Desk (quest_mysteries) can use this system because it rewards completed skill and understanding with transparent records.
- Asynchronous Challenge Cards (arcade_async_challenges) can use this system because it creates personal records, practice paths and transparent skill achievements.
- Titles and Identity Marks (character_titles) can use this system because it uses transparent practice and achievements to unlock documented character options.
### 13 · Reward and economy contract
- Reward policy: A category-specific mastery state and achievement references.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive skill mastery tracks view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the mastery_tracks rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for skill mastery tracks: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It lets players specialise and see progress without one global level dominating every activity.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# mastery_achievements — Achievement Registry
> Node 3 of 10 in **Progression, Mastery and Recognition** · World Loom identity `mastery_achievements` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Achievements have explicit triggers, versioned criteria, evidence sources and clear display settings.
- Why this belongs in Moonbyte: It turns accomplishments into trustworthy records that can be tested and audited.
- Distinctive interaction: Inspect an achievement, complete its stated condition, verify evidence and receive the idempotent unlock.
- Successful completion produces: A stable achievement record with source event and rules version.
### 02 · Purpose and boundaries
- Product role: Progression recognises real accomplishments using clear thresholds. It never depends on daily attendance, spending or hidden random reward rolls.
- This system is the canonical owner of achievement registry behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Progression, Mastery and Recognition; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It turns accomplishments into trustworthy records that can be tested and audited.
- Stable destination ID: mastery_achievements; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .mastery achievements and /mastery achievements are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect an achievement and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to achievement registry.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Achievements have explicit triggers, versioned criteria, evidence sources and clear display settings.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect an achievement, complete its stated condition, verify evidence and receive the idempotent unlock.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Achievement Registry is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A stable achievement record with source event and rules version.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses game results and practice lessons as skill evidence: The Practice School (arcade_skill_school).
- provides predictable side-grade character and utility milestones: Role Synergy Board (character_party_synergy).
- stores achievements, personal records and earned recognition: Collection Milestones (archive_milestones).
- supports optional guild recognition for contribution and cooperation: The Guild Hall (guild_hall).
### 12 · Incoming system relationships
- Guild Role Charter (guild_roles) can use this system because it turns verified participation and skill accomplishments into transparent guild recognition.
- Dialogue Routes (quest_dialogue) can use this system because it rewards completed skill and understanding with transparent records.
- Friendly Arcade League (arcade_league) can use this system because it creates personal records, practice paths and transparent skill achievements.
- Pixel Persona Studio (character_appearance) can use this system because it uses transparent practice and achievements to unlock documented character options.
### 13 · Reward and economy contract
- Reward policy: A stable achievement record with source event and rules version.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive achievement registry view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the mastery_achievements rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for achievement registry: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It turns accomplishments into trustworthy records that can be tested and audited.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# mastery_reputation — Player Honour Record
> Node 4 of 10 in **Progression, Mastery and Recognition** · World Loom identity `mastery_reputation` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A privacy-safe record recognises helpful play, fulfilled agreements and positive community contributions with appeal paths.
- Why this belongs in Moonbyte: It gives social trust an evidence basis without turning popularity into power.
- Distinctive interaction: Receive an eligible commendation, review its policy, inspect aggregate evidence and request correction if wrong.
- Successful completion produces: A non-transferable reputation summary with moderation safeguards.
### 02 · Purpose and boundaries
- Product role: Progression recognises real accomplishments using clear thresholds. It never depends on daily attendance, spending or hidden random reward rolls.
- This system is the canonical owner of player honour record behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Progression, Mastery and Recognition; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives social trust an evidence basis without turning popularity into power.
- Stable destination ID: mastery_reputation; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .mastery reputation and /mastery reputation are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Receive an eligible commendation and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to player honour record.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A privacy-safe record recognises helpful play, fulfilled agreements and positive community contributions with appeal paths.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Receive an eligible commendation, review its policy, inspect aggregate evidence and request correction if wrong.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Player Honour Record is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A non-transferable reputation summary with moderation safeguards.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses game results and practice lessons as skill evidence: Puzzle Generator and Validator (arcade_puzzle_factory).
- provides predictable side-grade character and utility milestones: Equipment Ledger (character_equipment).
- stores achievements, personal records and earned recognition: Collector's Provenance View (archive_provenance).
- supports optional guild recognition for contribution and cooperation: Guild Role Charter (guild_roles).
### 12 · Incoming system relationships
- The Great Works Board (guild_projects) can use this system because it turns verified participation and skill accomplishments into transparent guild recognition.
- Faction Storylines (quest_factions) can use this system because it rewards completed skill and understanding with transparent records.
- Replay and Review Desk (arcade_replays) can use this system because it creates personal records, practice paths and transparent skill achievements.
- Non-Combat Talents (character_utility) can use this system because it uses transparent practice and achievements to unlock documented character options.
### 13 · Reward and economy contract
- Reward policy: A non-transferable reputation summary with moderation safeguards.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive player honour record view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the mastery_reputation rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for player honour record: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives social trust an evidence basis without turning popularity into power.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# mastery_contracts — Personal Milestone Contracts
> Node 5 of 10 in **Progression, Mastery and Recognition** · World Loom identity `mastery_contracts` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can choose a bounded mastery objective such as learning a new puzzle type or completing a co-op role.
- Why this belongs in Moonbyte: It gives self-directed goals without punishing players for abandoning them.
- Distinctive interaction: Browse optional contracts, choose one, inspect completion conditions, make progress and stop or swap freely.
- Successful completion produces: A player-selected contract with no expiry penalty.
### 02 · Purpose and boundaries
- Product role: Progression recognises real accomplishments using clear thresholds. It never depends on daily attendance, spending or hidden random reward rolls.
- This system is the canonical owner of personal milestone contracts behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Progression, Mastery and Recognition; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives self-directed goals without punishing players for abandoning them.
- Stable destination ID: mastery_contracts; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .mastery contracts and /mastery contracts are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Browse optional contracts and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to personal milestone contracts.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can choose a bounded mastery objective such as learning a new puzzle type or completing a co-op role.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Browse optional contracts, choose one, inspect completion conditions, make progress and stop or swap freely.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Personal Milestone Contracts is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A player-selected contract with no expiry penalty.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses game results and practice lessons as skill evidence: The Party Game Shelf (arcade_party_collection).
- provides predictable side-grade character and utility milestones: Titles and Identity Marks (character_titles).
- stores achievements, personal records and earned recognition: The Grand Collection Index (archive_catalogue).
- supports optional guild recognition for contribution and cooperation: The Great Works Board (guild_projects).
### 12 · Incoming system relationships
- Guild Story Contracts (guild_quests) can use this system because it turns verified participation and skill accomplishments into transparent guild recognition.
- Local Tales (quest_side_stories) can use this system because it rewards completed skill and understanding with transparent records.
- Arcade Control Profiles (arcade_accessibility) can use this system because it creates personal records, practice paths and transparent skill achievements.
- Player Character Sheet (character_identity) can use this system because it uses transparent practice and achievements to unlock documented character options.
### 13 · Reward and economy contract
- Reward policy: A player-selected contract with no expiry penalty.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive personal milestone contracts view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the mastery_contracts rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for personal milestone contracts: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives self-directed goals without punishing players for abandoning them.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# mastery_routes — Milestone Route Planner
> Node 6 of 10 in **Progression, Mastery and Recognition** · World Loom identity `mastery_routes` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A planner shows several valid ways to progress toward a selected achievement or skill unlock.
- Why this belongs in Moonbyte: It turns long-term goals into clear choices and avoids undisclosed grind requirements.
- Distinctive interaction: Pick a milestone, compare its valid routes, pin one route and complete any eligible path.
- Successful completion produces: A saved plan and transparent progress estimate.
### 02 · Purpose and boundaries
- Product role: Progression recognises real accomplishments using clear thresholds. It never depends on daily attendance, spending or hidden random reward rolls.
- This system is the canonical owner of milestone route planner behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Progression, Mastery and Recognition; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It turns long-term goals into clear choices and avoids undisclosed grind requirements.
- Stable destination ID: mastery_routes; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .mastery routes and /mastery routes are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Pick a milestone and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to milestone route planner.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A planner shows several valid ways to progress toward a selected achievement or skill unlock.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Pick a milestone, compare its valid routes, pin one route and complete any eligible path.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Milestone Route Planner is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A saved plan and transparent progress estimate.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses game results and practice lessons as skill evidence: Asynchronous Challenge Cards (arcade_async_challenges).
- provides predictable side-grade character and utility milestones: Pixel Persona Studio (character_appearance).
- stores achievements, personal records and earned recognition: The Field Bestiary (archive_bestiary).
- supports optional guild recognition for contribution and cooperation: Guild Story Contracts (guild_quests).
### 12 · Incoming system relationships
- The Guild Vault (guild_vault) can use this system because it turns verified participation and skill accomplishments into transparent guild recognition.
- Shared Story Decisions (quest_coop_story) can use this system because it rewards completed skill and understanding with transparent records.
- Curated Game Routes (arcade_playlists) can use this system because it creates personal records, practice paths and transparent skill achievements.
- Archetype Codex (character_archetypes) can use this system because it uses transparent practice and achievements to unlock documented character options.
### 13 · Reward and economy contract
- Reward policy: A saved plan and transparent progress estimate.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive milestone route planner view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the mastery_routes rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for milestone route planner: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It turns long-term goals into clear choices and avoids undisclosed grind requirements.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# mastery_cosmetics — Earned Cosmetic Unlocks
> Node 7 of 10 in **Progression, Mastery and Recognition** · World Loom identity `mastery_cosmetics` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Cosmetics unlock through authored achievements, collections, community works and predictable milestones.
- Why this belongs in Moonbyte: It makes visible rewards meaningful without creating gameplay advantages.
- Distinctive interaction: Inspect a cosmetic source, complete its condition, claim the unlock and equip or hide it.
- Successful completion produces: An account-bound cosmetic entitlement with provenance.
### 02 · Purpose and boundaries
- Product role: Progression recognises real accomplishments using clear thresholds. It never depends on daily attendance, spending or hidden random reward rolls.
- This system is the canonical owner of earned cosmetic unlocks behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Progression, Mastery and Recognition; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes visible rewards meaningful without creating gameplay advantages.
- Stable destination ID: mastery_cosmetics; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .mastery cosmetics and /mastery cosmetics are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect a cosmetic source and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to earned cosmetic unlocks.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Cosmetics unlock through authored achievements, collections, community works and predictable milestones.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect a cosmetic source, complete its condition, claim the unlock and equip or hide it.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Earned Cosmetic Unlocks is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: An account-bound cosmetic entitlement with provenance.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses game results and practice lessons as skill evidence: Friendly Arcade League (arcade_league).
- provides predictable side-grade character and utility milestones: Non-Combat Talents (character_utility).
- stores achievements, personal records and earned recognition: The Relic Archive (archive_relics).
- supports optional guild recognition for contribution and cooperation: The Guild Vault (guild_vault).
### 12 · Incoming system relationships
- The Guild Cartography Room (guild_map) can use this system because it turns verified participation and skill accomplishments into transparent guild recognition.
- The Chapter Chronicle (quest_chronicle) can use this system because it rewards completed skill and understanding with transparent records.
- The Pocket Arcade (arcade_library) can use this system because it creates personal records, practice paths and transparent skill achievements.
- Skill Path Map (character_skill_paths) can use this system because it uses transparent practice and achievements to unlock documented character options.
### 13 · Reward and economy contract
- Reward policy: An account-bound cosmetic entitlement with provenance.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive earned cosmetic unlocks view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the mastery_cosmetics rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for earned cosmetic unlocks: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes visible rewards meaningful without creating gameplay advantages.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# mastery_badges — Skill Badge Portfolio
> Node 8 of 10 in **Progression, Mastery and Recognition** · World Loom identity `mastery_badges` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Badges summarise demonstrated ability in a specific rules version and are never traded or purchased.
- Why this belongs in Moonbyte: It gives players a portable record of mastery without claiming a broad measure of personal worth.
- Distinctive interaction: Complete a verified assessment, inspect its scope, display the badge if desired and update after later versions.
- Successful completion produces: A scoped skill badge with testable evidence.
### 02 · Purpose and boundaries
- Product role: Progression recognises real accomplishments using clear thresholds. It never depends on daily attendance, spending or hidden random reward rolls.
- This system is the canonical owner of skill badge portfolio behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Progression, Mastery and Recognition; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives players a portable record of mastery without claiming a broad measure of personal worth.
- Stable destination ID: mastery_badges; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .mastery badges and /mastery badges are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Complete a verified assessment and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to skill badge portfolio.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Badges summarise demonstrated ability in a specific rules version and are never traded or purchased.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Complete a verified assessment, inspect its scope, display the badge if desired and update after later versions.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Skill Badge Portfolio is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A scoped skill badge with testable evidence.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses game results and practice lessons as skill evidence: Replay and Review Desk (arcade_replays).
- provides predictable side-grade character and utility milestones: Player Character Sheet (character_identity).
- stores achievements, personal records and earned recognition: The Illustrated Compendium (archive_cards).
- supports optional guild recognition for contribution and cooperation: The Guild Cartography Room (guild_map).
### 12 · Incoming system relationships
- Guild Neighbourhoods (guild_alliances) can use this system because it turns verified participation and skill accomplishments into transparent guild recognition.
- Story Remix Paths (quest_remix) can use this system because it rewards completed skill and understanding with transparent records.
- Optional Daily Logic Desk (arcade_daily_logic) can use this system because it creates personal records, practice paths and transparent skill achievements.
- Adventure Loadouts (character_loadouts) can use this system because it uses transparent practice and achievements to unlock documented character options.
### 13 · Reward and economy contract
- Reward policy: A scoped skill badge with testable evidence.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive skill badge portfolio view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the mastery_badges rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for skill badge portfolio: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives players a portable record of mastery without claiming a broad measure of personal worth.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# mastery_records — Personal Best Registry
> Node 9 of 10 in **Progression, Mastery and Recognition** · World Loom identity `mastery_records` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Personal records store the best supported metric for each game alongside the rules version and difficulty.
- Why this belongs in Moonbyte: It supports improvement without comparing players who used incompatible conditions.
- Distinctive interaction: Finish a round, inspect the metric, compare like-for-like attempts and save a new best when earned.
- Successful completion produces: A personal record with immutable result reference.
### 02 · Purpose and boundaries
- Product role: Progression recognises real accomplishments using clear thresholds. It never depends on daily attendance, spending or hidden random reward rolls.
- This system is the canonical owner of personal best registry behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Progression, Mastery and Recognition; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports improvement without comparing players who used incompatible conditions.
- Stable destination ID: mastery_records; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .mastery records and /mastery records are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Finish a round and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to personal best registry.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Personal records store the best supported metric for each game alongside the rules version and difficulty.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Finish a round, inspect the metric, compare like-for-like attempts and save a new best when earned.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Personal Best Registry is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A personal record with immutable result reference.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses game results and practice lessons as skill evidence: Arcade Control Profiles (arcade_accessibility).
- provides predictable side-grade character and utility milestones: Archetype Codex (character_archetypes).
- stores achievements, personal records and earned recognition: The Museum Hall (archive_museum).
- supports optional guild recognition for contribution and cooperation: Guild Neighbourhoods (guild_alliances).
### 12 · Incoming system relationships
- Guild Trust Record (guild_reputation) can use this system because it turns verified participation and skill accomplishments into transparent guild recognition.
- The Quest Board (quest_board) can use this system because it rewards completed skill and understanding with transparent records.
- The Practice School (arcade_skill_school) can use this system because it creates personal records, practice paths and transparent skill achievements.
- Companion Chronicle (character_companions) can use this system because it uses transparent practice and achievements to unlock documented character options.
### 13 · Reward and economy contract
- Reward policy: A personal record with immutable result reference.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive personal best registry view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the mastery_records rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for personal best registry: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports improvement without comparing players who used incompatible conditions.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# mastery_account_rewards — Account-Bound Recognition
> Node 10 of 10 in **Progression, Mastery and Recognition** · World Loom identity `mastery_account_rewards` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Special recognition rewards remain attached to the earning player and are excluded from the Bazaar by definition.
- Why this belongs in Moonbyte: It prevents achievements from being converted into trade pressure or status farming.
- Distinctive interaction: Complete the stated milestone, receive an account-bound mark and view its source in the profile.
- Successful completion produces: A non-transferable achievement reward with anti-duplication tests.
### 02 · Purpose and boundaries
- Product role: Progression recognises real accomplishments using clear thresholds. It never depends on daily attendance, spending or hidden random reward rolls.
- This system is the canonical owner of account-bound recognition behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Progression, Mastery and Recognition; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It prevents achievements from being converted into trade pressure or status farming.
- Stable destination ID: mastery_account_rewards; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .mastery account rewards and /mastery account rewards are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Complete the stated milestone and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to account-bound recognition.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Special recognition rewards remain attached to the earning player and are excluded from the Bazaar by definition.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Complete the stated milestone, receive an account-bound mark and view its source in the profile.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Account-Bound Recognition is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A non-transferable achievement reward with anti-duplication tests.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses game results and practice lessons as skill evidence: Curated Game Routes (arcade_playlists).
- provides predictable side-grade character and utility milestones: Skill Path Map (character_skill_paths).
- stores achievements, personal records and earned recognition: Collection Sets (archive_sets).
- supports optional guild recognition for contribution and cooperation: Guild Trust Record (guild_reputation).
### 12 · Incoming system relationships
- The Guild Chronicle (guild_archive) can use this system because it turns verified participation and skill accomplishments into transparent guild recognition.
- Chapter Arcs (quest_arcs) can use this system because it rewards completed skill and understanding with transparent records.
- Puzzle Generator and Validator (arcade_puzzle_factory) can use this system because it creates personal records, practice paths and transparent skill achievements.
- Role Synergy Board (character_party_synergy) can use this system because it uses transparent practice and achievements to unlock documented character options.
### 13 · Reward and economy contract
- Reward policy: A non-transferable achievement reward with anti-duplication tests.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive account-bound recognition view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the mastery_account_rewards rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for account-bound recognition: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It prevents achievements from being converted into trade pressure or status farming.; it must remain satisfying in a bounded session and support a natural stopping point.
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

## Domain 15 — Festivals, Chapters and World Events

Events are authored content windows and celebrations, not scarcity traps. Core rewards and story access remain available through archives or repeat routes.

**Domain dependencies:** Server Culture and Community Layer (`server`) · Guilds and Long-Form Cooperation (`guild`) · Atlas, Regions and World Discovery (`atlas`) · The Hearth and Tavern (`hearth`)

# festival_world_fairs — World Fairs
> Node 1 of 10 in **Festivals, Chapters and World Events** · World Loom identity `festival_world_fairs` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A festival groups mini-games, story scenes, crafting recipes and shared decorations under one authored theme.
- Why this belongs in Moonbyte: It provides a celebratory reason to explore multiple systems without a timed reward trap.
- Distinctive interaction: Open the fair, inspect activities and durations, choose one station and return later without loss.
- Successful completion produces: A festival progress record and permanent archive of completed content.
### 02 · Purpose and boundaries
- Product role: Events are authored content windows and celebrations, not scarcity traps. Core rewards and story access remain available through archives or repeat routes.
- This system is the canonical owner of world fairs behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Festivals, Chapters and World Events; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It provides a celebratory reason to explore multiple systems without a timed reward trap.
- Stable destination ID: festival_world_fairs; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .festival world fairs and /festival world fairs are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open the fair and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to world fairs.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A festival groups mini-games, story scenes, crafting recipes and shared decorations under one authored theme.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open the fair, inspect activities and durations, choose one station and return later without loss.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: World Fairs is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A festival progress record and permanent archive of completed content.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses community calendars and guild preferences for event planning: Server World Identity (server_identity).
- gives guilds cooperative roles in festivals and world-building: The Great Works Board (guild_projects).
- adds visible decorations, dialogues and optional puzzles to known regions: Atlas Overlays (atlas_map_layers).
- surfaces a festival calmly through the Tavern and player-selected digests: The Tavern Wayfinder (hearth_routes).
### 12 · Incoming system relationships
- Server World Identity (server_identity) can use this system because it turns server calendars into optional community events and celebrations.
- Biome Ecology (atlas_biomes) can use this system because it lets authored world events decorate or add content to known regions without removing access.
- Thoughtful Gift Exchange (fellowship_gifting) can use this system because it connects shared milestones to optional community events.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A festival progress record and permanent archive of completed content.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive world fairs view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the festival_world_fairs rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for world fairs: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It provides a celebratory reason to explore multiple systems without a timed reward trap.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# festival_chapters — Seasonal Story Chapters
> Node 2 of 10 in **Festivals, Chapters and World Events** · World Loom identity `festival_chapters` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A chapter release adds a new narrative arc and map content while older chapters remain available to new players.
- Why this belongs in Moonbyte: It creates a living story without forcing simultaneous attendance.
- Distinctive interaction: Read the chapter description, choose a starting point, play at a personal pace and archive the ending.
- Successful completion produces: A versioned chapter state and accessible replay route.
### 02 · Purpose and boundaries
- Product role: Events are authored content windows and celebrations, not scarcity traps. Core rewards and story access remain available through archives or repeat routes.
- This system is the canonical owner of seasonal story chapters behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Festivals, Chapters and World Events; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates a living story without forcing simultaneous attendance.
- Stable destination ID: festival_chapters; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .festival chapters and /festival chapters are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Read the chapter description and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to seasonal story chapters.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A chapter release adds a new narrative arc and map content while older chapters remain available to new players.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Read the chapter description, choose a starting point, play at a personal pace and archive the ending.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Seasonal Story Chapters is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned chapter state and accessible replay route.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses community calendars and guild preferences for event planning: Community Calendar (server_calendar).
- gives guilds cooperative roles in festivals and world-building: Guild Story Contracts (guild_quests).
- adds visible decorations, dialogues and optional puzzles to known regions: Cartographer's Desk (atlas_cartographer).
- surfaces a festival calmly through the Tavern and player-selected digests: The Lantern and Ladle (hearth_main_room).
### 12 · Incoming system relationships
- Community Calendar (server_calendar) can use this system because it turns server calendars into optional community events and celebrations.
- Landmark Network (atlas_landmarks) can use this system because it lets authored world events decorate or add content to known regions without removing access.
- Community Message Board (fellowship_mailbox) can use this system because it connects shared milestones to optional community events.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A versioned chapter state and accessible replay route.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive seasonal story chapters view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the festival_chapters rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for seasonal story chapters: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates a living story without forcing simultaneous attendance.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# festival_server_events — Server Celebration Kits
> Node 3 of 10 in **Festivals, Chapters and World Events** · World Loom identity `festival_server_events` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Administrators can assemble approved event schedules, party tables, quizzes and decoration options for their community.
- Why this belongs in Moonbyte: It makes Moonbyte adaptable to a server's culture and moderation needs.
- Distinctive interaction: Choose a kit, preview its content, set channel rules, publish the event and close it cleanly.
- Successful completion produces: A guild-owned event configuration with scoped permissions.
### 02 · Purpose and boundaries
- Product role: Events are authored content windows and celebrations, not scarcity traps. Core rewards and story access remain available through archives or repeat routes.
- This system is the canonical owner of server celebration kits behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Festivals, Chapters and World Events; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes Moonbyte adaptable to a server's culture and moderation needs.
- Stable destination ID: festival_server_events; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .festival server events and /festival server events are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a kit and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to server celebration kits.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Administrators can assemble approved event schedules, party tables, quizzes and decoration options for their community.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a kit, preview its content, set channel rules, publish the event and close it cleanly.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Server Celebration Kits is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A guild-owned event configuration with scoped permissions.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses community calendars and guild preferences for event planning: Channel Wayfinding (server_channels).
- gives guilds cooperative roles in festivals and world-building: The Guild Vault (guild_vault).
- adds visible decorations, dialogues and optional puzzles to known regions: The Route Planner (atlas_route_planner).
- surfaces a festival calmly through the Tavern and player-selected digests: The Notice Wall (hearth_notice_wall).
### 12 · Incoming system relationships
- Channel Wayfinding (server_channels) can use this system because it turns server calendars into optional community events and celebrations.
- Known Passage Network (atlas_shortcuts) can use this system because it lets authored world events decorate or add content to known regions without removing access.
- Positive Contribution Notes (fellowship_commendations) can use this system because it connects shared milestones to optional community events.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A guild-owned event configuration with scoped permissions.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive server celebration kits view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the festival_server_events rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for server celebration kits: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes Moonbyte adaptable to a server's culture and moderation needs.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# festival_world_projects — World Restoration Projects
> Node 4 of 10 in **Festivals, Chapters and World Events** · World Loom identity `festival_world_projects` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A shared project lets players contribute puzzle solutions, crafted objects or discoveries to a permanent world improvement.
- Why this belongs in Moonbyte: It makes the wider world react to community activity with a durable outcome.
- Distinctive interaction: Inspect a project, choose a contribution path, submit valid work and view the published world change.
- Successful completion produces: A global project milestone and world-state event.
### 02 · Purpose and boundaries
- Product role: Events are authored content windows and celebrations, not scarcity traps. Core rewards and story access remain available through archives or repeat routes.
- This system is the canonical owner of world restoration projects behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Festivals, Chapters and World Events; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes the wider world react to community activity with a durable outcome.
- Stable destination ID: festival_world_projects; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .festival world projects and /festival world projects are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect a project and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to world restoration projects.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A shared project lets players contribute puzzle solutions, crafted objects or discoveries to a permanent world improvement.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect a project, choose a contribution path, submit valid work and view the published world change.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: World Restoration Projects is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A global project milestone and world-state event.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses community calendars and guild preferences for event planning: Server Challenge Board (server_challenges).
- gives guilds cooperative roles in festivals and world-building: The Guild Cartography Room (guild_map).
- adds visible decorations, dialogues and optional puzzles to known regions: Living World State (atlas_world_state).
- surfaces a festival calmly through the Tavern and player-selected digests: The Rumour Circle (hearth_rumour_circle).
### 12 · Incoming system relationships
- Server Challenge Board (server_challenges) can use this system because it turns server calendars into optional community events and celebrations.
- Environmental Logic Sites (atlas_environment_puzzles) can use this system because it lets authored world events decorate or add content to known regions without removing access.
- Shared Session Memories (fellowship_memories) can use this system because it connects shared milestones to optional community events.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A global project milestone and world-state event.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive world restoration projects view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the festival_world_projects rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for world restoration projects: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes the wider world react to community activity with a durable outcome.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# festival_rotating_puzzles — Puzzle Rotation Archive
> Node 5 of 10 in **Festivals, Chapters and World Events** · World Loom identity `festival_rotating_puzzles` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A featured puzzle changes on a documented schedule while previous puzzles remain available in an archive.
- Why this belongs in Moonbyte: It keeps discovery fresh without making a missed day or week punitive.
- Distinctive interaction: Open the featured puzzle, inspect its date and rules, play now or choose an archived version.
- Successful completion produces: A dated puzzle instance and persistent archive entry.
### 02 · Purpose and boundaries
- Product role: Events are authored content windows and celebrations, not scarcity traps. Core rewards and story access remain available through archives or repeat routes.
- This system is the canonical owner of puzzle rotation archive behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Festivals, Chapters and World Events; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It keeps discovery fresh without making a missed day or week punitive.
- Stable destination ID: festival_rotating_puzzles; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .festival rotating puzzles and /festival rotating puzzles are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open the featured puzzle and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to puzzle rotation archive.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A featured puzzle changes on a documented schedule while previous puzzles remain available in an archive.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open the featured puzzle, inspect its date and rules, play now or choose an archived version.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Puzzle Rotation Archive is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A dated puzzle instance and persistent archive entry.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses community calendars and guild preferences for event planning: The Community Museum (server_museum).
- gives guilds cooperative roles in festivals and world-building: Guild Neighbourhoods (guild_alliances).
- adds visible decorations, dialogues and optional puzzles to known regions: The Grand Atlas (atlas_world_index).
- surfaces a festival calmly through the Tavern and player-selected digests: The Common Tables (hearth_table_games).
### 12 · Incoming system relationships
- The Community Museum (server_museum) can use this system because it turns server calendars into optional community events and celebrations.
- Atlas Overlays (atlas_map_layers) can use this system because it lets authored world events decorate or add content to known regions without removing access.
- Friend Discovery Routes (fellowship_discovery) can use this system because it connects shared milestones to optional community events.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A dated puzzle instance and persistent archive entry.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive puzzle rotation archive view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the festival_rotating_puzzles rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for puzzle rotation archive: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It keeps discovery fresh without making a missed day or week punitive.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# festival_celebration_wall — The Celebration Wall
> Node 6 of 10 in **Festivals, Chapters and World Events** · World Loom identity `festival_celebration_wall` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A shared board highlights completed guild projects, player creativity and milestone achievements with visibility controls.
- Why this belongs in Moonbyte: It shifts celebrations toward meaningful contribution instead of raw attendance totals.
- Distinctive interaction: Choose a verified event, preview its attribution, publish to an allowed audience and archive it.
- Successful completion produces: A public-safe celebration entry.
### 02 · Purpose and boundaries
- Product role: Events are authored content windows and celebrations, not scarcity traps. Core rewards and story access remain available through archives or repeat routes.
- This system is the canonical owner of the celebration wall behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Festivals, Chapters and World Events; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It shifts celebrations toward meaningful contribution instead of raw attendance totals.
- Stable destination ID: festival_celebration_wall; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .festival celebration wall and /festival celebration wall are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a verified event and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the celebration wall.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A shared board highlights completed guild projects, player creativity and milestone achievements with visibility controls.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a verified event, preview its attribution, publish to an allowed audience and archive it.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Celebration Wall is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A public-safe celebration entry.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses community calendars and guild preferences for event planning: Neighbourhood Goals (server_goals).
- gives guilds cooperative roles in festivals and world-building: Guild Trust Record (guild_reputation).
- adds visible decorations, dialogues and optional puzzles to known regions: Discovery Layers (atlas_discovery_fog).
- surfaces a festival calmly through the Tavern and player-selected digests: The Host's Bench (hearth_hosts_bench).
### 12 · Incoming system relationships
- Neighbourhood Goals (server_goals) can use this system because it turns server calendars into optional community events and celebrations.
- Cartographer's Desk (atlas_cartographer) can use this system because it lets authored world events decorate or add content to known regions without removing access.
- Social Boundaries Panel (fellowship_safety) can use this system because it connects shared milestones to optional community events.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A public-safe celebration entry.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the celebration wall view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the festival_celebration_wall rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the celebration wall: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It shifts celebrations toward meaningful contribution instead of raw attendance totals.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# festival_cross_server — Neighbourhood Festival Circuit
> Node 7 of 10 in **Festivals, Chapters and World Events** · World Loom identity `festival_cross_server` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Independent servers can opt into a shared puzzle or art theme while keeping configuration and user data separate.
- Why this belongs in Moonbyte: It gives Moonbyte a wider community feel without merging private server records.
- Distinctive interaction: Review the public event rules, opt in at guild level, submit a valid result and view the aggregate public outcome.
- Successful completion produces: A cross-server event result with no shared private profile data.
### 02 · Purpose and boundaries
- Product role: Events are authored content windows and celebrations, not scarcity traps. Core rewards and story access remain available through archives or repeat routes.
- This system is the canonical owner of neighbourhood festival circuit behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Festivals, Chapters and World Events; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives Moonbyte a wider community feel without merging private server records.
- Stable destination ID: festival_cross_server; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .festival cross server and /festival cross server are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Review the public event rules and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to neighbourhood festival circuit.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Independent servers can opt into a shared puzzle or art theme while keeping configuration and user data separate.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Review the public event rules, opt in at guild level, submit a valid result and view the aggregate public outcome.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Neighbourhood Festival Circuit is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A cross-server event result with no shared private profile data.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses community calendars and guild preferences for event planning: Scoped Scoreboards (server_leaderboards).
- gives guilds cooperative roles in festivals and world-building: The Guild Chronicle (guild_archive).
- adds visible decorations, dialogues and optional puzzles to known regions: Biome Ecology (atlas_biomes).
- surfaces a festival calmly through the Tavern and player-selected digests: The Story Stage (hearth_bard_stage).
### 12 · Incoming system relationships
- Scoped Scoreboards (server_leaderboards) can use this system because it turns server calendars into optional community events and celebrations.
- The Route Planner (atlas_route_planner) can use this system because it lets authored world events decorate or add content to known regions without removing access.
- Friend Circle (fellowship_friends) can use this system because it connects shared milestones to optional community events.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A cross-server event result with no shared private profile data.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive neighbourhood festival circuit view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the festival_cross_server rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for neighbourhood festival circuit: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives Moonbyte a wider community feel without merging private server records.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# festival_return_paths — Open Archive Routes
> Node 8 of 10 in **Festivals, Chapters and World Events** · World Loom identity `festival_return_paths` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Every festival or chapter has a later access route for players who were absent during its original presentation.
- Why this belongs in Moonbyte: It removes pressure to log in on a schedule and protects content longevity.
- Distinctive interaction: Browse the archive, inspect what is still playable, start a preserved activity and keep the resulting record.
- Successful completion produces: An evergreen content link and versioned archive state.
### 02 · Purpose and boundaries
- Product role: Events are authored content windows and celebrations, not scarcity traps. Core rewards and story access remain available through archives or repeat routes.
- This system is the canonical owner of open archive routes behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Festivals, Chapters and World Events; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It removes pressure to log in on a schedule and protects content longevity.
- Stable destination ID: festival_return_paths; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .festival return paths and /festival return paths are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Browse the archive and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to open archive routes.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Every festival or chapter has a later access route for players who were absent during its original presentation.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Browse the archive, inspect what is still playable, start a preserved activity and keep the resulting record.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Open Archive Routes is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: An evergreen content link and versioned archive state.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses community calendars and guild preferences for event planning: Server Steward Desk (server_moderation).
- gives guilds cooperative roles in festivals and world-building: Guild Charters (guild_charters).
- adds visible decorations, dialogues and optional puzzles to known regions: Landmark Network (atlas_landmarks).
- surfaces a festival calmly through the Tavern and player-selected digests: The Hearth Kitchen (hearth_kitchen).
### 12 · Incoming system relationships
- Server Steward Desk (server_moderation) can use this system because it turns server calendars into optional community events and celebrations.
- Living World State (atlas_world_state) can use this system because it lets authored world events decorate or add content to known regions without removing access.
- Cooperative Role Planner (fellowship_roles) can use this system because it connects shared milestones to optional community events.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: An evergreen content link and versioned archive state.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive open archive routes view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the festival_return_paths rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for open archive routes: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It removes pressure to log in on a schedule and protects content longevity.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# festival_chronicles — Festival Chronicle
> Node 9 of 10 in **Festivals, Chapters and World Events** · World Loom identity `festival_chronicles` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: The chronicle stores festival artwork, completed puzzles, authored stories and community summaries after an event ends.
- Why this belongs in Moonbyte: It turns temporary presentation into lasting community history.
- Distinctive interaction: Select an event, browse its sections, follow content links and add an original recap if permitted.
- Successful completion produces: A permanent event archive with attribution.
### 02 · Purpose and boundaries
- Product role: Events are authored content windows and celebrations, not scarcity traps. Core rewards and story access remain available through archives or repeat routes.
- This system is the canonical owner of festival chronicle behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Festivals, Chapters and World Events; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It turns temporary presentation into lasting community history.
- Stable destination ID: festival_chronicles; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .festival chronicles and /festival chronicles are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select an event and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to festival chronicle.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: The chronicle stores festival artwork, completed puzzles, authored stories and community summaries after an event ends.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select an event, browse its sections, follow content links and add an original recap if permitted.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Festival Chronicle is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A permanent event archive with attribution.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses community calendars and guild preferences for event planning: Server Data Boundary (server_privacy).
- gives guilds cooperative roles in festivals and world-building: The Guild Hall (guild_hall).
- adds visible decorations, dialogues and optional puzzles to known regions: Known Passage Network (atlas_shortcuts).
- surfaces a festival calmly through the Tavern and player-selected digests: The Regulars' Circle (hearth_regulars).
### 12 · Incoming system relationships
- Server Data Boundary (server_privacy) can use this system because it turns server calendars into optional community events and celebrations.
- The Grand Atlas (atlas_world_index) can use this system because it lets authored world events decorate or add content to known regions without removing access.
- Opt-In Mentor Sessions (fellowship_mentor) can use this system because it connects shared milestones to optional community events.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A permanent event archive with attribution.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive festival chronicle view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the festival_chronicles rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for festival chronicle: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It turns temporary presentation into lasting community history.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# festival_anniversary — Moonbyte Anniversary
> Node 10 of 10 in **Festivals, Chapters and World Events** · World Loom identity `festival_anniversary` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: An anniversary celebration looks back at the world's history, community projects and game milestones without resetting progress.
- Why this belongs in Moonbyte: It creates a shared moment of reflection rather than an exclusive reward race.
- Distinctive interaction: Open the anniversary page, view personal and server memories, play optional retrospective games and close it.
- Successful completion produces: A dated celebration archive and cosmetic recognition that is not power.
### 02 · Purpose and boundaries
- Product role: Events are authored content windows and celebrations, not scarcity traps. Core rewards and story access remain available through archives or repeat routes.
- This system is the canonical owner of moonbyte anniversary behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Festivals, Chapters and World Events; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates a shared moment of reflection rather than an exclusive reward race.
- Stable destination ID: festival_anniversary; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .festival anniversary and /festival anniversary are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open the anniversary page and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to moonbyte anniversary.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: An anniversary celebration looks back at the world's history, community projects and game milestones without resetting progress.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open the anniversary page, view personal and server memories, play optional retrospective games and close it.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Moonbyte Anniversary is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A dated celebration archive and cosmetic recognition that is not power.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses community calendars and guild preferences for event planning: The Quiet Community Digest (server_digest).
- gives guilds cooperative roles in festivals and world-building: Guild Role Charter (guild_roles).
- adds visible decorations, dialogues and optional puzzles to known regions: Environmental Logic Sites (atlas_environment_puzzles).
- surfaces a festival calmly through the Tavern and player-selected digests: The Common Room Project (hearth_renovation).
### 12 · Incoming system relationships
- The Quiet Community Digest (server_digest) can use this system because it turns server calendars into optional community events and celebrations.
- Discovery Layers (atlas_discovery_fog) can use this system because it lets authored world events decorate or add content to known regions without removing access.
- Interest Clubs (fellowship_clubs) can use this system because it connects shared milestones to optional community events.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A dated celebration archive and cosmetic recognition that is not power.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive moonbyte anniversary view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the festival_anniversary rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for moonbyte anniversary: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates a shared moment of reflection rather than an exclusive reward race.; it must remain satisfying in a bounded session and support a natural stopping point.
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

## Domain 16 — Homes, Gardens and Personal Spaces

Homes are expressive personal spaces where players display earned discoveries and craft decorations. Visiting and sharing are opt-in.

**Domain dependencies:** Gathering, Crafting and Workshops (`craft`) · Collections, Bestiary and Museum (`archive`) · Friends, Community Bonds and Social Play (`fellowship`) · Guilds and Long-Form Cooperation (`guild`)

# homestead_plot — Personal Home Plot
> Node 1 of 10 in **Homes, Gardens and Personal Spaces** · World Loom identity `homestead_plot` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Each player receives a simple personal space with a few themed areas that can be arranged using earned objects.
- Why this belongs in Moonbyte: It creates a calm long-term destination that expresses progress without requiring constant maintenance.
- Distinctive interaction: Open the home, choose an area, place an owned decoration, preview and save the layout.
- Successful completion produces: A personal layout state and an accessible text inventory.
### 02 · Purpose and boundaries
- Product role: Homes are expressive personal spaces where players display earned discoveries and craft decorations. Visiting and sharing are opt-in.
- This system is the canonical owner of personal home plot behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Homes, Gardens and Personal Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates a calm long-term destination that expresses progress without requiring constant maintenance.
- Stable destination ID: homestead_plot; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .homestead plot and /homestead plot are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open the home and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to personal home plot.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Each player receives a simple personal space with a few themed areas that can be arranged using earned objects.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open the home, choose an area, place an owned decoration, preview and save the layout.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Personal Home Plot is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A personal layout state and an accessible text inventory.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses crafted objects and workshop recipes as home furnishings: Field Gathering (craft_gathering).
- allows players to exhibit earned collection entries and curated sets: The Illustrated Compendium (archive_cards).
- supports opt-in friend visits and party gatherings: Positive Contribution Notes (fellowship_commendations).
- allows optional shared guild rooms and community displays: The Guild Chronicle (guild_archive).
### 12 · Incoming system relationships
- Field Gathering (craft_gathering) can use this system because it turns crafted objects into useful home decorations and workshop upgrades.
- The Relic Archive (archive_relics) can use this system because it lets players exhibit earned collections in a personally controlled space.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A personal layout state and an accessible text inventory.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive personal home plot view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the homestead_plot rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for personal home plot: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates a calm long-term destination that expresses progress without requiring constant maintenance.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# homestead_editor — Room Layout Studio
> Node 2 of 10 in **Homes, Gardens and Personal Spaces** · World Loom identity `homestead_editor` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A grid-based editor lets players position approved furniture and pixel art within bounded, mobile-friendly controls.
- Why this belongs in Moonbyte: It offers creative expression without relying on frame-perfect drag interactions.
- Distinctive interaction: Choose a room, select an object, place or rotate it, validate the grid and save a draft or publish it.
- Successful completion produces: A versioned layout recipe with collision and bounds validation.
### 02 · Purpose and boundaries
- Product role: Homes are expressive personal spaces where players display earned discoveries and craft decorations. Visiting and sharing are opt-in.
- This system is the canonical owner of room layout studio behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Homes, Gardens and Personal Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It offers creative expression without relying on frame-perfect drag interactions.
- Stable destination ID: homestead_editor; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .homestead editor and /homestead editor are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a room and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to room layout studio.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A grid-based editor lets players position approved furniture and pixel art within bounded, mobile-friendly controls.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a room, select an object, place or rotate it, validate the grid and save a draft or publish it.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Room Layout Studio is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned layout recipe with collision and bounds validation.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses crafted objects and workshop recipes as home furnishings: The Recipe Codex (craft_recipe_book).
- allows players to exhibit earned collection entries and curated sets: The Museum Hall (archive_museum).
- supports opt-in friend visits and party gatherings: Shared Session Memories (fellowship_memories).
- allows optional shared guild rooms and community displays: Guild Charters (guild_charters).
### 12 · Incoming system relationships
- The Recipe Codex (craft_recipe_book) can use this system because it turns crafted objects into useful home decorations and workshop upgrades.
- The Illustrated Compendium (archive_cards) can use this system because it lets players exhibit earned collections in a personally controlled space.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A versioned layout recipe with collision and bounds validation.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive room layout studio view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the homestead_editor rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for room layout studio: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It offers creative expression without relying on frame-perfect drag interactions.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# homestead_display — Display Shelves
> Node 3 of 10 in **Homes, Gardens and Personal Spaces** · World Loom identity `homestead_display` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Shelves expose selected collectibles and achievements to permitted visitors while leaving the rest of the inventory private.
- Why this belongs in Moonbyte: It turns a collection into a story without publishing ownership by default.
- Distinctive interaction: Choose a shelf, select entries, set visibility, preview the result and invite visitors if desired.
- Successful completion produces: A display configuration separate from inventory ownership.
### 02 · Purpose and boundaries
- Product role: Homes are expressive personal spaces where players display earned discoveries and craft decorations. Visiting and sharing are opt-in.
- This system is the canonical owner of display shelves behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Homes, Gardens and Personal Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It turns a collection into a story without publishing ownership by default.
- Stable destination ID: homestead_display; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .homestead display and /homestead display are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a shelf and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to display shelves.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Shelves expose selected collectibles and achievements to permitted visitors while leaving the rest of the inventory private.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a shelf, select entries, set visibility, preview the result and invite visitors if desired.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Display Shelves is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A display configuration separate from inventory ownership.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses crafted objects and workshop recipes as home furnishings: The Maker's Workshop (craft_workshop).
- allows players to exhibit earned collection entries and curated sets: Collection Sets (archive_sets).
- supports opt-in friend visits and party gatherings: Friend Discovery Routes (fellowship_discovery).
- allows optional shared guild rooms and community displays: The Guild Hall (guild_hall).
### 12 · Incoming system relationships
- The Maker's Workshop (craft_workshop) can use this system because it turns crafted objects into useful home decorations and workshop upgrades.
- The Museum Hall (archive_museum) can use this system because it lets players exhibit earned collections in a personally controlled space.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A display configuration separate from inventory ownership.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive display shelves view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the homestead_display rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for display shelves: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It turns a collection into a story without publishing ownership by default.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# homestead_garden — The Pocket Garden
> Node 4 of 10 in **Homes, Gardens and Personal Spaces** · World Loom identity `homestead_garden` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A garden uses deterministic planting and optional puzzle-based tending to create decorations and lore, with no decay for absence.
- Why this belongs in Moonbyte: It provides a gentle crafting loop without idle timers or punitive harvest windows.
- Distinctive interaction: Choose a plant, inspect its recipe, complete an optional care activity and collect when convenient.
- Successful completion produces: A persistent garden state with non-decaying progress.
### 02 · Purpose and boundaries
- Product role: Homes are expressive personal spaces where players display earned discoveries and craft decorations. Visiting and sharing are opt-in.
- This system is the canonical owner of the pocket garden behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Homes, Gardens and Personal Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It provides a gentle crafting loop without idle timers or punitive harvest windows.
- Stable destination ID: homestead_garden; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .homestead garden and /homestead garden are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a plant and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the pocket garden.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A garden uses deterministic planting and optional puzzle-based tending to create decorations and lore, with no decay for absence.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a plant, inspect its recipe, complete an optional care activity and collect when convenient.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Pocket Garden is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A persistent garden state with non-decaying progress.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses crafted objects and workshop recipes as home furnishings: Known Quality Grades (craft_quality).
- allows players to exhibit earned collection entries and curated sets: The Cosmetic Gallery (archive_cosmetics).
- supports opt-in friend visits and party gatherings: Social Boundaries Panel (fellowship_safety).
- allows optional shared guild rooms and community displays: Guild Role Charter (guild_roles).
### 12 · Incoming system relationships
- Known Quality Grades (craft_quality) can use this system because it turns crafted objects into useful home decorations and workshop upgrades.
- Collection Sets (archive_sets) can use this system because it lets players exhibit earned collections in a personally controlled space.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A persistent garden state with non-decaying progress.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the pocket garden view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the homestead_garden rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the pocket garden: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It provides a gentle crafting loop without idle timers or punitive harvest windows.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# homestead_workshop — Home Workshop Extension
> Node 5 of 10 in **Homes, Gardens and Personal Spaces** · World Loom identity `homestead_workshop` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can place a crafted workshop station that provides a more convenient view of existing recipes and loadouts.
- Why this belongs in Moonbyte: It links crafting to personal space without granting hidden production multipliers.
- Distinctive interaction: Craft or earn a station, place it, inspect its supported recipes and launch the standard workshop service.
- Successful completion produces: A home station configuration that references canonical recipes.
### 02 · Purpose and boundaries
- Product role: Homes are expressive personal spaces where players display earned discoveries and craft decorations. Visiting and sharing are opt-in.
- This system is the canonical owner of home workshop extension behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Homes, Gardens and Personal Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It links crafting to personal space without granting hidden production multipliers.
- Stable destination ID: homestead_workshop; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .homestead workshop and /homestead workshop are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Craft or earn a station and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to home workshop extension.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can place a crafted workshop station that provides a more convenient view of existing recipes and loadouts.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Craft or earn a station, place it, inspect its supported recipes and launch the standard workshop service.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Home Workshop Extension is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A home station configuration that references canonical recipes.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses crafted objects and workshop recipes as home furnishings: Commission Bench (craft_commissions).
- allows players to exhibit earned collection entries and curated sets: The Living Lore Index (archive_lore).
- supports opt-in friend visits and party gatherings: Friend Circle (fellowship_friends).
- allows optional shared guild rooms and community displays: The Great Works Board (guild_projects).
### 12 · Incoming system relationships
- Commission Bench (craft_commissions) can use this system because it turns crafted objects into useful home decorations and workshop upgrades.
- The Cosmetic Gallery (archive_cosmetics) can use this system because it lets players exhibit earned collections in a personally controlled space.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A home station configuration that references canonical recipes.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive home workshop extension view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the homestead_workshop rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for home workshop extension: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It links crafting to personal space without granting hidden production multipliers.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# homestead_companion_nook — Companion Nook
> Node 6 of 10 in **Homes, Gardens and Personal Spaces** · World Loom identity `homestead_companion_nook` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A home area displays companions already earned through story content and lets players choose a current non-combat appearance.
- Why this belongs in Moonbyte: It provides a visual home for original companions without paid acquisition or neglect mechanics.
- Distinctive interaction: Select an owned companion, preview its display pose, set visibility and save the room.
- Successful completion produces: A cosmetic companion display with no decay or feeding timer.
### 02 · Purpose and boundaries
- Product role: Homes are expressive personal spaces where players display earned discoveries and craft decorations. Visiting and sharing are opt-in.
- This system is the canonical owner of companion nook behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Homes, Gardens and Personal Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It provides a visual home for original companions without paid acquisition or neglect mechanics.
- Stable destination ID: homestead_companion_nook; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .homestead companion nook and /homestead companion nook are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select an owned companion and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to companion nook.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A home area displays companions already earned through story content and lets players choose a current non-combat appearance.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select an owned companion, preview its display pose, set visibility and save the room.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Companion Nook is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A cosmetic companion display with no decay or feeding timer.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses crafted objects and workshop recipes as home furnishings: Material Reclaim (craft_recycling).
- allows players to exhibit earned collection entries and curated sets: Collection Milestones (archive_milestones).
- supports opt-in friend visits and party gatherings: Cooperative Role Planner (fellowship_roles).
- allows optional shared guild rooms and community displays: Guild Story Contracts (guild_quests).
### 12 · Incoming system relationships
- Material Reclaim (craft_recycling) can use this system because it turns crafted objects into useful home decorations and workshop upgrades.
- The Living Lore Index (archive_lore) can use this system because it lets players exhibit earned collections in a personally controlled space.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A cosmetic companion display with no decay or feeding timer.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive companion nook view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the homestead_companion_nook rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for companion nook: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It provides a visual home for original companions without paid acquisition or neglect mechanics.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# homestead_decor_sets — Decorative Collections
> Node 7 of 10 in **Homes, Gardens and Personal Spaces** · World Loom identity `homestead_decor_sets` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Decor sets group furnishings by region, story or event and show exact item sources and missing pieces.
- Why this belongs in Moonbyte: It combines crafting and collections into a clear home-building goal.
- Distinctive interaction: Choose a set, inspect its recipe sources, craft or discover pieces and arrange them in the room.
- Successful completion produces: A completed decor set and optional display achievement.
### 02 · Purpose and boundaries
- Product role: Homes are expressive personal spaces where players display earned discoveries and craft decorations. Visiting and sharing are opt-in.
- This system is the canonical owner of decorative collections behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Homes, Gardens and Personal Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It combines crafting and collections into a clear home-building goal.
- Stable destination ID: homestead_decor_sets; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .homestead decor sets and /homestead decor sets are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a set and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to decorative collections.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Decor sets group furnishings by region, story or event and show exact item sources and missing pieces.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a set, inspect its recipe sources, craft or discover pieces and arrange them in the room.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Decorative Collections is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A completed decor set and optional display achievement.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses crafted objects and workshop recipes as home furnishings: Workshop Repair (craft_repair).
- allows players to exhibit earned collection entries and curated sets: Collector's Provenance View (archive_provenance).
- supports opt-in friend visits and party gatherings: Opt-In Mentor Sessions (fellowship_mentor).
- allows optional shared guild rooms and community displays: The Guild Vault (guild_vault).
### 12 · Incoming system relationships
- Workshop Repair (craft_repair) can use this system because it turns crafted objects into useful home decorations and workshop upgrades.
- Collection Milestones (archive_milestones) can use this system because it lets players exhibit earned collections in a personally controlled space.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A completed decor set and optional display achievement.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive decorative collections view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the homestead_decor_sets rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for decorative collections: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It combines crafting and collections into a clear home-building goal.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# homestead_visits — Friend Visit Rooms
> Node 8 of 10 in **Homes, Gardens and Personal Spaces** · World Loom identity `homestead_visits` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can create a visit link with explicit guest permissions and a short-lived view scope for their home.
- Why this belongs in Moonbyte: It supports social expression without opening private inventory or unsolicited visits.
- Distinctive interaction: Choose guest access, create a room visit, let invited friends view permitted areas and close access at any time.
- Successful completion produces: A visit session with explicit scope and audit-safe access expiry.
### 02 · Purpose and boundaries
- Product role: Homes are expressive personal spaces where players display earned discoveries and craft decorations. Visiting and sharing are opt-in.
- This system is the canonical owner of friend visit rooms behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Homes, Gardens and Personal Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports social expression without opening private inventory or unsolicited visits.
- Stable destination ID: homestead_visits; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .homestead visits and /homestead visits are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose guest access and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to friend visit rooms.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can create a visit link with explicit guest permissions and a short-lived view scope for their home.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose guest access, create a room visit, let invited friends view permitted areas and close access at any time.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Friend Visit Rooms is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A visit session with explicit scope and audit-safe access expiry.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses crafted objects and workshop recipes as home furnishings: Pattern and Set Assembly (craft_sets).
- allows players to exhibit earned collection entries and curated sets: The Grand Collection Index (archive_catalogue).
- supports opt-in friend visits and party gatherings: Interest Clubs (fellowship_clubs).
- allows optional shared guild rooms and community displays: The Guild Cartography Room (guild_map).
### 12 · Incoming system relationships
- Pattern and Set Assembly (craft_sets) can use this system because it turns crafted objects into useful home decorations and workshop upgrades.
- Collector's Provenance View (archive_provenance) can use this system because it lets players exhibit earned collections in a personally controlled space.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A visit session with explicit scope and audit-safe access expiry.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive friend visit rooms view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the homestead_visits rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for friend visit rooms: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports social expression without opening private inventory or unsolicited visits.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# homestead_neighbourhood — Shared Neighbourhood Square
> Node 9 of 10 in **Homes, Gardens and Personal Spaces** · World Loom identity `homestead_neighbourhood` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A guild or server can curate a shared public square with contributed decorations and community project displays.
- Why this belongs in Moonbyte: It creates a common space connected to guild projects and festivals.
- Distinctive interaction: Open the square, inspect its project, submit an eligible decoration and review publication permissions.
- Successful completion produces: A guild-scoped shared layout and contributor record.
### 02 · Purpose and boundaries
- Product role: Homes are expressive personal spaces where players display earned discoveries and craft decorations. Visiting and sharing are opt-in.
- This system is the canonical owner of shared neighbourhood square behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Homes, Gardens and Personal Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates a common space connected to guild projects and festivals.
- Stable destination ID: homestead_neighbourhood; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .homestead neighbourhood and /homestead neighbourhood are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open the square and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to shared neighbourhood square.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A guild or server can curate a shared public square with contributed decorations and community project displays.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open the square, inspect its project, submit an eligible decoration and review publication permissions.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Shared Neighbourhood Square is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A guild-scoped shared layout and contributor record.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses crafted objects and workshop recipes as home furnishings: Guild Workshop (craft_shared_bench).
- allows players to exhibit earned collection entries and curated sets: The Field Bestiary (archive_bestiary).
- supports opt-in friend visits and party gatherings: Thoughtful Gift Exchange (fellowship_gifting).
- allows optional shared guild rooms and community displays: Guild Neighbourhoods (guild_alliances).
### 12 · Incoming system relationships
- Guild Workshop (craft_shared_bench) can use this system because it turns crafted objects into useful home decorations and workshop upgrades.
- The Grand Collection Index (archive_catalogue) can use this system because it lets players exhibit earned collections in a personally controlled space.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A guild-scoped shared layout and contributor record.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive shared neighbourhood square view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the homestead_neighbourhood rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for shared neighbourhood square: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates a common space connected to guild projects and festivals.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# homestead_snapshots — Home Design Snapshots
> Node 10 of 10 in **Homes, Gardens and Personal Spaces** · World Loom identity `homestead_snapshots` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can save named layout snapshots, compare changes and restore an earlier approved arrangement.
- Why this belongs in Moonbyte: It makes creative editing reversible and reduces fear of losing work.
- Distinctive interaction: Save a snapshot, make changes, preview the diff and restore any retained version.
- Successful completion produces: A versioned layout history with owner-controlled retention.
### 02 · Purpose and boundaries
- Product role: Homes are expressive personal spaces where players display earned discoveries and craft decorations. Visiting and sharing are opt-in.
- This system is the canonical owner of home design snapshots behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Homes, Gardens and Personal Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes creative editing reversible and reduces fear of losing work.
- Stable destination ID: homestead_snapshots; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .homestead snapshots and /homestead snapshots are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Save a snapshot and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to home design snapshots.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can save named layout snapshots, compare changes and restore an earlier approved arrangement.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Save a snapshot, make changes, preview the diff and restore any retained version.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Home Design Snapshots is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned layout history with owner-controlled retention.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- uses crafted objects and workshop recipes as home furnishings: Recipe Research (craft_discovery).
- allows players to exhibit earned collection entries and curated sets: The Relic Archive (archive_relics).
- supports opt-in friend visits and party gatherings: Community Message Board (fellowship_mailbox).
- allows optional shared guild rooms and community displays: Guild Trust Record (guild_reputation).
### 12 · Incoming system relationships
- Recipe Research (craft_discovery) can use this system because it turns crafted objects into useful home decorations and workshop upgrades.
- The Field Bestiary (archive_bestiary) can use this system because it lets players exhibit earned collections in a personally controlled space.
- The Archive can preserve this node's completed outputs as evidence-backed entries.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A versioned layout history with owner-controlled retention.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive home design snapshots view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the homestead_snapshots rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for home design snapshots: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes creative editing reversible and reduces fear of losing work.; it must remain satisfying in a bounded session and support a natural stopping point.
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
