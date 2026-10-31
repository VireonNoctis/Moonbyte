# Volume II — Atlas, Expeditions and Story

> Moonbyte World Loom · 40 connected system proposals · This is a design specification, not a claim of implementation.

Each card is a distinct proposal with a stable graph identity, a player loop, state and ownership boundaries, Discord and Activity surfaces, privacy rules, acceptance tests and explicit cross-system links.

## Domain 5 — Atlas, Regions and World Discovery

The Atlas provides a navigable world graph. Discovery is evidence-based, persistent and useful to quests, expeditions, collections and community maps.

**Domain dependencies:** Expeditions and Exploration Sessions (`expedition`) · Quests, Mystery and Story (`quest`) · Collections, Bestiary and Museum (`archive`) · Festivals, Chapters and World Events (`festival`)

# atlas_world_index — The Grand Atlas
> Node 1 of 10 in **Atlas, Regions and World Discovery** · World Loom identity `atlas_world_index` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A canonical world map links regions, settlements, landmarks, routes and story chapters through stable IDs.
- Why this belongs in Moonbyte: It gives every feature a common place reference and prevents disconnected content lists.
- Distinctive interaction: Open the Atlas, choose a region, inspect its known routes, pin a destination and launch a compatible activity.
- Successful completion produces: A world graph and stable location IDs shared across game modules.
### 02 · Purpose and boundaries
- Product role: The Atlas provides a navigable world graph. Discovery is evidence-based, persistent and useful to quests, expeditions, collections and community maps.
- This system is the canonical owner of the grand atlas behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Atlas, Regions and World Discovery; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives every feature a common place reference and prevents disconnected content lists.
- Stable destination ID: atlas_world_index; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .atlas world index and /atlas world index are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open the Atlas and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the grand atlas.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A canonical world map links regions, settlements, landmarks, routes and story chapters through stable IDs.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open the Atlas, choose a region, inspect its known routes, pin a destination and launch a compatible activity.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Grand Atlas is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A world graph and stable location IDs shared across game modules.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- supplies destinations, distances, landmarks and route constraints for expeditions: Expedition Planner (expedition_planner).
- turns discovered places and clues into optional story routes: The Investigation Desk (quest_mysteries).
- stores discoveries, cartography and lore as durable collection entries: The Cosmetic Gallery (archive_cosmetics).
- lets authored world events decorate or add content to known regions without removing access: Moonbyte Anniversary (festival_anniversary).
### 12 · Incoming system relationships
- Expedition Planner (expedition_planner) can use this system because it selects known regions, routes, landmarks and environmental rules.
- Choice and Consequence Graph (quest_branches) can use this system because it places every quest objective at a named region, route or landmark.
- The Museum Hall (archive_museum) can use this system because it turns verified location discoveries and cartography into permanent entries.
- Neighbourhood Festival Circuit (festival_cross_server) can use this system because it adds visible decorations, dialogues and optional puzzles to known regions.
### 13 · Reward and economy contract
- Reward policy: A world graph and stable location IDs shared across game modules.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the grand atlas view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the atlas_world_index rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the grand atlas: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives every feature a common place reference and prevents disconnected content lists.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# atlas_discovery_fog — Discovery Layers
> Node 2 of 10 in **Atlas, Regions and World Discovery** · World Loom identity `atlas_discovery_fog` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A map distinguishes unknown, rumoured, visited and verified locations without hiding essential rules behind random rolls.
- Why this belongs in Moonbyte: It makes exploration meaningful while keeping players aware of what is speculation and what is confirmed.
- Distinctive interaction: Inspect a region, choose a lead, visit through a supported activity, verify evidence and update the map layer.
- Successful completion produces: A player-specific discovery state and optional public-safe map contribution.
### 02 · Purpose and boundaries
- Product role: The Atlas provides a navigable world graph. Discovery is evidence-based, persistent and useful to quests, expeditions, collections and community maps.
- This system is the canonical owner of discovery layers behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Atlas, Regions and World Discovery; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes exploration meaningful while keeping players aware of what is speculation and what is confirmed.
- Stable destination ID: atlas_discovery_fog; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .atlas discovery fog and /atlas discovery fog are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect a region and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to discovery layers.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A map distinguishes unknown, rumoured, visited and verified locations without hiding essential rules behind random rolls.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect a region, choose a lead, visit through a supported activity, verify evidence and update the map layer.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Discovery Layers is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A player-specific discovery state and optional public-safe map contribution.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- supplies destinations, distances, landmarks and route constraints for expeditions: Route Encounters (expedition_route_events).
- turns discovered places and clues into optional story routes: Dialogue Routes (quest_dialogue).
- stores discoveries, cartography and lore as durable collection entries: The Living Lore Index (archive_lore).
- lets authored world events decorate or add content to known regions without removing access: World Fairs (festival_world_fairs).
### 12 · Incoming system relationships
- Route Encounters (expedition_route_events) can use this system because it selects known regions, routes, landmarks and environmental rules.
- The Investigation Desk (quest_mysteries) can use this system because it places every quest objective at a named region, route or landmark.
- Collection Sets (archive_sets) can use this system because it turns verified location discoveries and cartography into permanent entries.
- Open Archive Routes (festival_return_paths) can use this system because it adds visible decorations, dialogues and optional puzzles to known regions.
### 13 · Reward and economy contract
- Reward policy: A player-specific discovery state and optional public-safe map contribution.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive discovery layers view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the atlas_discovery_fog rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for discovery layers: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes exploration meaningful while keeping players aware of what is speculation and what is confirmed.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# atlas_biomes — Biome Ecology
> Node 3 of 10 in **Atlas, Regions and World Discovery** · World Loom identity `atlas_biomes` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Each region has an authored visual palette, puzzle vocabulary, resources, NPC roles and environmental mechanics.
- Why this belongs in Moonbyte: It makes locations mechanically distinct instead of changing only the background image.
- Distinctive interaction: Choose a biome, learn its rules, encounter its resource and puzzle types, then carry discoveries into crafting or story.
- Successful completion produces: Versioned biome definitions and reusable environment rules.
### 02 · Purpose and boundaries
- Product role: The Atlas provides a navigable world graph. Discovery is evidence-based, persistent and useful to quests, expeditions, collections and community maps.
- This system is the canonical owner of biome ecology behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Atlas, Regions and World Discovery; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes locations mechanically distinct instead of changing only the background image.
- Stable destination ID: atlas_biomes; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .atlas biomes and /atlas biomes are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a biome and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to biome ecology.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Each region has an authored visual palette, puzzle vocabulary, resources, NPC roles and environmental mechanics.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a biome, learn its rules, encounter its resource and puzzle types, then carry discoveries into crafting or story.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Biome Ecology is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: Versioned biome definitions and reusable environment rules.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- supplies destinations, distances, landmarks and route constraints for expeditions: Expedition Roles (expedition_roles).
- turns discovered places and clues into optional story routes: Faction Storylines (quest_factions).
- stores discoveries, cartography and lore as durable collection entries: Collection Milestones (archive_milestones).
- lets authored world events decorate or add content to known regions without removing access: Seasonal Story Chapters (festival_chapters).
### 12 · Incoming system relationships
- Expedition Roles (expedition_roles) can use this system because it selects known regions, routes, landmarks and environmental rules.
- Dialogue Routes (quest_dialogue) can use this system because it places every quest objective at a named region, route or landmark.
- The Cosmetic Gallery (archive_cosmetics) can use this system because it turns verified location discoveries and cartography into permanent entries.
- Festival Chronicle (festival_chronicles) can use this system because it adds visible decorations, dialogues and optional puzzles to known regions.
### 13 · Reward and economy contract
- Reward policy: Versioned biome definitions and reusable environment rules.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive biome ecology view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the atlas_biomes rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for biome ecology: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes locations mechanically distinct instead of changing only the background image.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# atlas_landmarks — Landmark Network
> Node 4 of 10 in **Atlas, Regions and World Discovery** · World Loom identity `atlas_landmarks` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Landmarks act as stable navigation anchors with lore, nearby challenges, access notes and shared map references.
- Why this belongs in Moonbyte: It gives players memorable destinations and makes directions easy to discuss in Discord.
- Distinctive interaction: Select a landmark, read its context, follow a route, complete an optional interaction and save the discovery.
- Successful completion produces: Landmark state, route references and an archive record.
### 02 · Purpose and boundaries
- Product role: The Atlas provides a navigable world graph. Discovery is evidence-based, persistent and useful to quests, expeditions, collections and community maps.
- This system is the canonical owner of landmark network behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Atlas, Regions and World Discovery; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives players memorable destinations and makes directions easy to discuss in Discord.
- Stable destination ID: atlas_landmarks; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .atlas landmarks and /atlas landmarks are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select a landmark and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to landmark network.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Landmarks act as stable navigation anchors with lore, nearby challenges, access notes and shared map references.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select a landmark, read its context, follow a route, complete an optional interaction and save the discovery.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Landmark Network is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: Landmark state, route references and an archive record.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- supplies destinations, distances, landmarks and route constraints for expeditions: Prepared Supply Kit (expedition_supplies).
- turns discovered places and clues into optional story routes: Local Tales (quest_side_stories).
- stores discoveries, cartography and lore as durable collection entries: Collector's Provenance View (archive_provenance).
- lets authored world events decorate or add content to known regions without removing access: Server Celebration Kits (festival_server_events).
### 12 · Incoming system relationships
- Prepared Supply Kit (expedition_supplies) can use this system because it selects known regions, routes, landmarks and environmental rules.
- Faction Storylines (quest_factions) can use this system because it places every quest objective at a named region, route or landmark.
- The Living Lore Index (archive_lore) can use this system because it turns verified location discoveries and cartography into permanent entries.
- Moonbyte Anniversary (festival_anniversary) can use this system because it adds visible decorations, dialogues and optional puzzles to known regions.
### 13 · Reward and economy contract
- Reward policy: Landmark state, route references and an archive record.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive landmark network view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the atlas_landmarks rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for landmark network: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives players memorable destinations and makes directions easy to discuss in Discord.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# atlas_shortcuts — Known Passage Network
> Node 5 of 10 in **Atlas, Regions and World Discovery** · World Loom identity `atlas_shortcuts` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Verified shortcuts connect previously explored locations and remove repetitive navigation steps.
- Why this belongs in Moonbyte: It rewards knowledge and makes return trips faster without selling travel or requiring a timer.
- Distinctive interaction: Discover a passage, solve or fulfil its published requirement, confirm both endpoints and use it in later routes.
- Successful completion produces: A validated edge in the world graph with no hidden random travel outcome.
### 02 · Purpose and boundaries
- Product role: The Atlas provides a navigable world graph. Discovery is evidence-based, persistent and useful to quests, expeditions, collections and community maps.
- This system is the canonical owner of known passage network behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Atlas, Regions and World Discovery; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It rewards knowledge and makes return trips faster without selling travel or requiring a timer.
- Stable destination ID: atlas_shortcuts; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .atlas shortcuts and /atlas shortcuts are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Discover a passage and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to known passage network.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Verified shortcuts connect previously explored locations and remove repetitive navigation steps.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Discover a passage, solve or fulfil its published requirement, confirm both endpoints and use it in later routes.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Known Passage Network is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A validated edge in the world graph with no hidden random travel outcome.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- supplies destinations, distances, landmarks and route constraints for expeditions: Waystation Checkpoints (expedition_camps).
- turns discovered places and clues into optional story routes: Shared Story Decisions (quest_coop_story).
- stores discoveries, cartography and lore as durable collection entries: The Grand Collection Index (archive_catalogue).
- lets authored world events decorate or add content to known regions without removing access: World Restoration Projects (festival_world_projects).
### 12 · Incoming system relationships
- Waystation Checkpoints (expedition_camps) can use this system because it selects known regions, routes, landmarks and environmental rules.
- Local Tales (quest_side_stories) can use this system because it places every quest objective at a named region, route or landmark.
- Collection Milestones (archive_milestones) can use this system because it turns verified location discoveries and cartography into permanent entries.
- World Fairs (festival_world_fairs) can use this system because it adds visible decorations, dialogues and optional puzzles to known regions.
### 13 · Reward and economy contract
- Reward policy: A validated edge in the world graph with no hidden random travel outcome.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive known passage network view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the atlas_shortcuts rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for known passage network: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It rewards knowledge and makes return trips faster without selling travel or requiring a timer.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# atlas_environment_puzzles — Environmental Logic Sites
> Node 6 of 10 in **Atlas, Regions and World Discovery** · World Loom identity `atlas_environment_puzzles` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Some landmarks contain local puzzles that use the shape, order, symbols or history of the region.
- Why this belongs in Moonbyte: It makes the world itself playable and feeds puzzle knowledge into the wider quest graph.
- Distinctive interaction: Inspect environmental clues, collect evidence, test an allowed hypothesis, solve and record the method.
- Successful completion produces: A solved site state and an optional explanation for future players.
### 02 · Purpose and boundaries
- Product role: The Atlas provides a navigable world graph. Discovery is evidence-based, persistent and useful to quests, expeditions, collections and community maps.
- This system is the canonical owner of environmental logic sites behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Atlas, Regions and World Discovery; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes the world itself playable and feeds puzzle knowledge into the wider quest graph.
- Stable destination ID: atlas_environment_puzzles; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .atlas environment puzzles and /atlas environment puzzles are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect environmental clues and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to environmental logic sites.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Some landmarks contain local puzzles that use the shape, order, symbols or history of the region.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect environmental clues, collect evidence, test an allowed hypothesis, solve and record the method.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Environmental Logic Sites is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A solved site state and an optional explanation for future players.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- supplies destinations, distances, landmarks and route constraints for expeditions: The Expedition Journal (expedition_journal).
- turns discovered places and clues into optional story routes: The Chapter Chronicle (quest_chronicle).
- stores discoveries, cartography and lore as durable collection entries: The Field Bestiary (archive_bestiary).
- lets authored world events decorate or add content to known regions without removing access: Puzzle Rotation Archive (festival_rotating_puzzles).
### 12 · Incoming system relationships
- The Expedition Journal (expedition_journal) can use this system because it selects known regions, routes, landmarks and environmental rules.
- Shared Story Decisions (quest_coop_story) can use this system because it places every quest objective at a named region, route or landmark.
- Collector's Provenance View (archive_provenance) can use this system because it turns verified location discoveries and cartography into permanent entries.
- Seasonal Story Chapters (festival_chapters) can use this system because it adds visible decorations, dialogues and optional puzzles to known regions.
### 13 · Reward and economy contract
- Reward policy: A solved site state and an optional explanation for future players.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive environmental logic sites view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the atlas_environment_puzzles rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for environmental logic sites: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes the world itself playable and feeds puzzle knowledge into the wider quest graph.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# atlas_map_layers — Atlas Overlays
> Node 7 of 10 in **Atlas, Regions and World Discovery** · World Loom identity `atlas_map_layers` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can switch between routes, resources, story clues, public guild notes and personal pins without mixing their visibility.
- Why this belongs in Moonbyte: It makes a dense world readable and prevents private notes from becoming public by default.
- Distinctive interaction: Select a layer, filter symbols, inspect provenance and choose whether a note is personal, party or guild-visible.
- Successful completion produces: Separate versioned map layers and safe display metadata.
### 02 · Purpose and boundaries
- Product role: The Atlas provides a navigable world graph. Discovery is evidence-based, persistent and useful to quests, expeditions, collections and community maps.
- This system is the canonical owner of atlas overlays behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Atlas, Regions and World Discovery; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes a dense world readable and prevents private notes from becoming public by default.
- Stable destination ID: atlas_map_layers; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .atlas map layers and /atlas map layers are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select a layer and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to atlas overlays.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can switch between routes, resources, story clues, public guild notes and personal pins without mixing their visibility.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select a layer, filter symbols, inspect provenance and choose whether a note is personal, party or guild-visible.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Atlas Overlays is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: Separate versioned map layers and safe display metadata.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- supplies destinations, distances, landmarks and route constraints for expeditions: Return and Recovery Routes (expedition_recovery).
- turns discovered places and clues into optional story routes: Story Remix Paths (quest_remix).
- stores discoveries, cartography and lore as durable collection entries: The Relic Archive (archive_relics).
- lets authored world events decorate or add content to known regions without removing access: The Celebration Wall (festival_celebration_wall).
### 12 · Incoming system relationships
- Return and Recovery Routes (expedition_recovery) can use this system because it selects known regions, routes, landmarks and environmental rules.
- The Chapter Chronicle (quest_chronicle) can use this system because it places every quest objective at a named region, route or landmark.
- The Grand Collection Index (archive_catalogue) can use this system because it turns verified location discoveries and cartography into permanent entries.
- Server Celebration Kits (festival_server_events) can use this system because it adds visible decorations, dialogues and optional puzzles to known regions.
### 13 · Reward and economy contract
- Reward policy: Separate versioned map layers and safe display metadata.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive atlas overlays view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the atlas_map_layers rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for atlas overlays: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes a dense world readable and prevents private notes from becoming public by default.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# atlas_cartographer — Cartographer's Desk
> Node 8 of 10 in **Atlas, Regions and World Discovery** · World Loom identity `atlas_cartographer` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players can turn verified discoveries into clean maps, route guides and original annotations for other explorers.
- Why this belongs in Moonbyte: It creates a meaningful knowledge role and connects exploration to community authorship.
- Distinctive interaction: Choose a region, select verified points, arrange a route, add original notes, preview and publish at a chosen scope.
- Successful completion produces: A versioned cartographic work with source references.
### 02 · Purpose and boundaries
- Product role: The Atlas provides a navigable world graph. Discovery is evidence-based, persistent and useful to quests, expeditions, collections and community maps.
- This system is the canonical owner of cartographer's desk behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Atlas, Regions and World Discovery; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates a meaningful knowledge role and connects exploration to community authorship.
- Stable destination ID: atlas_cartographer; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .atlas cartographer and /atlas cartographer are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a region and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to cartographer's desk.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players can turn verified discoveries into clean maps, route guides and original annotations for other explorers.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a region, select verified points, arrange a route, add original notes, preview and publish at a chosen scope.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Cartographer's Desk is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned cartographic work with source references.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- supplies destinations, distances, landmarks and route constraints for expeditions: Explorer Contracts (expedition_contracts).
- turns discovered places and clues into optional story routes: The Quest Board (quest_board).
- stores discoveries, cartography and lore as durable collection entries: The Illustrated Compendium (archive_cards).
- lets authored world events decorate or add content to known regions without removing access: Neighbourhood Festival Circuit (festival_cross_server).
### 12 · Incoming system relationships
- Explorer Contracts (expedition_contracts) can use this system because it selects known regions, routes, landmarks and environmental rules.
- Story Remix Paths (quest_remix) can use this system because it places every quest objective at a named region, route or landmark.
- The Field Bestiary (archive_bestiary) can use this system because it turns verified location discoveries and cartography into permanent entries.
- World Restoration Projects (festival_world_projects) can use this system because it adds visible decorations, dialogues and optional puzzles to known regions.
### 13 · Reward and economy contract
- Reward policy: A versioned cartographic work with source references.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive cartographer's desk view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the atlas_cartographer rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for cartographer's desk: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates a meaningful knowledge role and connects exploration to community authorship.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# atlas_route_planner — The Route Planner
> Node 9 of 10 in **Atlas, Regions and World Discovery** · World Loom identity `atlas_route_planner` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A route planner estimates step count, puzzle difficulty and expected session time across known destinations.
- Why this belongs in Moonbyte: It helps players choose an adventure that fits their available time without forcing a long session.
- Distinctive interaction: Choose start and destination, apply party filters, preview route costs and hazards, then launch or save.
- Successful completion produces: A route proposal with explicit assumptions and no real-time travel pressure.
### 02 · Purpose and boundaries
- Product role: The Atlas provides a navigable world graph. Discovery is evidence-based, persistent and useful to quests, expeditions, collections and community maps.
- This system is the canonical owner of the route planner behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Atlas, Regions and World Discovery; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It helps players choose an adventure that fits their available time without forcing a long session.
- Stable destination ID: atlas_route_planner; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .atlas route planner and /atlas route planner are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose start and destination and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the route planner.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A route planner estimates step count, puzzle difficulty and expected session time across known destinations.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose start and destination, apply party filters, preview route costs and hazards, then launch or save.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Route Planner is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A route proposal with explicit assumptions and no real-time travel pressure.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- supplies destinations, distances, landmarks and route constraints for expeditions: Discovery Scorecard (expedition_score).
- turns discovered places and clues into optional story routes: Chapter Arcs (quest_arcs).
- stores discoveries, cartography and lore as durable collection entries: The Museum Hall (archive_museum).
- lets authored world events decorate or add content to known regions without removing access: Open Archive Routes (festival_return_paths).
### 12 · Incoming system relationships
- Discovery Scorecard (expedition_score) can use this system because it selects known regions, routes, landmarks and environmental rules.
- The Quest Board (quest_board) can use this system because it places every quest objective at a named region, route or landmark.
- The Relic Archive (archive_relics) can use this system because it turns verified location discoveries and cartography into permanent entries.
- Puzzle Rotation Archive (festival_rotating_puzzles) can use this system because it adds visible decorations, dialogues and optional puzzles to known regions.
### 13 · Reward and economy contract
- Reward policy: A route proposal with explicit assumptions and no real-time travel pressure.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the route planner view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the atlas_route_planner rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the route planner: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It helps players choose an adventure that fits their available time without forcing a long session.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# atlas_world_state — Living World State
> Node 10 of 10 in **Atlas, Regions and World Discovery** · World Loom identity `atlas_world_state` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Authored global chapter flags can alter region descriptions, NPC dialogue and available puzzles while preserving completed content.
- Why this belongs in Moonbyte: It creates a world that evolves through story outcomes rather than time-limited scarcity.
- Distinctive interaction: Read a chapter outcome, inspect the change log, visit an affected region and interact with its new state.
- Successful completion produces: Versioned world-state events with deterministic migrations and archival access.
### 02 · Purpose and boundaries
- Product role: The Atlas provides a navigable world graph. Discovery is evidence-based, persistent and useful to quests, expeditions, collections and community maps.
- This system is the canonical owner of living world state behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Atlas, Regions and World Discovery; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates a world that evolves through story outcomes rather than time-limited scarcity.
- Stable destination ID: atlas_world_state; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .atlas world state and /atlas world state are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Read a chapter outcome and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to living world state.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Authored global chapter flags can alter region descriptions, NPC dialogue and available puzzles while preserving completed content.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Read a chapter outcome, inspect the change log, visit an affected region and interact with its new state.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Living World State is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: Versioned world-state events with deterministic migrations and archival access.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- supplies destinations, distances, landmarks and route constraints for expeditions: Expedition Route Archive (expedition_replay).
- turns discovered places and clues into optional story routes: Choice and Consequence Graph (quest_branches).
- stores discoveries, cartography and lore as durable collection entries: Collection Sets (archive_sets).
- lets authored world events decorate or add content to known regions without removing access: Festival Chronicle (festival_chronicles).
### 12 · Incoming system relationships
- Expedition Route Archive (expedition_replay) can use this system because it selects known regions, routes, landmarks and environmental rules.
- Chapter Arcs (quest_arcs) can use this system because it places every quest objective at a named region, route or landmark.
- The Illustrated Compendium (archive_cards) can use this system because it turns verified location discoveries and cartography into permanent entries.
- The Celebration Wall (festival_celebration_wall) can use this system because it adds visible decorations, dialogues and optional puzzles to known regions.
### 13 · Reward and economy contract
- Reward policy: Versioned world-state events with deterministic migrations and archival access.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive living world state view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the atlas_world_state rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for living world state: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates a world that evolves through story outcomes rather than time-limited scarcity.; it must remain satisfying in a bounded session and support a natural stopping point.
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

## Domain 6 — Expeditions and Exploration Sessions

Expeditions turn the Atlas into bounded, resumable adventures with cooperative roles, clear checkpoints and discovery-based results.

**Domain dependencies:** Atlas, Regions and World Discovery (`atlas`) · Vaults, Dungeons and Puzzle Spaces (`vault`) · Guilds and Long-Form Cooperation (`guild`) · Collections, Bestiary and Museum (`archive`)

# expedition_planner — Expedition Planner
> Node 1 of 10 in **Expeditions and Exploration Sessions** · World Loom identity `expedition_planner` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A party chooses a destination, route, difficulty, session length and optional objectives before the journey begins.
- Why this belongs in Moonbyte: It converts open-world exploration into a bounded activity suitable for Discord interactions.
- Distinctive interaction: Select a destination, preview route and risks, invite party members, choose roles and confirm the plan.
- Successful completion produces: A versioned expedition plan with clear membership and estimated duration.
### 02 · Purpose and boundaries
- Product role: Expeditions turn the Atlas into bounded, resumable adventures with cooperative roles, clear checkpoints and discovery-based results.
- This system is the canonical owner of expedition planner behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Expeditions and Exploration Sessions; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It converts open-world exploration into a bounded activity suitable for Discord interactions.
- Stable destination ID: expedition_planner; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .expedition planner and /expedition planner are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select a destination and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to expedition planner.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A party chooses a destination, route, difficulty, session length and optional objectives before the journey begins.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select a destination, preview route and risks, invite party members, choose roles and confirm the plan.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Expedition Planner is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned expedition plan with clear membership and estimated duration.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- selects known regions, routes, landmarks and environmental rules: The Grand Atlas (atlas_world_index).
- routes suitable discoveries into puzzle chambers and structured challenge rooms: Relic Galleries (vault_relic_rooms).
- allows guilds to coordinate asynchronous contributions and party membership: The Guild Cartography Room (guild_map).
- turns discoveries, route notes and completed objectives into permanent entries: Collector's Provenance View (archive_provenance).
### 12 · Incoming system relationships
- Guild Charters (guild_charters) can use this system because it forms parties for map-based expeditions and cooperative routes.
- Biome Ecology (atlas_biomes) can use this system because it supplies destinations, distances, landmarks and route constraints for expeditions.
- Commission Bench (craft_commissions) can use this system because it creates disclosed route kits and tools before an expedition begins.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A versioned expedition plan with clear membership and estimated duration.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive expedition planner view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the expedition_planner rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for expedition planner: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It converts open-world exploration into a bounded activity suitable for Discord interactions.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# expedition_route_events — Route Encounters
> Node 2 of 10 in **Expeditions and Exploration Sessions** · World Loom identity `expedition_route_events` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Routes contain authored riddles, environmental obstacles, navigation choices and non-graphic turn-based encounters.
- Why this belongs in Moonbyte: It gives travel decisions a gameplay purpose without requiring real-time input or violent spectacle.
- Distinctive interaction: Reach a route node, inspect the challenge, choose a legal party action, resolve it and continue or retreat.
- Successful completion produces: A resolved route node and an explanation of the outcome.
### 02 · Purpose and boundaries
- Product role: Expeditions turn the Atlas into bounded, resumable adventures with cooperative roles, clear checkpoints and discovery-based results.
- This system is the canonical owner of route encounters behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Expeditions and Exploration Sessions; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives travel decisions a gameplay purpose without requiring real-time input or violent spectacle.
- Stable destination ID: expedition_route_events; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .expedition route events and /expedition route events are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Reach a route node and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to route encounters.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Routes contain authored riddles, environmental obstacles, navigation choices and non-graphic turn-based encounters.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Reach a route node, inspect the challenge, choose a legal party action, resolve it and continue or retreat.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Route Encounters is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A resolved route node and an explanation of the outcome.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- selects known regions, routes, landmarks and environmental rules: Discovery Layers (atlas_discovery_fog).
- routes suitable discoveries into puzzle chambers and structured challenge rooms: Mastery Trial Rooms (vault_mastery_trials).
- allows guilds to coordinate asynchronous contributions and party membership: Guild Neighbourhoods (guild_alliances).
- turns discoveries, route notes and completed objectives into permanent entries: The Grand Collection Index (archive_catalogue).
### 12 · Incoming system relationships
- The Guild Hall (guild_hall) can use this system because it forms parties for map-based expeditions and cooperative routes.
- Landmark Network (atlas_landmarks) can use this system because it supplies destinations, distances, landmarks and route constraints for expeditions.
- Material Reclaim (craft_recycling) can use this system because it creates disclosed route kits and tools before an expedition begins.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A resolved route node and an explanation of the outcome.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive route encounters view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the expedition_route_events rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for route encounters: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives travel decisions a gameplay purpose without requiring real-time input or violent spectacle.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# expedition_roles — Expedition Roles
> Node 3 of 10 in **Expeditions and Exploration Sessions** · World Loom identity `expedition_roles` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players select complementary roles such as Scout, Scholar, Navigator or Maker that expose different useful actions.
- Why this belongs in Moonbyte: It rewards coordination and knowledge rather than requiring one player to dominate every activity.
- Distinctive interaction: Choose a role, inspect its allowed actions, contribute during the matching phase and change roles between runs.
- Successful completion produces: A party role assignment and clear action permissions.
### 02 · Purpose and boundaries
- Product role: Expeditions turn the Atlas into bounded, resumable adventures with cooperative roles, clear checkpoints and discovery-based results.
- This system is the canonical owner of expedition roles behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Expeditions and Exploration Sessions; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It rewards coordination and knowledge rather than requiring one player to dominate every activity.
- Stable destination ID: expedition_roles; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .expedition roles and /expedition roles are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a role and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to expedition roles.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players select complementary roles such as Scout, Scholar, Navigator or Maker that expose different useful actions.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a role, inspect its allowed actions, contribute during the matching phase and change roles between runs.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Expedition Roles is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A party role assignment and clear action permissions.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- selects known regions, routes, landmarks and environmental rules: Biome Ecology (atlas_biomes).
- routes suitable discoveries into puzzle chambers and structured challenge rooms: The Grand Mechanism (vault_boss_logic).
- allows guilds to coordinate asynchronous contributions and party membership: Guild Trust Record (guild_reputation).
- turns discoveries, route notes and completed objectives into permanent entries: The Field Bestiary (archive_bestiary).
### 12 · Incoming system relationships
- Guild Role Charter (guild_roles) can use this system because it forms parties for map-based expeditions and cooperative routes.
- Known Passage Network (atlas_shortcuts) can use this system because it supplies destinations, distances, landmarks and route constraints for expeditions.
- Workshop Repair (craft_repair) can use this system because it creates disclosed route kits and tools before an expedition begins.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A party role assignment and clear action permissions.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive expedition roles view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the expedition_roles rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for expedition roles: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It rewards coordination and knowledge rather than requiring one player to dominate every activity.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# expedition_supplies — Prepared Supply Kit
> Node 4 of 10 in **Expeditions and Exploration Sessions** · World Loom identity `expedition_supplies` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Players assemble a visible kit from earned, non-premium materials with exact effects stated before departure.
- Why this belongs in Moonbyte: It links crafting and planning while keeping item consumption predictable.
- Distinctive interaction: Review a route's supply requirements, select owned items, confirm the kit and see the planned use for each item.
- Successful completion produces: A locked loadout snapshot with item provenance and deterministic use rules.
### 02 · Purpose and boundaries
- Product role: Expeditions turn the Atlas into bounded, resumable adventures with cooperative roles, clear checkpoints and discovery-based results.
- This system is the canonical owner of prepared supply kit behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Expeditions and Exploration Sessions; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It links crafting and planning while keeping item consumption predictable.
- Stable destination ID: expedition_supplies; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .expedition supplies and /expedition supplies are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Review a route's supply requirements and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to prepared supply kit.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Players assemble a visible kit from earned, non-premium materials with exact effects stated before departure.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Review a route's supply requirements, select owned items, confirm the kit and see the planned use for each item.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Prepared Supply Kit is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A locked loadout snapshot with item provenance and deterministic use rules.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- selects known regions, routes, landmarks and environmental rules: Landmark Network (atlas_landmarks).
- routes suitable discoveries into puzzle chambers and structured challenge rooms: Split-Path Chambers (vault_parallel_rooms).
- allows guilds to coordinate asynchronous contributions and party membership: The Guild Chronicle (guild_archive).
- turns discoveries, route notes and completed objectives into permanent entries: The Relic Archive (archive_relics).
### 12 · Incoming system relationships
- The Great Works Board (guild_projects) can use this system because it forms parties for map-based expeditions and cooperative routes.
- Environmental Logic Sites (atlas_environment_puzzles) can use this system because it supplies destinations, distances, landmarks and route constraints for expeditions.
- Pattern and Set Assembly (craft_sets) can use this system because it creates disclosed route kits and tools before an expedition begins.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A locked loadout snapshot with item provenance and deterministic use rules.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive prepared supply kit view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the expedition_supplies rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for prepared supply kit: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It links crafting and planning while keeping item consumption predictable.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# expedition_camps — Waystation Checkpoints
> Node 5 of 10 in **Expeditions and Exploration Sessions** · World Loom identity `expedition_camps` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Waystations let a party save its current route node, review progress and leave without losing completed objectives.
- Why this belongs in Moonbyte: It makes longer play respectful of real schedules and Discord disconnects.
- Distinctive interaction: Reach a checkpoint, inspect the saved state, choose continue, pause or leave, then resume from the exact checkpoint.
- Successful completion produces: A recoverable expedition checkpoint with idempotent progress.
### 02 · Purpose and boundaries
- Product role: Expeditions turn the Atlas into bounded, resumable adventures with cooperative roles, clear checkpoints and discovery-based results.
- This system is the canonical owner of waystation checkpoints behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Expeditions and Exploration Sessions; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes longer play respectful of real schedules and Discord disconnects.
- Stable destination ID: expedition_camps; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .expedition camps and /expedition camps are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Reach a checkpoint and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to waystation checkpoints.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Waystations let a party save its current route node, review progress and leave without losing completed objectives.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Reach a checkpoint, inspect the saved state, choose continue, pause or leave, then resume from the exact checkpoint.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Waystation Checkpoints is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A recoverable expedition checkpoint with idempotent progress.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- selects known regions, routes, landmarks and environmental rules: Known Passage Network (atlas_shortcuts).
- routes suitable discoveries into puzzle chambers and structured challenge rooms: Vault Save Stones (vault_checkpoints).
- allows guilds to coordinate asynchronous contributions and party membership: Guild Charters (guild_charters).
- turns discoveries, route notes and completed objectives into permanent entries: The Illustrated Compendium (archive_cards).
### 12 · Incoming system relationships
- Guild Story Contracts (guild_quests) can use this system because it forms parties for map-based expeditions and cooperative routes.
- Atlas Overlays (atlas_map_layers) can use this system because it supplies destinations, distances, landmarks and route constraints for expeditions.
- Guild Workshop (craft_shared_bench) can use this system because it creates disclosed route kits and tools before an expedition begins.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A recoverable expedition checkpoint with idempotent progress.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive waystation checkpoints view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the expedition_camps rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for waystation checkpoints: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes longer play respectful of real schedules and Discord disconnects.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# expedition_journal — The Expedition Journal
> Node 6 of 10 in **Expeditions and Exploration Sessions** · World Loom identity `expedition_journal` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A journal assembles route decisions, clues, discovered landmarks and party contributions into a readable recap.
- Why this belongs in Moonbyte: It makes each run useful even when the party does not complete every optional objective.
- Distinctive interaction: Record an event, attach a map point, add an original note, choose visibility and review the finished log.
- Successful completion produces: A structured private, party or guild journal entry.
### 02 · Purpose and boundaries
- Product role: Expeditions turn the Atlas into bounded, resumable adventures with cooperative roles, clear checkpoints and discovery-based results.
- This system is the canonical owner of the expedition journal behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Expeditions and Exploration Sessions; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes each run useful even when the party does not complete every optional objective.
- Stable destination ID: expedition_journal; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .expedition journal and /expedition journal are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Record an event and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the expedition journal.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A journal assembles route decisions, clues, discovered landmarks and party contributions into a readable recap.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Record an event, attach a map point, add an original note, choose visibility and review the finished log.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Expedition Journal is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A structured private, party or guild journal entry.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- selects known regions, routes, landmarks and environmental rules: Environmental Logic Sites (atlas_environment_puzzles).
- routes suitable discoveries into puzzle chambers and structured challenge rooms: Fair Difficulty Profiles (vault_difficulty).
- allows guilds to coordinate asynchronous contributions and party membership: The Guild Hall (guild_hall).
- turns discoveries, route notes and completed objectives into permanent entries: The Museum Hall (archive_museum).
### 12 · Incoming system relationships
- The Guild Vault (guild_vault) can use this system because it forms parties for map-based expeditions and cooperative routes.
- Cartographer's Desk (atlas_cartographer) can use this system because it supplies destinations, distances, landmarks and route constraints for expeditions.
- Recipe Research (craft_discovery) can use this system because it creates disclosed route kits and tools before an expedition begins.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A structured private, party or guild journal entry.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the expedition journal view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the expedition_journal rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the expedition journal: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes each run useful even when the party does not complete every optional objective.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# expedition_recovery — Return and Recovery Routes
> Node 7 of 10 in **Expeditions and Exploration Sessions** · World Loom identity `expedition_recovery` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A party that pauses or withdraws receives a clear return route and keeps every already completed objective.
- Why this belongs in Moonbyte: It prevents a failed or interrupted expedition from confiscating unrelated progress.
- Distinctive interaction: Choose to withdraw, review saved milestones, close active phases and return to the last safe hub.
- Successful completion produces: A clean session end state and preserved completed discoveries.
### 02 · Purpose and boundaries
- Product role: Expeditions turn the Atlas into bounded, resumable adventures with cooperative roles, clear checkpoints and discovery-based results.
- This system is the canonical owner of return and recovery routes behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Expeditions and Exploration Sessions; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It prevents a failed or interrupted expedition from confiscating unrelated progress.
- Stable destination ID: expedition_recovery; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .expedition recovery and /expedition recovery are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose to withdraw and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to return and recovery routes.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A party that pauses or withdraws receives a clear return route and keeps every already completed objective.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose to withdraw, review saved milestones, close active phases and return to the last safe hub.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Return and Recovery Routes is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A clean session end state and preserved completed discoveries.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- selects known regions, routes, landmarks and environmental rules: Atlas Overlays (atlas_map_layers).
- routes suitable discoveries into puzzle chambers and structured challenge rooms: Verified Puzzle Registry (vault_solution_registry).
- allows guilds to coordinate asynchronous contributions and party membership: Guild Role Charter (guild_roles).
- turns discoveries, route notes and completed objectives into permanent entries: Collection Sets (archive_sets).
### 12 · Incoming system relationships
- The Guild Cartography Room (guild_map) can use this system because it forms parties for map-based expeditions and cooperative routes.
- The Route Planner (atlas_route_planner) can use this system because it supplies destinations, distances, landmarks and route constraints for expeditions.
- Field Gathering (craft_gathering) can use this system because it creates disclosed route kits and tools before an expedition begins.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A clean session end state and preserved completed discoveries.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive return and recovery routes view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the expedition_recovery rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for return and recovery routes: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It prevents a failed or interrupted expedition from confiscating unrelated progress.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# expedition_contracts — Explorer Contracts
> Node 8 of 10 in **Expeditions and Exploration Sessions** · World Loom identity `expedition_contracts` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: An optional contract asks a party to document a specific route, solve a local puzzle or catalogue an authored discovery.
- Why this belongs in Moonbyte: It gives each expedition a clear purpose and feeds verified knowledge back to the world.
- Distinctive interaction: Read the objective and reward, attach a suitable route, complete it, submit evidence and collect the fixed result.
- Successful completion produces: A contract record linked to the Atlas and player or guild progress.
### 02 · Purpose and boundaries
- Product role: Expeditions turn the Atlas into bounded, resumable adventures with cooperative roles, clear checkpoints and discovery-based results.
- This system is the canonical owner of explorer contracts behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Expeditions and Exploration Sessions; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives each expedition a clear purpose and feeds verified knowledge back to the world.
- Stable destination ID: expedition_contracts; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .expedition contracts and /expedition contracts are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Read the objective and reward and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to explorer contracts.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: An optional contract asks a party to document a specific route, solve a local puzzle or catalogue an authored discovery.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Read the objective and reward, attach a suitable route, complete it, submit evidence and collect the fixed result.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Explorer Contracts is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A contract record linked to the Atlas and player or guild progress.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- selects known regions, routes, landmarks and environmental rules: Cartographer's Desk (atlas_cartographer).
- routes suitable discoveries into puzzle chambers and structured challenge rooms: Vault Blueprint Library (vault_blueprints).
- allows guilds to coordinate asynchronous contributions and party membership: The Great Works Board (guild_projects).
- turns discoveries, route notes and completed objectives into permanent entries: The Cosmetic Gallery (archive_cosmetics).
### 12 · Incoming system relationships
- Guild Neighbourhoods (guild_alliances) can use this system because it forms parties for map-based expeditions and cooperative routes.
- Living World State (atlas_world_state) can use this system because it supplies destinations, distances, landmarks and route constraints for expeditions.
- The Recipe Codex (craft_recipe_book) can use this system because it creates disclosed route kits and tools before an expedition begins.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A contract record linked to the Atlas and player or guild progress.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive explorer contracts view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the expedition_contracts rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for explorer contracts: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives each expedition a clear purpose and feeds verified knowledge back to the world.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# expedition_score — Discovery Scorecard
> Node 9 of 10 in **Expeditions and Exploration Sessions** · World Loom identity `expedition_score` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A scorecard recognises clue quality, optional objectives, teamwork and route efficiency with transparent scoring rules.
- Why this belongs in Moonbyte: It supports mastery without rewarding excessive session length or random loot.
- Distinctive interaction: Finish an expedition, inspect each scoring component, compare the rules version and save a personal record.
- Successful completion produces: A deterministic score and contribution summary.
### 02 · Purpose and boundaries
- Product role: Expeditions turn the Atlas into bounded, resumable adventures with cooperative roles, clear checkpoints and discovery-based results.
- This system is the canonical owner of discovery scorecard behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Expeditions and Exploration Sessions; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports mastery without rewarding excessive session length or random loot.
- Stable destination ID: expedition_score; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .expedition score and /expedition score are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Finish an expedition and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to discovery scorecard.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A scorecard recognises clue quality, optional objectives, teamwork and route efficiency with transparent scoring rules.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Finish an expedition, inspect each scoring component, compare the rules version and save a personal record.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Discovery Scorecard is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A deterministic score and contribution summary.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- selects known regions, routes, landmarks and environmental rules: The Route Planner (atlas_route_planner).
- routes suitable discoveries into puzzle chambers and structured challenge rooms: Puzzle Chambers (vault_chambers).
- allows guilds to coordinate asynchronous contributions and party membership: Guild Story Contracts (guild_quests).
- turns discoveries, route notes and completed objectives into permanent entries: The Living Lore Index (archive_lore).
### 12 · Incoming system relationships
- Guild Trust Record (guild_reputation) can use this system because it forms parties for map-based expeditions and cooperative routes.
- The Grand Atlas (atlas_world_index) can use this system because it supplies destinations, distances, landmarks and route constraints for expeditions.
- The Maker's Workshop (craft_workshop) can use this system because it creates disclosed route kits and tools before an expedition begins.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A deterministic score and contribution summary.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive discovery scorecard view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the expedition_score rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for discovery scorecard: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports mastery without rewarding excessive session length or random loot.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# expedition_replay — Expedition Route Archive
> Node 10 of 10 in **Expeditions and Exploration Sessions** · World Loom identity `expedition_replay` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Completed routes can be replayed with altered authored objectives or difficulty while preserving the original discovery record.
- Why this belongs in Moonbyte: It supports learning and group play without disguising repetition as new content.
- Distinctive interaction: Select an archived route, choose a published variant, start a fresh run and compare results.
- Successful completion produces: A new run linked to the original route version.
### 02 · Purpose and boundaries
- Product role: Expeditions turn the Atlas into bounded, resumable adventures with cooperative roles, clear checkpoints and discovery-based results.
- This system is the canonical owner of expedition route archive behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Expeditions and Exploration Sessions; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports learning and group play without disguising repetition as new content.
- Stable destination ID: expedition_replay; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .expedition replay and /expedition replay are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select an archived route and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to expedition route archive.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Completed routes can be replayed with altered authored objectives or difficulty while preserving the original discovery record.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select an archived route, choose a published variant, start a fresh run and compare results.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Expedition Route Archive is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A new run linked to the original route version.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- selects known regions, routes, landmarks and environmental rules: Living World State (atlas_world_state).
- routes suitable discoveries into puzzle chambers and structured challenge rooms: Logic Key Mechanisms (vault_logic_keys).
- allows guilds to coordinate asynchronous contributions and party membership: The Guild Vault (guild_vault).
- turns discoveries, route notes and completed objectives into permanent entries: Collection Milestones (archive_milestones).
### 12 · Incoming system relationships
- The Guild Chronicle (guild_archive) can use this system because it forms parties for map-based expeditions and cooperative routes.
- Discovery Layers (atlas_discovery_fog) can use this system because it supplies destinations, distances, landmarks and route constraints for expeditions.
- Known Quality Grades (craft_quality) can use this system because it creates disclosed route kits and tools before an expedition begins.
- Mastery can recognise the node's valid completion events under published criteria.
### 13 · Reward and economy contract
- Reward policy: A new run linked to the original route version.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive expedition route archive view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the expedition_replay rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for expedition route archive: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports learning and group play without disguising repetition as new content.; it must remain satisfying in a bounded session and support a natural stopping point.
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

## Domain 7 — Quests, Mystery and Story

Quests provide authored purpose across the Tavern, Atlas, Vaults, NPCs and guilds. Branches are visible, resumable and replayable.

**Domain dependencies:** Atlas, Regions and World Discovery (`atlas`) · Vaults, Dungeons and Puzzle Spaces (`vault`) · Progression, Mastery and Recognition (`mastery`) · The Hearth and Tavern (`hearth`)

# quest_board — The Quest Board
> Node 1 of 10 in **Quests, Mystery and Story** · World Loom identity `quest_board` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A curated board groups story chapters, side stories, guild requests and puzzles by topic and expected time.
- Why this belongs in Moonbyte: It makes the world's activity legible instead of turning quests into an endless notification stream.
- Distinctive interaction: Browse categories, inspect prerequisites and fixed rewards, pin an objective and accept it when ready.
- Successful completion produces: A player-owned quest state with visible conditions and no hidden timers.
### 02 · Purpose and boundaries
- Product role: Quests provide authored purpose across the Tavern, Atlas, Vaults, NPCs and guilds. Branches are visible, resumable and replayable.
- This system is the canonical owner of the quest board behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Quests, Mystery and Story; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes the world's activity legible instead of turning quests into an endless notification stream.
- Stable destination ID: quest_board; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .quest board and /quest board are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Browse categories and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the quest board.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A curated board groups story chapters, side stories, guild requests and puzzles by topic and expected time.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Browse categories, inspect prerequisites and fixed rewards, pin an objective and accept it when ready.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Quest Board is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A player-owned quest state with visible conditions and no hidden timers.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- places every quest objective at a named region, route or landmark: The Grand Atlas (atlas_world_index).
- lets quests culminate in structured puzzle or mastery spaces: Relic Galleries (vault_relic_rooms).
- rewards completed skill and understanding with transparent records: Earned Cosmetic Unlocks (mastery_cosmetics).
- surfaces rumours and story contracts in a calm central hub: The Tavern Wayfinder (hearth_routes).
### 12 · Incoming system relationships
- The Lantern and Ladle (hearth_main_room) can use this system because it converts rumours, NPC conversations and notice-board entries into optional quest starts.
- Biome Ecology (atlas_biomes) can use this system because it turns discovered places and clues into optional story routes.
- Mastery Trial Rooms (vault_mastery_trials) can use this system because it provides authored quest objectives and narrative context for each challenge.
- Equipment Ledger (character_equipment) can use this system because it connects character choices to authored story dialogue and branch flags.
### 13 · Reward and economy contract
- Reward policy: A player-owned quest state with visible conditions and no hidden timers.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the quest board view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the quest_board rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the quest board: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes the world's activity legible instead of turning quests into an endless notification stream.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# quest_arcs — Chapter Arcs
> Node 2 of 10 in **Quests, Mystery and Story** · World Loom identity `quest_arcs` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A chapter arc links multiple short quests into one story with checkpoints and clearly described completion conditions.
- Why this belongs in Moonbyte: It creates continuity across sessions without requiring a player to be online at a specific hour.
- Distinctive interaction: Read the chapter summary, choose an available branch, complete an objective, save and resume the next chapter.
- Successful completion produces: A versioned narrative graph and durable checkpoint.
### 02 · Purpose and boundaries
- Product role: Quests provide authored purpose across the Tavern, Atlas, Vaults, NPCs and guilds. Branches are visible, resumable and replayable.
- This system is the canonical owner of chapter arcs behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Quests, Mystery and Story; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates continuity across sessions without requiring a player to be online at a specific hour.
- Stable destination ID: quest_arcs; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .quest arcs and /quest arcs are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Read the chapter summary and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to chapter arcs.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A chapter arc links multiple short quests into one story with checkpoints and clearly described completion conditions.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Read the chapter summary, choose an available branch, complete an objective, save and resume the next chapter.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Chapter Arcs is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned narrative graph and durable checkpoint.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- places every quest objective at a named region, route or landmark: Discovery Layers (atlas_discovery_fog).
- lets quests culminate in structured puzzle or mastery spaces: Mastery Trial Rooms (vault_mastery_trials).
- rewards completed skill and understanding with transparent records: Skill Badge Portfolio (mastery_badges).
- surfaces rumours and story contracts in a calm central hub: The Lantern and Ladle (hearth_main_room).
### 12 · Incoming system relationships
- The Notice Wall (hearth_notice_wall) can use this system because it converts rumours, NPC conversations and notice-board entries into optional quest starts.
- Landmark Network (atlas_landmarks) can use this system because it turns discovered places and clues into optional story routes.
- The Grand Mechanism (vault_boss_logic) can use this system because it provides authored quest objectives and narrative context for each challenge.
- Titles and Identity Marks (character_titles) can use this system because it connects character choices to authored story dialogue and branch flags.
### 13 · Reward and economy contract
- Reward policy: A versioned narrative graph and durable checkpoint.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive chapter arcs view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the quest_arcs rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for chapter arcs: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates continuity across sessions without requiring a player to be online at a specific hour.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# quest_branches — Choice and Consequence Graph
> Node 3 of 10 in **Quests, Mystery and Story** · World Loom identity `quest_branches` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Important story decisions change later dialogue, available routes and archive summaries through authored rules.
- Why this belongs in Moonbyte: It gives players agency without secretly changing reward odds or punishing a non-optimal choice.
- Distinctive interaction: Review the choice context, inspect known consequences where appropriate, select an option and record the branch.
- Successful completion produces: A persistent branch flag with replay-safe consequences.
### 02 · Purpose and boundaries
- Product role: Quests provide authored purpose across the Tavern, Atlas, Vaults, NPCs and guilds. Branches are visible, resumable and replayable.
- This system is the canonical owner of choice and consequence graph behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Quests, Mystery and Story; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives players agency without secretly changing reward odds or punishing a non-optimal choice.
- Stable destination ID: quest_branches; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .quest branches and /quest branches are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Review the choice context and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to choice and consequence graph.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Important story decisions change later dialogue, available routes and archive summaries through authored rules.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Review the choice context, inspect known consequences where appropriate, select an option and record the branch.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Choice and Consequence Graph is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A persistent branch flag with replay-safe consequences.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- places every quest objective at a named region, route or landmark: Biome Ecology (atlas_biomes).
- lets quests culminate in structured puzzle or mastery spaces: The Grand Mechanism (vault_boss_logic).
- rewards completed skill and understanding with transparent records: Personal Best Registry (mastery_records).
- surfaces rumours and story contracts in a calm central hub: The Notice Wall (hearth_notice_wall).
### 12 · Incoming system relationships
- The Rumour Circle (hearth_rumour_circle) can use this system because it converts rumours, NPC conversations and notice-board entries into optional quest starts.
- Known Passage Network (atlas_shortcuts) can use this system because it turns discovered places and clues into optional story routes.
- Split-Path Chambers (vault_parallel_rooms) can use this system because it provides authored quest objectives and narrative context for each challenge.
- Pixel Persona Studio (character_appearance) can use this system because it connects character choices to authored story dialogue and branch flags.
### 13 · Reward and economy contract
- Reward policy: A persistent branch flag with replay-safe consequences.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive choice and consequence graph view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the quest_branches rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for choice and consequence graph: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives players agency without secretly changing reward odds or punishing a non-optimal choice.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# quest_mysteries — The Investigation Desk
> Node 4 of 10 in **Quests, Mystery and Story** · World Loom identity `quest_mysteries` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Mystery quests combine evidence cards, map references, NPC statements and deduction boards into a solvable case.
- Why this belongs in Moonbyte: It links social rumour, exploration and puzzle systems in a coherent detective experience.
- Distinctive interaction: Collect evidence, compare sources, form a hypothesis, test it against the rules and submit a conclusion.
- Successful completion produces: An evidence graph, hypothesis history and verified solution result.
### 02 · Purpose and boundaries
- Product role: Quests provide authored purpose across the Tavern, Atlas, Vaults, NPCs and guilds. Branches are visible, resumable and replayable.
- This system is the canonical owner of the investigation desk behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Quests, Mystery and Story; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It links social rumour, exploration and puzzle systems in a coherent detective experience.
- Stable destination ID: quest_mysteries; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .quest mysteries and /quest mysteries are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Collect evidence and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the investigation desk.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Mystery quests combine evidence cards, map references, NPC statements and deduction boards into a solvable case.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Collect evidence, compare sources, form a hypothesis, test it against the rules and submit a conclusion.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Investigation Desk is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: An evidence graph, hypothesis history and verified solution result.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- places every quest objective at a named region, route or landmark: Landmark Network (atlas_landmarks).
- lets quests culminate in structured puzzle or mastery spaces: Split-Path Chambers (vault_parallel_rooms).
- rewards completed skill and understanding with transparent records: Account-Bound Recognition (mastery_account_rewards).
- surfaces rumours and story contracts in a calm central hub: The Rumour Circle (hearth_rumour_circle).
### 12 · Incoming system relationships
- The Common Tables (hearth_table_games) can use this system because it converts rumours, NPC conversations and notice-board entries into optional quest starts.
- Environmental Logic Sites (atlas_environment_puzzles) can use this system because it turns discovered places and clues into optional story routes.
- Vault Save Stones (vault_checkpoints) can use this system because it provides authored quest objectives and narrative context for each challenge.
- Non-Combat Talents (character_utility) can use this system because it connects character choices to authored story dialogue and branch flags.
### 13 · Reward and economy contract
- Reward policy: An evidence graph, hypothesis history and verified solution result.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the investigation desk view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the quest_mysteries rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the investigation desk: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It links social rumour, exploration and puzzle systems in a coherent detective experience.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# quest_dialogue — Dialogue Routes
> Node 5 of 10 in **Quests, Mystery and Story** · World Loom identity `quest_dialogue` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: NPC conversations expose topic-based dialogue nodes that react to completed story flags and the player's selected approach.
- Why this belongs in Moonbyte: It makes character writing part of the game engine rather than hard-coded command text.
- Distinctive interaction: Choose a topic, inspect available responses, make a narrative choice and unlock the relevant follow-up.
- Successful completion produces: A versioned dialogue node and associated story flags.
### 02 · Purpose and boundaries
- Product role: Quests provide authored purpose across the Tavern, Atlas, Vaults, NPCs and guilds. Branches are visible, resumable and replayable.
- This system is the canonical owner of dialogue routes behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Quests, Mystery and Story; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes character writing part of the game engine rather than hard-coded command text.
- Stable destination ID: quest_dialogue; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .quest dialogue and /quest dialogue are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a topic and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to dialogue routes.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: NPC conversations expose topic-based dialogue nodes that react to completed story flags and the player's selected approach.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a topic, inspect available responses, make a narrative choice and unlock the relevant follow-up.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Dialogue Routes is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A versioned dialogue node and associated story flags.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- places every quest objective at a named region, route or landmark: Known Passage Network (atlas_shortcuts).
- lets quests culminate in structured puzzle or mastery spaces: Vault Save Stones (vault_checkpoints).
- rewards completed skill and understanding with transparent records: Experience and Milestone Ledger (mastery_experience).
- surfaces rumours and story contracts in a calm central hub: The Common Tables (hearth_table_games).
### 12 · Incoming system relationships
- The Host's Bench (hearth_hosts_bench) can use this system because it converts rumours, NPC conversations and notice-board entries into optional quest starts.
- Atlas Overlays (atlas_map_layers) can use this system because it turns discovered places and clues into optional story routes.
- Fair Difficulty Profiles (vault_difficulty) can use this system because it provides authored quest objectives and narrative context for each challenge.
- Player Character Sheet (character_identity) can use this system because it connects character choices to authored story dialogue and branch flags.
### 13 · Reward and economy contract
- Reward policy: A versioned dialogue node and associated story flags.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive dialogue routes view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the quest_dialogue rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for dialogue routes: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes character writing part of the game engine rather than hard-coded command text.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# quest_factions — Faction Storylines
> Node 6 of 10 in **Quests, Mystery and Story** · World Loom identity `quest_factions` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Several authored factions offer parallel narratives and useful world services without requiring allegiance for basic play.
- Why this belongs in Moonbyte: It provides long-term narrative choice without locking players out of core content.
- Distinctive interaction: Read each faction's values, complete an introductory task, choose optional cooperation and track consequences.
- Successful completion produces: Faction state and clearly scoped rewards or dialogue changes.
### 02 · Purpose and boundaries
- Product role: Quests provide authored purpose across the Tavern, Atlas, Vaults, NPCs and guilds. Branches are visible, resumable and replayable.
- This system is the canonical owner of faction storylines behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Quests, Mystery and Story; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It provides long-term narrative choice without locking players out of core content.
- Stable destination ID: quest_factions; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .quest factions and /quest factions are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Read each faction's values and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to faction storylines.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Several authored factions offer parallel narratives and useful world services without requiring allegiance for basic play.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Read each faction's values, complete an introductory task, choose optional cooperation and track consequences.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Faction Storylines is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: Faction state and clearly scoped rewards or dialogue changes.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- places every quest objective at a named region, route or landmark: Environmental Logic Sites (atlas_environment_puzzles).
- lets quests culminate in structured puzzle or mastery spaces: Fair Difficulty Profiles (vault_difficulty).
- rewards completed skill and understanding with transparent records: Skill Mastery Tracks (mastery_tracks).
- surfaces rumours and story contracts in a calm central hub: The Host's Bench (hearth_hosts_bench).
### 12 · Incoming system relationships
- The Story Stage (hearth_bard_stage) can use this system because it converts rumours, NPC conversations and notice-board entries into optional quest starts.
- Cartographer's Desk (atlas_cartographer) can use this system because it turns discovered places and clues into optional story routes.
- Verified Puzzle Registry (vault_solution_registry) can use this system because it provides authored quest objectives and narrative context for each challenge.
- Archetype Codex (character_archetypes) can use this system because it connects character choices to authored story dialogue and branch flags.
### 13 · Reward and economy contract
- Reward policy: Faction state and clearly scoped rewards or dialogue changes.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive faction storylines view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the quest_factions rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for faction storylines: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It provides long-term narrative choice without locking players out of core content.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# quest_side_stories — Local Tales
> Node 7 of 10 in **Quests, Mystery and Story** · World Loom identity `quest_side_stories` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Small stories attached to a landmark, item, guild member or tavern regular provide self-contained adventures.
- Why this belongs in Moonbyte: It ensures a player can have a satisfying short session without committing to a chapter arc.
- Distinctive interaction: Find a lead, read the setup, solve one or more compact objectives and archive the ending.
- Successful completion produces: A complete short narrative record.
### 02 · Purpose and boundaries
- Product role: Quests provide authored purpose across the Tavern, Atlas, Vaults, NPCs and guilds. Branches are visible, resumable and replayable.
- This system is the canonical owner of local tales behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Quests, Mystery and Story; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It ensures a player can have a satisfying short session without committing to a chapter arc.
- Stable destination ID: quest_side_stories; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .quest side stories and /quest side stories are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Find a lead and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to local tales.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Small stories attached to a landmark, item, guild member or tavern regular provide self-contained adventures.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Find a lead, read the setup, solve one or more compact objectives and archive the ending.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Local Tales is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A complete short narrative record.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- places every quest objective at a named region, route or landmark: Atlas Overlays (atlas_map_layers).
- lets quests culminate in structured puzzle or mastery spaces: Verified Puzzle Registry (vault_solution_registry).
- rewards completed skill and understanding with transparent records: Achievement Registry (mastery_achievements).
- surfaces rumours and story contracts in a calm central hub: The Story Stage (hearth_bard_stage).
### 12 · Incoming system relationships
- The Hearth Kitchen (hearth_kitchen) can use this system because it converts rumours, NPC conversations and notice-board entries into optional quest starts.
- The Route Planner (atlas_route_planner) can use this system because it turns discovered places and clues into optional story routes.
- Vault Blueprint Library (vault_blueprints) can use this system because it provides authored quest objectives and narrative context for each challenge.
- Skill Path Map (character_skill_paths) can use this system because it connects character choices to authored story dialogue and branch flags.
### 13 · Reward and economy contract
- Reward policy: A complete short narrative record.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive local tales view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the quest_side_stories rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for local tales: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It ensures a player can have a satisfying short session without committing to a chapter arc.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# quest_coop_story — Shared Story Decisions
> Node 8 of 10 in **Quests, Mystery and Story** · World Loom identity `quest_coop_story` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A party can pool clues and make one agreed story decision with each participant seeing the same outcome and contribution summary.
- Why this belongs in Moonbyte: It makes collaborative storytelling a first-class multiplayer mode.
- Distinctive interaction: Create a shared chapter, gather private or public clues, vote under clear rules and commit one final choice.
- Successful completion produces: A party-owned branch with contribution attribution.
### 02 · Purpose and boundaries
- Product role: Quests provide authored purpose across the Tavern, Atlas, Vaults, NPCs and guilds. Branches are visible, resumable and replayable.
- This system is the canonical owner of shared story decisions behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Quests, Mystery and Story; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes collaborative storytelling a first-class multiplayer mode.
- Stable destination ID: quest_coop_story; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .quest coop story and /quest coop story are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Create a shared chapter and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to shared story decisions.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A party can pool clues and make one agreed story decision with each participant seeing the same outcome and contribution summary.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Create a shared chapter, gather private or public clues, vote under clear rules and commit one final choice.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Shared Story Decisions is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A party-owned branch with contribution attribution.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- places every quest objective at a named region, route or landmark: Cartographer's Desk (atlas_cartographer).
- lets quests culminate in structured puzzle or mastery spaces: Vault Blueprint Library (vault_blueprints).
- rewards completed skill and understanding with transparent records: Player Honour Record (mastery_reputation).
- surfaces rumours and story contracts in a calm central hub: The Hearth Kitchen (hearth_kitchen).
### 12 · Incoming system relationships
- The Regulars' Circle (hearth_regulars) can use this system because it converts rumours, NPC conversations and notice-board entries into optional quest starts.
- Living World State (atlas_world_state) can use this system because it turns discovered places and clues into optional story routes.
- Puzzle Chambers (vault_chambers) can use this system because it provides authored quest objectives and narrative context for each challenge.
- Adventure Loadouts (character_loadouts) can use this system because it connects character choices to authored story dialogue and branch flags.
### 13 · Reward and economy contract
- Reward policy: A party-owned branch with contribution attribution.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive shared story decisions view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the quest_coop_story rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for shared story decisions: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes collaborative storytelling a first-class multiplayer mode.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# quest_chronicle — The Chapter Chronicle
> Node 9 of 10 in **Quests, Mystery and Story** · World Loom identity `quest_chronicle` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A searchable archive records completed branches, important clues, character appearances and world-state changes.
- Why this belongs in Moonbyte: It helps players remember the story and supports returning after a long pause.
- Distinctive interaction: Open a chapter, filter its entries, inspect source links and resume from an available checkpoint.
- Successful completion produces: A player-facing narrative history that respects visibility.
### 02 · Purpose and boundaries
- Product role: Quests provide authored purpose across the Tavern, Atlas, Vaults, NPCs and guilds. Branches are visible, resumable and replayable.
- This system is the canonical owner of the chapter chronicle behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Quests, Mystery and Story; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It helps players remember the story and supports returning after a long pause.
- Stable destination ID: quest_chronicle; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .quest chronicle and /quest chronicle are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Open a chapter and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the chapter chronicle.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A searchable archive records completed branches, important clues, character appearances and world-state changes.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Open a chapter, filter its entries, inspect source links and resume from an available checkpoint.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Chapter Chronicle is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A player-facing narrative history that respects visibility.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- places every quest objective at a named region, route or landmark: The Route Planner (atlas_route_planner).
- lets quests culminate in structured puzzle or mastery spaces: Puzzle Chambers (vault_chambers).
- rewards completed skill and understanding with transparent records: Personal Milestone Contracts (mastery_contracts).
- surfaces rumours and story contracts in a calm central hub: The Regulars' Circle (hearth_regulars).
### 12 · Incoming system relationships
- The Common Room Project (hearth_renovation) can use this system because it converts rumours, NPC conversations and notice-board entries into optional quest starts.
- The Grand Atlas (atlas_world_index) can use this system because it turns discovered places and clues into optional story routes.
- Logic Key Mechanisms (vault_logic_keys) can use this system because it provides authored quest objectives and narrative context for each challenge.
- Companion Chronicle (character_companions) can use this system because it connects character choices to authored story dialogue and branch flags.
### 13 · Reward and economy contract
- Reward policy: A player-facing narrative history that respects visibility.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the chapter chronicle view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the quest_chronicle rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the chapter chronicle: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It helps players remember the story and supports returning after a long pause.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# quest_remix — Story Remix Paths
> Node 10 of 10 in **Quests, Mystery and Story** · World Loom identity `quest_remix` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A completed story can be replayed through clearly labelled alternate routes that do not erase the canonical first run.
- Why this belongs in Moonbyte: It supports curiosity and replay without forcing reset or loss of progress.
- Distinctive interaction: Choose an archived branch, select a remix variant, review differences and play in a separate run context.
- Successful completion produces: An additional branch record kept beside the original.
### 02 · Purpose and boundaries
- Product role: Quests provide authored purpose across the Tavern, Atlas, Vaults, NPCs and guilds. Branches are visible, resumable and replayable.
- This system is the canonical owner of story remix paths behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Quests, Mystery and Story; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports curiosity and replay without forcing reset or loss of progress.
- Stable destination ID: quest_remix; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .quest remix and /quest remix are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose an archived branch and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to story remix paths.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A completed story can be replayed through clearly labelled alternate routes that do not erase the canonical first run.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose an archived branch, select a remix variant, review differences and play in a separate run context.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Story Remix Paths is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: An additional branch record kept beside the original.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- places every quest objective at a named region, route or landmark: Living World State (atlas_world_state).
- lets quests culminate in structured puzzle or mastery spaces: Logic Key Mechanisms (vault_logic_keys).
- rewards completed skill and understanding with transparent records: Milestone Route Planner (mastery_routes).
- surfaces rumours and story contracts in a calm central hub: The Common Room Project (hearth_renovation).
### 12 · Incoming system relationships
- The Tavern Wayfinder (hearth_routes) can use this system because it converts rumours, NPC conversations and notice-board entries into optional quest starts.
- Discovery Layers (atlas_discovery_fog) can use this system because it turns discovered places and clues into optional story routes.
- Relic Galleries (vault_relic_rooms) can use this system because it provides authored quest objectives and narrative context for each challenge.
- Role Synergy Board (character_party_synergy) can use this system because it connects character choices to authored story dialogue and branch flags.
### 13 · Reward and economy contract
- Reward policy: An additional branch record kept beside the original.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive story remix paths view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the quest_remix rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for story remix paths: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports curiosity and replay without forcing reset or loss of progress.; it must remain satisfying in a bounded session and support a natural stopping point.
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

## Domain 8 — Vaults, Dungeons and Puzzle Spaces

Vaults are designed challenge spaces with disclosed rules, fixed rewards, saved checkpoints and no paid or chance-based loot-box behaviour.

**Domain dependencies:** Quests, Mystery and Story (`quest`) · Gathering, Crafting and Workshops (`craft`) · Fair Play, Competition and Verification (`fairness`) · Guilds and Long-Form Cooperation (`guild`)

# vault_blueprints — Vault Blueprint Library
> Node 1 of 10 in **Vaults, Dungeons and Puzzle Spaces** · World Loom identity `vault_blueprints` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Every Vault is assembled from a versioned blueprint that states room order, rules, difficulty, solutions and exact rewards.
- Why this belongs in Moonbyte: It makes challenge content testable and prevents a Vault from becoming a hidden random-reward box.
- Distinctive interaction: Preview a blueprint, inspect its session length and solution policy, choose solo or party mode and enter.
- Successful completion produces: A validated Vault definition with explicit versions and stable room IDs.
### 02 · Purpose and boundaries
- Product role: Vaults are designed challenge spaces with disclosed rules, fixed rewards, saved checkpoints and no paid or chance-based loot-box behaviour.
- This system is the canonical owner of vault blueprint library behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Vaults, Dungeons and Puzzle Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It makes challenge content testable and prevents a Vault from becoming a hidden random-reward box.
- Stable destination ID: vault_blueprints; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .vault blueprints and /vault blueprints are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Preview a blueprint and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to vault blueprint library.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Every Vault is assembled from a versioned blueprint that states room order, rules, difficulty, solutions and exact rewards.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Preview a blueprint, inspect its session length and solution policy, choose solo or party mode and enter.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Vault Blueprint Library is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A validated Vault definition with explicit versions and stable room IDs.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides authored quest objectives and narrative context for each challenge: The Quest Board (quest_board).
- connects known recipes and crafted tools to specific puzzle mechanics: Known Quality Grades (craft_quality).
- uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed: Tournament Rules Desk (fairness_tournament).
- hosts shared roles and parallel clue paths for Guild Vaults: The Guild Chronicle (guild_archive).
### 12 · Incoming system relationships
- Guild Charters (guild_charters) can use this system because it provides the membership and contribution model for Guild Vault challenges.
- Expedition Roles (expedition_roles) can use this system because it routes suitable discoveries into puzzle chambers and structured challenge rooms.
- Dialogue Routes (quest_dialogue) can use this system because it lets quests culminate in structured puzzle or mastery spaces.
- The Cosmetic Gallery (archive_cosmetics) can use this system because it archives relics and challenge records produced by completed Vault rooms.
### 13 · Reward and economy contract
- Reward policy: A validated Vault definition with explicit versions and stable room IDs.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive vault blueprint library view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the vault_blueprints rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for vault blueprint library: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It makes challenge content testable and prevents a Vault from becoming a hidden random-reward box.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# vault_chambers — Puzzle Chambers
> Node 2 of 10 in **Vaults, Dungeons and Puzzle Spaces** · World Loom identity `vault_chambers` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A chamber contains one focused logic, spatial, word or deduction task with a verified solution path.
- Why this belongs in Moonbyte: It gives the Vault a reusable unit of meaningful gameplay.
- Distinctive interaction: Enter a chamber, inspect clues, submit a legal action, receive grounded feedback and open the next state when solved.
- Successful completion produces: A solved chamber state and optional explanation.
### 02 · Purpose and boundaries
- Product role: Vaults are designed challenge spaces with disclosed rules, fixed rewards, saved checkpoints and no paid or chance-based loot-box behaviour.
- This system is the canonical owner of puzzle chambers behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Vaults, Dungeons and Puzzle Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It gives the Vault a reusable unit of meaningful gameplay.
- Stable destination ID: vault_chambers; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .vault chambers and /vault chambers are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Enter a chamber and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to puzzle chambers.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A chamber contains one focused logic, spatial, word or deduction task with a verified solution path.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Enter a chamber, inspect clues, submit a legal action, receive grounded feedback and open the next state when solved.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Puzzle Chambers is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A solved chamber state and optional explanation.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides authored quest objectives and narrative context for each challenge: Chapter Arcs (quest_arcs).
- connects known recipes and crafted tools to specific puzzle mechanics: Commission Bench (craft_commissions).
- uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed: Leaderboard Integrity (fairness_leaderboard).
- hosts shared roles and parallel clue paths for Guild Vaults: Guild Charters (guild_charters).
### 12 · Incoming system relationships
- The Guild Hall (guild_hall) can use this system because it provides the membership and contribution model for Guild Vault challenges.
- Prepared Supply Kit (expedition_supplies) can use this system because it routes suitable discoveries into puzzle chambers and structured challenge rooms.
- Faction Storylines (quest_factions) can use this system because it lets quests culminate in structured puzzle or mastery spaces.
- The Living Lore Index (archive_lore) can use this system because it archives relics and challenge records produced by completed Vault rooms.
### 13 · Reward and economy contract
- Reward policy: A solved chamber state and optional explanation.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive puzzle chambers view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the vault_chambers rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for puzzle chambers: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It gives the Vault a reusable unit of meaningful gameplay.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# vault_logic_keys — Logic Key Mechanisms
> Node 3 of 10 in **Vaults, Dungeons and Puzzle Spaces** · World Loom identity `vault_logic_keys` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Keys are earned by solving visible sub-puzzles and open specific doors whose requirements are disclosed in advance.
- Why this belongs in Moonbyte: It connects rooms without introducing consumable chance keys or surprise access costs.
- Distinctive interaction: Inspect the door, identify its linked puzzle, solve the condition and use the persistent unlock.
- Successful completion produces: A deterministic access flag tied to the Vault blueprint.
### 02 · Purpose and boundaries
- Product role: Vaults are designed challenge spaces with disclosed rules, fixed rewards, saved checkpoints and no paid or chance-based loot-box behaviour.
- This system is the canonical owner of logic key mechanisms behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Vaults, Dungeons and Puzzle Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It connects rooms without introducing consumable chance keys or surprise access costs.
- Stable destination ID: vault_logic_keys; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .vault logic keys and /vault logic keys are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect the door and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to logic key mechanisms.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Keys are earned by solving visible sub-puzzles and open specific doors whose requirements are disclosed in advance.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect the door, identify its linked puzzle, solve the condition and use the persistent unlock.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Logic Key Mechanisms is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A deterministic access flag tied to the Vault blueprint.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides authored quest objectives and narrative context for each challenge: Choice and Consequence Graph (quest_branches).
- connects known recipes and crafted tools to specific puzzle mechanics: Material Reclaim (craft_recycling).
- uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed: Equal Conditions Profiles (fairness_access).
- hosts shared roles and parallel clue paths for Guild Vaults: The Guild Hall (guild_hall).
### 12 · Incoming system relationships
- Guild Role Charter (guild_roles) can use this system because it provides the membership and contribution model for Guild Vault challenges.
- Waystation Checkpoints (expedition_camps) can use this system because it routes suitable discoveries into puzzle chambers and structured challenge rooms.
- Local Tales (quest_side_stories) can use this system because it lets quests culminate in structured puzzle or mastery spaces.
- Collection Milestones (archive_milestones) can use this system because it archives relics and challenge records produced by completed Vault rooms.
### 13 · Reward and economy contract
- Reward policy: A deterministic access flag tied to the Vault blueprint.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive logic key mechanisms view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the vault_logic_keys rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for logic key mechanisms: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It connects rooms without introducing consumable chance keys or surprise access costs.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# vault_relic_rooms — Relic Galleries
> Node 4 of 10 in **Vaults, Dungeons and Puzzle Spaces** · World Loom identity `vault_relic_rooms` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A relic room presents curated lore artefacts and collection context as a discoverable gallery with exact completion criteria.
- Why this belongs in Moonbyte: It ties Vault exploration to the Museum and story archive.
- Distinctive interaction: Inspect the gallery, solve its access puzzle, read the provenance entry and record the discovery.
- Successful completion produces: A fixed relic discovery and an archive reference.
### 02 · Purpose and boundaries
- Product role: Vaults are designed challenge spaces with disclosed rules, fixed rewards, saved checkpoints and no paid or chance-based loot-box behaviour.
- This system is the canonical owner of relic galleries behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Vaults, Dungeons and Puzzle Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It ties Vault exploration to the Museum and story archive.
- Stable destination ID: vault_relic_rooms; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .vault relic rooms and /vault relic rooms are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect the gallery and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to relic galleries.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A relic room presents curated lore artefacts and collection context as a discoverable gallery with exact completion criteria.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect the gallery, solve its access puzzle, read the provenance entry and record the discovery.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Relic Galleries is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A fixed relic discovery and an archive reference.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides authored quest objectives and narrative context for each challenge: The Investigation Desk (quest_mysteries).
- connects known recipes and crafted tools to specific puzzle mechanics: Workshop Repair (craft_repair).
- uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed: Result Appeal Desk (fairness_appeals).
- hosts shared roles and parallel clue paths for Guild Vaults: Guild Role Charter (guild_roles).
### 12 · Incoming system relationships
- The Great Works Board (guild_projects) can use this system because it provides the membership and contribution model for Guild Vault challenges.
- The Expedition Journal (expedition_journal) can use this system because it routes suitable discoveries into puzzle chambers and structured challenge rooms.
- Shared Story Decisions (quest_coop_story) can use this system because it lets quests culminate in structured puzzle or mastery spaces.
- Collector's Provenance View (archive_provenance) can use this system because it archives relics and challenge records produced by completed Vault rooms.
### 13 · Reward and economy contract
- Reward policy: A fixed relic discovery and an archive reference.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive relic galleries view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the vault_relic_rooms rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for relic galleries: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It ties Vault exploration to the Museum and story archive.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# vault_mastery_trials — Mastery Trial Rooms
> Node 5 of 10 in **Vaults, Dungeons and Puzzle Spaces** · World Loom identity `vault_mastery_trials` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A trial tests a known mini-game or puzzle skill using a published difficulty and scoring rubric.
- Why this belongs in Moonbyte: It links arcade mastery to adventure progress without giving random power rewards.
- Distinctive interaction: Select a difficulty, review rules, complete the bounded trial and inspect the scoring breakdown.
- Successful completion produces: A skill record, personal best and fixed completion recognition.
### 02 · Purpose and boundaries
- Product role: Vaults are designed challenge spaces with disclosed rules, fixed rewards, saved checkpoints and no paid or chance-based loot-box behaviour.
- This system is the canonical owner of mastery trial rooms behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Vaults, Dungeons and Puzzle Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It links arcade mastery to adventure progress without giving random power rewards.
- Stable destination ID: vault_mastery_trials; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .vault mastery trials and /vault mastery trials are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Select a difficulty and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to mastery trial rooms.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A trial tests a known mini-game or puzzle skill using a published difficulty and scoring rubric.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Select a difficulty, review rules, complete the bounded trial and inspect the scoring breakdown.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Mastery Trial Rooms is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A skill record, personal best and fixed completion recognition.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides authored quest objectives and narrative context for each challenge: Dialogue Routes (quest_dialogue).
- connects known recipes and crafted tools to specific puzzle mechanics: Pattern and Set Assembly (craft_sets).
- uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed: Published Rulebooks (fairness_rulebooks).
- hosts shared roles and parallel clue paths for Guild Vaults: The Great Works Board (guild_projects).
### 12 · Incoming system relationships
- Guild Story Contracts (guild_quests) can use this system because it provides the membership and contribution model for Guild Vault challenges.
- Return and Recovery Routes (expedition_recovery) can use this system because it routes suitable discoveries into puzzle chambers and structured challenge rooms.
- The Chapter Chronicle (quest_chronicle) can use this system because it lets quests culminate in structured puzzle or mastery spaces.
- The Grand Collection Index (archive_catalogue) can use this system because it archives relics and challenge records produced by completed Vault rooms.
### 13 · Reward and economy contract
- Reward policy: A skill record, personal best and fixed completion recognition.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive mastery trial rooms view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the vault_mastery_trials rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for mastery trial rooms: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It links arcade mastery to adventure progress without giving random power rewards.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# vault_boss_logic — The Grand Mechanism
> Node 6 of 10 in **Vaults, Dungeons and Puzzle Spaces** · World Loom identity `vault_boss_logic` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A Vault finale is a multi-step logic encounter where the group coordinates switches, clues and turn-based choices.
- Why this belongs in Moonbyte: It delivers a memorable climax without graphic violence or a requirement for frame-perfect action.
- Distinctive interaction: Inspect the mechanism, assign roles, resolve each phase and use checkpoints before the final solution.
- Successful completion produces: A completed finale state and a deterministic result summary.
### 02 · Purpose and boundaries
- Product role: Vaults are designed challenge spaces with disclosed rules, fixed rewards, saved checkpoints and no paid or chance-based loot-box behaviour.
- This system is the canonical owner of the grand mechanism behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Vaults, Dungeons and Puzzle Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It delivers a memorable climax without graphic violence or a requirement for frame-perfect action.
- Stable destination ID: vault_boss_logic; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .vault boss logic and /vault boss logic are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Inspect the mechanism and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to the grand mechanism.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A Vault finale is a multi-step logic encounter where the group coordinates switches, clues and turn-based choices.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Inspect the mechanism, assign roles, resolve each phase and use checkpoints before the final solution.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: The Grand Mechanism is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A completed finale state and a deterministic result summary.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides authored quest objectives and narrative context for each challenge: Faction Storylines (quest_factions).
- connects known recipes and crafted tools to specific puzzle mechanics: Guild Workshop (craft_shared_bench).
- uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed: Commit and Reveal Service (fairness_commit_reveal).
- hosts shared roles and parallel clue paths for Guild Vaults: Guild Story Contracts (guild_quests).
### 12 · Incoming system relationships
- The Guild Vault (guild_vault) can use this system because it provides the membership and contribution model for Guild Vault challenges.
- Explorer Contracts (expedition_contracts) can use this system because it routes suitable discoveries into puzzle chambers and structured challenge rooms.
- Story Remix Paths (quest_remix) can use this system because it lets quests culminate in structured puzzle or mastery spaces.
- The Field Bestiary (archive_bestiary) can use this system because it archives relics and challenge records produced by completed Vault rooms.
### 13 · Reward and economy contract
- Reward policy: A completed finale state and a deterministic result summary.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive the grand mechanism view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the vault_boss_logic rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for the grand mechanism: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It delivers a memorable climax without graphic violence or a requirement for frame-perfect action.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# vault_parallel_rooms — Split-Path Chambers
> Node 7 of 10 in **Vaults, Dungeons and Puzzle Spaces** · World Loom identity `vault_parallel_rooms` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Different participants receive separate clues or mechanisms that must be combined to open a shared route.
- Why this belongs in Moonbyte: It creates meaningful communication and makes each participant useful.
- Distinctive interaction: Distribute roles, inspect individual clues privately, share permitted facts and commit a combined solution.
- Successful completion produces: A party state containing private clue assignments and shared conclusions.
### 02 · Purpose and boundaries
- Product role: Vaults are designed challenge spaces with disclosed rules, fixed rewards, saved checkpoints and no paid or chance-based loot-box behaviour.
- This system is the canonical owner of split-path chambers behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Vaults, Dungeons and Puzzle Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It creates meaningful communication and makes each participant useful.
- Stable destination ID: vault_parallel_rooms; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .vault parallel rooms and /vault parallel rooms are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Distribute roles and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to split-path chambers.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Different participants receive separate clues or mechanisms that must be combined to open a shared route.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Distribute roles, inspect individual clues privately, share permitted facts and commit a combined solution.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Split-Path Chambers is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A party state containing private clue assignments and shared conclusions.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides authored quest objectives and narrative context for each challenge: Local Tales (quest_side_stories).
- connects known recipes and crafted tools to specific puzzle mechanics: Recipe Research (craft_discovery).
- uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed: Scoring Contract Registry (fairness_scoring).
- hosts shared roles and parallel clue paths for Guild Vaults: The Guild Vault (guild_vault).
### 12 · Incoming system relationships
- The Guild Cartography Room (guild_map) can use this system because it provides the membership and contribution model for Guild Vault challenges.
- Discovery Scorecard (expedition_score) can use this system because it routes suitable discoveries into puzzle chambers and structured challenge rooms.
- The Quest Board (quest_board) can use this system because it lets quests culminate in structured puzzle or mastery spaces.
- The Relic Archive (archive_relics) can use this system because it archives relics and challenge records produced by completed Vault rooms.
### 13 · Reward and economy contract
- Reward policy: A party state containing private clue assignments and shared conclusions.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive split-path chambers view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the vault_parallel_rooms rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for split-path chambers: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It creates meaningful communication and makes each participant useful.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# vault_checkpoints — Vault Save Stones
> Node 8 of 10 in **Vaults, Dungeons and Puzzle Spaces** · World Loom identity `vault_checkpoints` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Explicit save stones record completed rooms, chosen routes and earned discoveries so a run can be resumed later.
- Why this belongs in Moonbyte: It respects interrupted sessions and prevents progress loss from Discord disconnections.
- Distinctive interaction: Reach a stone, inspect saved progress, pause or leave, then resume with the same blueprint version.
- Successful completion produces: A recoverable checkpoint and idempotent room-completion history.
### 02 · Purpose and boundaries
- Product role: Vaults are designed challenge spaces with disclosed rules, fixed rewards, saved checkpoints and no paid or chance-based loot-box behaviour.
- This system is the canonical owner of vault save stones behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Vaults, Dungeons and Puzzle Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It respects interrupted sessions and prevents progress loss from Discord disconnections.
- Stable destination ID: vault_checkpoints; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .vault checkpoints and /vault checkpoints are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Reach a stone and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to vault save stones.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Explicit save stones record completed rooms, chosen routes and earned discoveries so a run can be resumed later.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Reach a stone, inspect saved progress, pause or leave, then resume with the same blueprint version.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Vault Save Stones is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A recoverable checkpoint and idempotent room-completion history.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides authored quest objectives and narrative context for each challenge: Shared Story Decisions (quest_coop_story).
- connects known recipes and crafted tools to specific puzzle mechanics: Field Gathering (craft_gathering).
- uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed: Optional Skill Rating (fairness_ratings).
- hosts shared roles and parallel clue paths for Guild Vaults: The Guild Cartography Room (guild_map).
### 12 · Incoming system relationships
- Guild Neighbourhoods (guild_alliances) can use this system because it provides the membership and contribution model for Guild Vault challenges.
- Expedition Route Archive (expedition_replay) can use this system because it routes suitable discoveries into puzzle chambers and structured challenge rooms.
- Chapter Arcs (quest_arcs) can use this system because it lets quests culminate in structured puzzle or mastery spaces.
- The Illustrated Compendium (archive_cards) can use this system because it archives relics and challenge records produced by completed Vault rooms.
### 13 · Reward and economy contract
- Reward policy: A recoverable checkpoint and idempotent room-completion history.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive vault save stones view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the vault_checkpoints rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for vault save stones: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It respects interrupted sessions and prevents progress loss from Discord disconnections.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# vault_difficulty — Fair Difficulty Profiles
> Node 9 of 10 in **Vaults, Dungeons and Puzzle Spaces** · World Loom identity `vault_difficulty` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: Difficulty profiles adjust clue density, allowed hints and puzzle complexity using documented, testable rules.
- Why this belongs in Moonbyte: It supports different player skills without hidden adaptive difficulty or pressure.
- Distinctive interaction: Choose a profile, preview its changes, start a clean run and switch only between runs.
- Successful completion produces: A named rules profile and comparable score context.
### 02 · Purpose and boundaries
- Product role: Vaults are designed challenge spaces with disclosed rules, fixed rewards, saved checkpoints and no paid or chance-based loot-box behaviour.
- This system is the canonical owner of fair difficulty profiles behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Vaults, Dungeons and Puzzle Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It supports different player skills without hidden adaptive difficulty or pressure.
- Stable destination ID: vault_difficulty; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .vault difficulty and /vault difficulty are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Choose a profile and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to fair difficulty profiles.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: Difficulty profiles adjust clue density, allowed hints and puzzle complexity using documented, testable rules.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Choose a profile, preview its changes, start a clean run and switch only between runs.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Fair Difficulty Profiles is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A named rules profile and comparable score context.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides authored quest objectives and narrative context for each challenge: The Chapter Chronicle (quest_chronicle).
- connects known recipes and crafted tools to specific puzzle mechanics: The Recipe Codex (craft_recipe_book).
- uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed: Match Review Transcript (fairness_match_review).
- hosts shared roles and parallel clue paths for Guild Vaults: Guild Neighbourhoods (guild_alliances).
### 12 · Incoming system relationships
- Guild Trust Record (guild_reputation) can use this system because it provides the membership and contribution model for Guild Vault challenges.
- Expedition Planner (expedition_planner) can use this system because it routes suitable discoveries into puzzle chambers and structured challenge rooms.
- Choice and Consequence Graph (quest_branches) can use this system because it lets quests culminate in structured puzzle or mastery spaces.
- The Museum Hall (archive_museum) can use this system because it archives relics and challenge records produced by completed Vault rooms.
### 13 · Reward and economy contract
- Reward policy: A named rules profile and comparable score context.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive fair difficulty profiles view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the vault_difficulty rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for fair difficulty profiles: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It supports different player skills without hidden adaptive difficulty or pressure.; it must remain satisfying in a bounded session and support a natural stopping point.
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

# vault_solution_registry — Verified Puzzle Registry
> Node 10 of 10 in **Vaults, Dungeons and Puzzle Spaces** · World Loom identity `vault_solution_registry` · Design status: proposed, not implemented.
### 01 · Player promise
- Player-facing promise: A registry stores known solutions, validation results, duplicate hashes and editorial approvals before a puzzle can be published.
- Why this belongs in Moonbyte: It prevents unwinnable, ambiguous or accidentally duplicated challenge content.
- Distinctive interaction: Submit a puzzle, run validators, review the solution count, approve a version and publish it to a Vault.
- Successful completion produces: A validated puzzle asset with reproducible tests.
### 02 · Purpose and boundaries
- Product role: Vaults are designed challenge spaces with disclosed rules, fixed rewards, saved checkpoints and no paid or chance-based loot-box behaviour.
- This system is the canonical owner of verified puzzle registry behaviour; other systems call its public contract rather than copying its rules.
- The concept is proposed content, not an implemented command, live database table or promise that every variant will ship.
- It must never become a disguised wager, paid random reward, punitive timer or reason to keep a player online.
### 03 · Place in the world
- Primary home: Vaults, Dungeons and Puzzle Spaces; the node is discoverable from the Tavern Wayfinder and the relevant world or collection index.
- World placement: It prevents unwinnable, ambiguous or accidentally duplicated challenge content.
- Stable destination ID: vault_solution_registry; display wording may evolve without changing saved references.
- Presentation uses original full-colour pixel art, readable labels and an accessible text alternative.
### 04 · Discovery and entry
- Primary entry: a contextual card reached from the Tavern, Atlas, party, guild board or a linked feature.
- Proposed command surface: .vault solution registry and /vault solution registry are naming sketches, not registered commands.
- The entry card explains player count, expected time, rules version, visibility, prerequisites and whether progress is saved.
- Starting requires an explicit player action; browsing a card or receiving a suggestion never silently joins a session.
### 05 · Core player loop
- Step one: Submit a puzzle and read the published terms before committing.
- Step two: choose a legal action, audience or party role appropriate to verified puzzle registry.
- Step three: the Rust rules service validates the action and resolves one canonical state transition.
- Step four: show the result, save the permitted output and offer a clear return, pause or stop route.
### 06 · Rules and resolution
- Authoritative rule: A registry stores known solutions, validation results, duplicate hashes and editorial approvals before a puzzle can be published.
- Every input is checked against player identity, current state, session membership, phase, permissions and guild scope.
- Repeated or stale Discord interactions are idempotent; they cannot duplicate a reward, trade, contribution or story choice.
- Invalid actions return a specific, safe explanation and leave durable state unchanged.
### 07 · State machine and lifecycle
- Shared lifecycle: Draft or Available → Ready → Active → Completed; Paused, Cancelled and Expired are explicit supported exits where relevant.
- Node-specific state records the current phase, definition version, participants, completed objectives and legal next actions.
- Pause and resume restore the exact valid checkpoint; an expired interaction cannot rewind a completed transition.
- Migrations must define how older state versions are read, upgraded or safely closed before content is changed.
### 08 · Player choice and autonomy
- Meaningful choice: Submit a puzzle, run validators, review the solution count, approve a version and publish it to a Vault.
- Optional branches are labelled as optional; declining this activity does not block unrelated core play.
- Players can inspect consequences and known costs before confirmation; irreversible choices receive a clear confirmation step.
- Leave, mute, pause and privacy controls remain visible and are never hidden behind a reward claim.
### 09 · Content and variation model
- Content seed: Verified Puzzle Registry is driven by authored, versioned definitions rather than arbitrary free-form runtime behaviour.
- Allowed variants change named parameters such as difficulty, room order or presentation while preserving a validated rule contract.
- Every public variant has source attribution, content review status, compatibility metadata and a reproducible test fixture.
- A content editor may propose changes, but publication cannot bypass validation, moderation or the Rust engine's allowed schema.
### 10 · Inputs, outputs and ownership
- Expected inputs: player actions, session context, published content definitions and only the minimum player or guild settings needed.
- Primary output: A validated puzzle asset with reproducible tests.
- Every output has a stable source event and owner; inventory, profile, world and guild modules consume it through explicit services.
- The system may not directly change another module's private tables or trust client-supplied ownership and balances.
### 11 · Mind-map connections
- provides authored quest objectives and narrative context for each challenge: Story Remix Paths (quest_remix).
- connects known recipes and crafted tools to specific puzzle mechanics: The Maker's Workshop (craft_workshop).
- uses reproducible transcripts for eligible randomised score-only elements while keeping rewards fixed: Integrity Guardrails (fairness_anti_abuse).
- hosts shared roles and parallel clue paths for Guild Vaults: Guild Trust Record (guild_reputation).
### 12 · Incoming system relationships
- The Guild Chronicle (guild_archive) can use this system because it provides the membership and contribution model for Guild Vault challenges.
- Route Encounters (expedition_route_events) can use this system because it routes suitable discoveries into puzzle chambers and structured challenge rooms.
- The Investigation Desk (quest_mysteries) can use this system because it lets quests culminate in structured puzzle or mastery spaces.
- Collection Sets (archive_sets) can use this system because it archives relics and challenge records produced by completed Vault rooms.
### 13 · Reward and economy contract
- Reward policy: A validated puzzle asset with reproducible tests.; exact eligible rewards and costs are disclosed before confirmation.
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
- A responsive verified puzzle registry view can present richer maps, cards, timelines or editors while keeping a browser fallback where supported.
- The Activity requests authorised view models and submits intent; it never computes trusted outcomes, balances, item ownership or permissions.
- Discord-native controls remain available when the Activity cannot load, and the same state is readable in text.
### 16 · Rust domain ownership
- Rust owns the vault_solution_registry rules, state transitions, action validation, reward eligibility and event production.
- The module should expose a narrow service contract and pure rule functions that can be tested without Discord or a live database.
- Poise and Serenity adapt commands and interactions; they do not bypass application services or write SQL directly.
- Tokio tasks remain bounded and event-driven; no per-frame loop, unbounded queue or global lock is introduced for this feature.
### 17 · SQLite persistence contract
- Persist only durable facts required for verified puzzle registry: stable IDs, owner scope, state version, relevant event references and timestamps.
- Balances and trades use immutable ledger entries; progress claims use unique idempotency keys; session state is versioned.
- All related writes occur in short SQLite transactions; no transaction or lock is held across Discord HTTP calls.
- Repository methods must support export, deletion, guild isolation, cleanup and restore tests for the records this node creates.
### 18 · Privacy, consent and permissions
- Identity is derived from authenticated Discord context and verified by Rust; client-submitted user or guild IDs are never trusted alone.
- Store no raw command text, message history or unnecessary personal profile fields to operate this node.
- Public, party, guild and private visibility are separate explicit scopes; defaults disclose the least information necessary.
- The privacy inventory must name every stored field, purpose, retention period and export or deletion path before release.
### 19 · Healthy engagement and safety
- The system is engaging through It prevents unwinnable, ambiguous or accidentally duplicated challenge content.; it must remain satisfying in a bounded session and support a natural stopping point.
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
