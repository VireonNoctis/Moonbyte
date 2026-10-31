# Volume X — Insights, Localisation, Onboarding and Accessibility

> Moonbyte World Loom · Expansion II · 40 additional proposals · Design proposal only.

<!-- WORLD_LOOM_GRAPH_DATA {"domains":[{"id":"insights","title":"Privacy-Preserving Product Insights","summary":"Operational insights use aggregates and opt-in preferences, not raw message logging or individual behavioural profiles.","links":[["trust","constrained_by","inherits minimisation and retention"],["server","summarises","shows scoped aggregates"],["fairness","measures","checks score consistency"],["platform","rendered_in","shows authorised reports"]],"items":[["insights_event_taxonomy","Minimal Event Taxonomy","A registry lists only event names required for reliability and feature-quality metrics.","Propose an event, state purpose, review privacy and approve schema.","A reviewed event schema."],["insights_aggregation","Aggregate Metrics Pipeline","Allowed events become bounded counts and latency histograms before reporting.","Aggregate, apply cohort thresholds and display summary.","A privacy-safe metric."],["insights_opt_in","Player Insight Preferences","Players opt in to optional product feedback metrics and revoke at any time.","Read purpose, enable category and revoke later.","A consent preference."],["insights_retention","Metrics Retention Controller","Metric classes have explicit retention and automatic cleanup rules.","Inspect rule, run cleanup and report counts only.","A retention policy."],["insights_guild_summary","Guild Activity Summary","Admins see aggregate session counts and reliability signals for their own guild.","Check scope and cohort note, then adjust settings.","A guild aggregate."],["insights_quality_feedback","Feature Quality Feedback","A form asks whether a feature was clear, accessible or reliable without collecting conversations.","Rate a feature, preview privacy notice and submit.","A minimal feedback record."],["insights_fairness_checks","Fairness Distribution Review","Eligible score-only game distributions are checked against published expectations.","Run batch, compare model and review anomalies.","A fairness report."],["insights_latency","Interaction Latency Observatory","Aggregate handler and database timings identify slow commands without per-frame measurement.","Measure, aggregate duration buckets and inspect slow paths.","A latency histogram."],["insights_error_catalogue","Safe Error Catalogue","Internal errors map to stable public reason codes without exposing SQL or secrets.","Classify error, retain correlation ID and render safe message.","A reason-code registry."],["insights_privacy_report","Data Practice Audit Report","Reviews compare stored schema and metrics against the privacy inventory.","Inspect fields, compare purposes and block releases for unresolved risks.","A privacy audit report."]]},{"id":"localisation","title":"Localization, Language and Culturalisation","summary":"The language layer preserves puzzle meaning, rules and age-appropriate wording across locales.","links":[["trust","uses_accessibility_policy_from","shares language-aware labels"],["creator","localises","reviews community content"],["quest","translates_story_for","adapts narrative faithfully"],["arcade","localises_rules_for","keeps instructions unambiguous"]],"items":[["locale_registry","Locale Registry","A registry defines locales, fallback chains, text direction and review coverage.","Choose locale, inspect coverage, select fallback and save.","A locale configuration."],["locale_string_catalogue","String Catalogue","Interface strings use stable keys, descriptions, placeholders and context notes.","Add source, document context, translate, validate placeholders and release.","A versioned string catalogue."],["locale_plural_rules","Plural and Number Rules","Formatting handles plural categories, numbers, dates and quantities by locale.","Render a value, validate it and reuse formatter on all surfaces.","A locale-aware result."],["locale_puzzle_semantics","Puzzle Translation Review","Puzzle notes ensure translated clues preserve logic or use an approved equivalent.","Translate, test solution independently and approve locale version.","A reviewed puzzle translation."],["locale_story_adaptation","Story Culturalisation Notes","Editors adapt idioms and context without changing canonical choices.","Propose adaptation, compare branch semantics and approve notes.","A story adaptation record."],["locale_glossary","Translator Glossary","A shared glossary fixes world terms, items, factions and puzzle vocabulary.","Search a term, inspect variants and link reviewed translation to keys.","A term dictionary."],["locale_rtl_layout","Right-to-Left Layout Support","Cards, boards and menus support RTL locales with correct punctuation and order.","Select locale, inspect screens, validate and approve layout.","A directional layout profile."],["locale_voice_tone","Character Voice Guides","Dialogue packs define tone and register per locale.","Review guide, validate scene samples and approve strings.","A localisation guide."],["locale_moderation","Language-Aware Content Review","Moderation reviews localised content with locale context.","Submit, tag locale, route to reviewer and record result.","A locale moderation result."],["locale_coverage","Translation Coverage Dashboard","Reports show missing, draft, reviewed and outdated strings per surface.","Inspect release coverage and block missing critical text.","A coverage report."]]},{"id":"onboarding","title":"New Player Onboarding and Return Journeys","summary":"Onboarding makes Moonbyte approachable without forced tutorials or lengthy setup. Players can explore and return after absence.","links":[["hearth","starts_at","uses Tavern arrival and Wayfinder"],["arcade","demonstrates","offers a first game"],["mastery","continues_into","shows learnable milestones"],["trust","explains","surfaces privacy and accessibility"]],"items":[["onboarding_arrival","First Arrival Scene","A card explains Moonbyte, how to open games and how to leave, with an optional guided route.","Read the overview, choose a tour or skip to Tavern.","A local onboarding preference."],["onboarding_quick_play","One-Tap Sample Game","A short disposable game teaches interactions without economy rewards or rankings.","Preview rules, play one round and return.","A sample completion event."],["onboarding_world_map","World Map Introduction","A guided map highlights Tavern, Arcade, Atlas, Vault and Bazaar connections.","Inspect nodes, follow a connection and finish or leave.","A viewed-tour flag."],["onboarding_privacy_choice","Privacy Setup Checkpoint","Profile setup explains required identifiers, optional visibility and deletion controls.","Review data, choose defaults and skip optional sharing.","A privacy preference."],["onboarding_controls","Control Familiarisation","A panel demonstrates buttons, menus, modals, slash commands and text fallbacks.","Test a control, read equivalent command and close.","A help preference."],["onboarding_first_quest","First Story Invitation","An optional mini-story introduces one NPC, destination and puzzle.","Meet guide, follow clue, solve and save story record.","A first quest completion."],["onboarding_interest_shelf","Interest-Based Discovery Shelf","Players select broad game categories to receive catalogue links without tracking.","Choose interests, inspect shelf and clear selections later.","An explicit preference set."],["onboarding_return_recap","Welcome-Back Recap","A recap from story flags and checkpoints helps players return without guilt language.","Inspect milestones, choose a destination or dismiss.","A deterministic recap."],["onboarding_command_guide","Command Discovery Guide","A guide lists only commands available in the installed build, separate from planned ideas.","Search, inspect usage and read implementation status.","A live command help view."],["onboarding_progress_check","Getting Started Checklist","An optional checklist covers profile basics, first game and privacy controls without streaks.","Complete chosen items, dismiss others or clear it.","Optional onboarding flags."]]},{"id":"accessibility_plus","title":"Advanced Accessibility and Interaction Design","summary":"Accessibility applies to every surface with explicit navigation, timing and comprehension patterns beyond theme toggles.","links":[["trust","extends","uses shared accessibility foundations"],["arcade","applies_to","defines accessible game modes"],["platform","implemented_on","unifies client controls"],["creator","validated_by","checks community content readability"]],"items":[["a11y_keyboard_map","Keyboard and Focus Map","The visual library defines predictable focus order, visible focus and shortcuts for board actions.","Navigate a view by keyboard and submit a standard action.","A tested focus map."],["a11y_screen_reader","Screen Reader Board Contract","Boards expose semantic cell labels, phase, legal actions and result changes.","Read board summary, navigate controls and hear state updates.","A semantic view model."],["a11y_text_density","Readable Density Controls","Players choose compact, standard or spacious text layouts.","Preview densities, choose one and retain globally.","A text-density preference."],["a11y_timing_profiles","Extended Thinking Profiles","Turn-based games offer equivalent extended or untimed modes where rules permit.","Inspect compatibility, choose profile and review score comparison rules.","A timing mode attached to results."],["a11y_input_alternatives","Alternative Input Patterns","Actions may use buttons, numbered choices or command syntax where supported.","Choose an input style and submit a standard validated action.","An input adapter preference."],["a11y_cognitive_scaffolds","Optional Cognitive Scaffolds","Optional step summaries, clearer instructions and glossary hints aid comprehension.","Choose support level, use it and disable when desired.","A support profile."],["a11y_colour_patterns","Colour and Pattern Redundancy","Critical states use labels, shapes and patterns as well as colour.","Switch palette and confirm meaning remains clear.","A visual state specification."],["a11y_motion_budget","Motion and Animation Budget","Animated components have documented reduced-motion alternatives and informative static states.","Preview animation, enable reduced motion and verify equivalent information.","A motion profile."],["a11y_error_language","Actionable Error Language","Errors say what happened, next steps and whether an action committed.","Trigger a safe invalid example and follow recovery guidance.","A public-safe error contract."],["a11y_content_readability","Content Readability Review","Checks review headings, sentence complexity, contrast, alt text and spoiler warnings.","Run review, revise flags and record approval.","A readability report."]]}]} -->

Each proposal defines its mechanic, player loop, cross-domain connections, Rust ownership, persistence, privacy, Discord and Activity behaviour, accessibility, testing and release gates.

## Privacy-Preserving Product Insights (`insights`)

Operational insights use aggregates and opt-in preferences, not raw message logging or individual behavioural profiles.

**Domain connections:** trust via constrained_by — inherits minimisation and retention; server via summarises — shows scoped aggregates; fairness via measures — checks score consistency; platform via rendered_in — shows authorised reports.

# insights_event_taxonomy — Minimal Event Taxonomy
> Expansion II · Privacy-Preserving Product Insights · Proposal 1 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A registry lists only event names required for reliability and feature-quality metrics.
- Player need: Propose an event, state purpose, review privacy and approve schema.
- Why distinct: Minimal Event Taxonomy owns a specific new mechanic and does not replace an existing domain.
- Successful output: A reviewed event schema.
### 02 · Product boundary
- Operational insights use aggregates and opt-in preferences, not raw message logging or individual behavioural profiles.
- Owner: Privacy-Preserving Product Insights (`insights`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `insights_event_taxonomy`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Propose an event, state purpose, review privacy and approve schema.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A registry lists only event names required for reliability and feature-quality metrics.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits minimisation and retention.
- Relationship type: `constrained_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to server: shows scoped aggregates.
- Relationship type: `summarises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: checks score consistency.
- Relationship type: `measures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to platform: shows authorised reports.
- Relationship type: `rendered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A reviewed event schema.
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

# insights_aggregation — Aggregate Metrics Pipeline
> Expansion II · Privacy-Preserving Product Insights · Proposal 2 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Allowed events become bounded counts and latency histograms before reporting.
- Player need: Aggregate, apply cohort thresholds and display summary.
- Why distinct: Aggregate Metrics Pipeline owns a specific new mechanic and does not replace an existing domain.
- Successful output: A privacy-safe metric.
### 02 · Product boundary
- Operational insights use aggregates and opt-in preferences, not raw message logging or individual behavioural profiles.
- Owner: Privacy-Preserving Product Insights (`insights`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `insights_aggregation`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Aggregate, apply cohort thresholds and display summary.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Allowed events become bounded counts and latency histograms before reporting.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits minimisation and retention.
- Relationship type: `constrained_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to server: shows scoped aggregates.
- Relationship type: `summarises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: checks score consistency.
- Relationship type: `measures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to platform: shows authorised reports.
- Relationship type: `rendered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A privacy-safe metric.
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

# insights_opt_in — Player Insight Preferences
> Expansion II · Privacy-Preserving Product Insights · Proposal 3 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Players opt in to optional product feedback metrics and revoke at any time.
- Player need: Read purpose, enable category and revoke later.
- Why distinct: Player Insight Preferences owns a specific new mechanic and does not replace an existing domain.
- Successful output: A consent preference.
### 02 · Product boundary
- Operational insights use aggregates and opt-in preferences, not raw message logging or individual behavioural profiles.
- Owner: Privacy-Preserving Product Insights (`insights`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `insights_opt_in`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Read purpose, enable category and revoke later.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Players opt in to optional product feedback metrics and revoke at any time.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits minimisation and retention.
- Relationship type: `constrained_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to server: shows scoped aggregates.
- Relationship type: `summarises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: checks score consistency.
- Relationship type: `measures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to platform: shows authorised reports.
- Relationship type: `rendered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A consent preference.
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

# insights_retention — Metrics Retention Controller
> Expansion II · Privacy-Preserving Product Insights · Proposal 4 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Metric classes have explicit retention and automatic cleanup rules.
- Player need: Inspect rule, run cleanup and report counts only.
- Why distinct: Metrics Retention Controller owns a specific new mechanic and does not replace an existing domain.
- Successful output: A retention policy.
### 02 · Product boundary
- Operational insights use aggregates and opt-in preferences, not raw message logging or individual behavioural profiles.
- Owner: Privacy-Preserving Product Insights (`insights`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `insights_retention`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect rule, run cleanup and report counts only.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Metric classes have explicit retention and automatic cleanup rules.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits minimisation and retention.
- Relationship type: `constrained_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to server: shows scoped aggregates.
- Relationship type: `summarises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: checks score consistency.
- Relationship type: `measures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to platform: shows authorised reports.
- Relationship type: `rendered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A retention policy.
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

# insights_guild_summary — Guild Activity Summary
> Expansion II · Privacy-Preserving Product Insights · Proposal 5 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Admins see aggregate session counts and reliability signals for their own guild.
- Player need: Check scope and cohort note, then adjust settings.
- Why distinct: Guild Activity Summary owns a specific new mechanic and does not replace an existing domain.
- Successful output: A guild aggregate.
### 02 · Product boundary
- Operational insights use aggregates and opt-in preferences, not raw message logging or individual behavioural profiles.
- Owner: Privacy-Preserving Product Insights (`insights`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `insights_guild_summary`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Check scope and cohort note, then adjust settings.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Admins see aggregate session counts and reliability signals for their own guild.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits minimisation and retention.
- Relationship type: `constrained_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to server: shows scoped aggregates.
- Relationship type: `summarises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: checks score consistency.
- Relationship type: `measures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to platform: shows authorised reports.
- Relationship type: `rendered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A guild aggregate.
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

# insights_quality_feedback — Feature Quality Feedback
> Expansion II · Privacy-Preserving Product Insights · Proposal 6 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A form asks whether a feature was clear, accessible or reliable without collecting conversations.
- Player need: Rate a feature, preview privacy notice and submit.
- Why distinct: Feature Quality Feedback owns a specific new mechanic and does not replace an existing domain.
- Successful output: A minimal feedback record.
### 02 · Product boundary
- Operational insights use aggregates and opt-in preferences, not raw message logging or individual behavioural profiles.
- Owner: Privacy-Preserving Product Insights (`insights`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `insights_quality_feedback`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Rate a feature, preview privacy notice and submit.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A form asks whether a feature was clear, accessible or reliable without collecting conversations.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits minimisation and retention.
- Relationship type: `constrained_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to server: shows scoped aggregates.
- Relationship type: `summarises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: checks score consistency.
- Relationship type: `measures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to platform: shows authorised reports.
- Relationship type: `rendered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A minimal feedback record.
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

# insights_fairness_checks — Fairness Distribution Review
> Expansion II · Privacy-Preserving Product Insights · Proposal 7 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Eligible score-only game distributions are checked against published expectations.
- Player need: Run batch, compare model and review anomalies.
- Why distinct: Fairness Distribution Review owns a specific new mechanic and does not replace an existing domain.
- Successful output: A fairness report.
### 02 · Product boundary
- Operational insights use aggregates and opt-in preferences, not raw message logging or individual behavioural profiles.
- Owner: Privacy-Preserving Product Insights (`insights`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `insights_fairness_checks`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Run batch, compare model and review anomalies.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Eligible score-only game distributions are checked against published expectations.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits minimisation and retention.
- Relationship type: `constrained_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to server: shows scoped aggregates.
- Relationship type: `summarises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: checks score consistency.
- Relationship type: `measures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to platform: shows authorised reports.
- Relationship type: `rendered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A fairness report.
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

# insights_latency — Interaction Latency Observatory
> Expansion II · Privacy-Preserving Product Insights · Proposal 8 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Aggregate handler and database timings identify slow commands without per-frame measurement.
- Player need: Measure, aggregate duration buckets and inspect slow paths.
- Why distinct: Interaction Latency Observatory owns a specific new mechanic and does not replace an existing domain.
- Successful output: A latency histogram.
### 02 · Product boundary
- Operational insights use aggregates and opt-in preferences, not raw message logging or individual behavioural profiles.
- Owner: Privacy-Preserving Product Insights (`insights`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `insights_latency`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Measure, aggregate duration buckets and inspect slow paths.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Aggregate handler and database timings identify slow commands without per-frame measurement.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits minimisation and retention.
- Relationship type: `constrained_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to server: shows scoped aggregates.
- Relationship type: `summarises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: checks score consistency.
- Relationship type: `measures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to platform: shows authorised reports.
- Relationship type: `rendered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A latency histogram.
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

# insights_error_catalogue — Safe Error Catalogue
> Expansion II · Privacy-Preserving Product Insights · Proposal 9 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Internal errors map to stable public reason codes without exposing SQL or secrets.
- Player need: Classify error, retain correlation ID and render safe message.
- Why distinct: Safe Error Catalogue owns a specific new mechanic and does not replace an existing domain.
- Successful output: A reason-code registry.
### 02 · Product boundary
- Operational insights use aggregates and opt-in preferences, not raw message logging or individual behavioural profiles.
- Owner: Privacy-Preserving Product Insights (`insights`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `insights_error_catalogue`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Classify error, retain correlation ID and render safe message.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Internal errors map to stable public reason codes without exposing SQL or secrets.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits minimisation and retention.
- Relationship type: `constrained_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to server: shows scoped aggregates.
- Relationship type: `summarises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: checks score consistency.
- Relationship type: `measures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to platform: shows authorised reports.
- Relationship type: `rendered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A reason-code registry.
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

# insights_privacy_report — Data Practice Audit Report
> Expansion II · Privacy-Preserving Product Insights · Proposal 10 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Reviews compare stored schema and metrics against the privacy inventory.
- Player need: Inspect fields, compare purposes and block releases for unresolved risks.
- Why distinct: Data Practice Audit Report owns a specific new mechanic and does not replace an existing domain.
- Successful output: A privacy audit report.
### 02 · Product boundary
- Operational insights use aggregates and opt-in preferences, not raw message logging or individual behavioural profiles.
- Owner: Privacy-Preserving Product Insights (`insights`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `insights_privacy_report`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect fields, compare purposes and block releases for unresolved risks.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Reviews compare stored schema and metrics against the privacy inventory.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: inherits minimisation and retention.
- Relationship type: `constrained_by`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to server: shows scoped aggregates.
- Relationship type: `summarises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to fairness: checks score consistency.
- Relationship type: `measures`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to platform: shows authorised reports.
- Relationship type: `rendered_in`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A privacy audit report.
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
## Localization, Language and Culturalisation (`localisation`)

The language layer preserves puzzle meaning, rules and age-appropriate wording across locales.

**Domain connections:** trust via uses_accessibility_policy_from — shares language-aware labels; creator via localises — reviews community content; quest via translates_story_for — adapts narrative faithfully; arcade via localises_rules_for — keeps instructions unambiguous.

# locale_registry — Locale Registry
> Expansion II · Localization, Language and Culturalisation · Proposal 1 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A registry defines locales, fallback chains, text direction and review coverage.
- Player need: Choose locale, inspect coverage, select fallback and save.
- Why distinct: Locale Registry owns a specific new mechanic and does not replace an existing domain.
- Successful output: A locale configuration.
### 02 · Product boundary
- The language layer preserves puzzle meaning, rules and age-appropriate wording across locales.
- Owner: Localization, Language and Culturalisation (`localisation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `locale_registry`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Choose locale, inspect coverage, select fallback and save.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A registry defines locales, fallback chains, text direction and review coverage.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: shares language-aware labels.
- Relationship type: `uses_accessibility_policy_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reviews community content.
- Relationship type: `localises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to quest: adapts narrative faithfully.
- Relationship type: `translates_story_for`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: keeps instructions unambiguous.
- Relationship type: `localises_rules_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A locale configuration.
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

# locale_string_catalogue — String Catalogue
> Expansion II · Localization, Language and Culturalisation · Proposal 2 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Interface strings use stable keys, descriptions, placeholders and context notes.
- Player need: Add source, document context, translate, validate placeholders and release.
- Why distinct: String Catalogue owns a specific new mechanic and does not replace an existing domain.
- Successful output: A versioned string catalogue.
### 02 · Product boundary
- The language layer preserves puzzle meaning, rules and age-appropriate wording across locales.
- Owner: Localization, Language and Culturalisation (`localisation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `locale_string_catalogue`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Add source, document context, translate, validate placeholders and release.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Interface strings use stable keys, descriptions, placeholders and context notes.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: shares language-aware labels.
- Relationship type: `uses_accessibility_policy_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reviews community content.
- Relationship type: `localises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to quest: adapts narrative faithfully.
- Relationship type: `translates_story_for`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: keeps instructions unambiguous.
- Relationship type: `localises_rules_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A versioned string catalogue.
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

# locale_plural_rules — Plural and Number Rules
> Expansion II · Localization, Language and Culturalisation · Proposal 3 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Formatting handles plural categories, numbers, dates and quantities by locale.
- Player need: Render a value, validate it and reuse formatter on all surfaces.
- Why distinct: Plural and Number Rules owns a specific new mechanic and does not replace an existing domain.
- Successful output: A locale-aware result.
### 02 · Product boundary
- The language layer preserves puzzle meaning, rules and age-appropriate wording across locales.
- Owner: Localization, Language and Culturalisation (`localisation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `locale_plural_rules`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Render a value, validate it and reuse formatter on all surfaces.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Formatting handles plural categories, numbers, dates and quantities by locale.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: shares language-aware labels.
- Relationship type: `uses_accessibility_policy_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reviews community content.
- Relationship type: `localises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to quest: adapts narrative faithfully.
- Relationship type: `translates_story_for`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: keeps instructions unambiguous.
- Relationship type: `localises_rules_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A locale-aware result.
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

# locale_puzzle_semantics — Puzzle Translation Review
> Expansion II · Localization, Language and Culturalisation · Proposal 4 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Puzzle notes ensure translated clues preserve logic or use an approved equivalent.
- Player need: Translate, test solution independently and approve locale version.
- Why distinct: Puzzle Translation Review owns a specific new mechanic and does not replace an existing domain.
- Successful output: A reviewed puzzle translation.
### 02 · Product boundary
- The language layer preserves puzzle meaning, rules and age-appropriate wording across locales.
- Owner: Localization, Language and Culturalisation (`localisation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `locale_puzzle_semantics`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Translate, test solution independently and approve locale version.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Puzzle notes ensure translated clues preserve logic or use an approved equivalent.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: shares language-aware labels.
- Relationship type: `uses_accessibility_policy_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reviews community content.
- Relationship type: `localises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to quest: adapts narrative faithfully.
- Relationship type: `translates_story_for`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: keeps instructions unambiguous.
- Relationship type: `localises_rules_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A reviewed puzzle translation.
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

# locale_story_adaptation — Story Culturalisation Notes
> Expansion II · Localization, Language and Culturalisation · Proposal 5 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Editors adapt idioms and context without changing canonical choices.
- Player need: Propose adaptation, compare branch semantics and approve notes.
- Why distinct: Story Culturalisation Notes owns a specific new mechanic and does not replace an existing domain.
- Successful output: A story adaptation record.
### 02 · Product boundary
- The language layer preserves puzzle meaning, rules and age-appropriate wording across locales.
- Owner: Localization, Language and Culturalisation (`localisation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `locale_story_adaptation`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Propose adaptation, compare branch semantics and approve notes.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Editors adapt idioms and context without changing canonical choices.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: shares language-aware labels.
- Relationship type: `uses_accessibility_policy_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reviews community content.
- Relationship type: `localises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to quest: adapts narrative faithfully.
- Relationship type: `translates_story_for`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: keeps instructions unambiguous.
- Relationship type: `localises_rules_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A story adaptation record.
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

# locale_glossary — Translator Glossary
> Expansion II · Localization, Language and Culturalisation · Proposal 6 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A shared glossary fixes world terms, items, factions and puzzle vocabulary.
- Player need: Search a term, inspect variants and link reviewed translation to keys.
- Why distinct: Translator Glossary owns a specific new mechanic and does not replace an existing domain.
- Successful output: A term dictionary.
### 02 · Product boundary
- The language layer preserves puzzle meaning, rules and age-appropriate wording across locales.
- Owner: Localization, Language and Culturalisation (`localisation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `locale_glossary`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Search a term, inspect variants and link reviewed translation to keys.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A shared glossary fixes world terms, items, factions and puzzle vocabulary.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: shares language-aware labels.
- Relationship type: `uses_accessibility_policy_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reviews community content.
- Relationship type: `localises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to quest: adapts narrative faithfully.
- Relationship type: `translates_story_for`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: keeps instructions unambiguous.
- Relationship type: `localises_rules_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A term dictionary.
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

# locale_rtl_layout — Right-to-Left Layout Support
> Expansion II · Localization, Language and Culturalisation · Proposal 7 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Cards, boards and menus support RTL locales with correct punctuation and order.
- Player need: Select locale, inspect screens, validate and approve layout.
- Why distinct: Right-to-Left Layout Support owns a specific new mechanic and does not replace an existing domain.
- Successful output: A directional layout profile.
### 02 · Product boundary
- The language layer preserves puzzle meaning, rules and age-appropriate wording across locales.
- Owner: Localization, Language and Culturalisation (`localisation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `locale_rtl_layout`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Select locale, inspect screens, validate and approve layout.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Cards, boards and menus support RTL locales with correct punctuation and order.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: shares language-aware labels.
- Relationship type: `uses_accessibility_policy_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reviews community content.
- Relationship type: `localises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to quest: adapts narrative faithfully.
- Relationship type: `translates_story_for`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: keeps instructions unambiguous.
- Relationship type: `localises_rules_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A directional layout profile.
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

# locale_voice_tone — Character Voice Guides
> Expansion II · Localization, Language and Culturalisation · Proposal 8 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Dialogue packs define tone and register per locale.
- Player need: Review guide, validate scene samples and approve strings.
- Why distinct: Character Voice Guides owns a specific new mechanic and does not replace an existing domain.
- Successful output: A localisation guide.
### 02 · Product boundary
- The language layer preserves puzzle meaning, rules and age-appropriate wording across locales.
- Owner: Localization, Language and Culturalisation (`localisation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `locale_voice_tone`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Review guide, validate scene samples and approve strings.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Dialogue packs define tone and register per locale.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: shares language-aware labels.
- Relationship type: `uses_accessibility_policy_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reviews community content.
- Relationship type: `localises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to quest: adapts narrative faithfully.
- Relationship type: `translates_story_for`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: keeps instructions unambiguous.
- Relationship type: `localises_rules_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A localisation guide.
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

# locale_moderation — Language-Aware Content Review
> Expansion II · Localization, Language and Culturalisation · Proposal 9 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Moderation reviews localised content with locale context.
- Player need: Submit, tag locale, route to reviewer and record result.
- Why distinct: Language-Aware Content Review owns a specific new mechanic and does not replace an existing domain.
- Successful output: A locale moderation result.
### 02 · Product boundary
- The language layer preserves puzzle meaning, rules and age-appropriate wording across locales.
- Owner: Localization, Language and Culturalisation (`localisation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `locale_moderation`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Submit, tag locale, route to reviewer and record result.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Moderation reviews localised content with locale context.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: shares language-aware labels.
- Relationship type: `uses_accessibility_policy_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reviews community content.
- Relationship type: `localises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to quest: adapts narrative faithfully.
- Relationship type: `translates_story_for`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: keeps instructions unambiguous.
- Relationship type: `localises_rules_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A locale moderation result.
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

# locale_coverage — Translation Coverage Dashboard
> Expansion II · Localization, Language and Culturalisation · Proposal 10 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Reports show missing, draft, reviewed and outdated strings per surface.
- Player need: Inspect release coverage and block missing critical text.
- Why distinct: Translation Coverage Dashboard owns a specific new mechanic and does not replace an existing domain.
- Successful output: A coverage report.
### 02 · Product boundary
- The language layer preserves puzzle meaning, rules and age-appropriate wording across locales.
- Owner: Localization, Language and Culturalisation (`localisation`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `locale_coverage`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect release coverage and block missing critical text.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Reports show missing, draft, reviewed and outdated strings per surface.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: shares language-aware labels.
- Relationship type: `uses_accessibility_policy_from`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to creator: reviews community content.
- Relationship type: `localises`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to quest: adapts narrative faithfully.
- Relationship type: `translates_story_for`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to arcade: keeps instructions unambiguous.
- Relationship type: `localises_rules_for`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A coverage report.
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
## New Player Onboarding and Return Journeys (`onboarding`)

Onboarding makes Moonbyte approachable without forced tutorials or lengthy setup. Players can explore and return after absence.

**Domain connections:** hearth via starts_at — uses Tavern arrival and Wayfinder; arcade via demonstrates — offers a first game; mastery via continues_into — shows learnable milestones; trust via explains — surfaces privacy and accessibility.

# onboarding_arrival — First Arrival Scene
> Expansion II · New Player Onboarding and Return Journeys · Proposal 1 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A card explains Moonbyte, how to open games and how to leave, with an optional guided route.
- Player need: Read the overview, choose a tour or skip to Tavern.
- Why distinct: First Arrival Scene owns a specific new mechanic and does not replace an existing domain.
- Successful output: A local onboarding preference.
### 02 · Product boundary
- Onboarding makes Moonbyte approachable without forced tutorials or lengthy setup. Players can explore and return after absence.
- Owner: New Player Onboarding and Return Journeys (`onboarding`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `onboarding_arrival`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Read the overview, choose a tour or skip to Tavern.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A card explains Moonbyte, how to open games and how to leave, with an optional guided route.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to hearth: uses Tavern arrival and Wayfinder.
- Relationship type: `starts_at`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: offers a first game.
- Relationship type: `demonstrates`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to mastery: shows learnable milestones.
- Relationship type: `continues_into`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: surfaces privacy and accessibility.
- Relationship type: `explains`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A local onboarding preference.
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

# onboarding_quick_play — One-Tap Sample Game
> Expansion II · New Player Onboarding and Return Journeys · Proposal 2 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A short disposable game teaches interactions without economy rewards or rankings.
- Player need: Preview rules, play one round and return.
- Why distinct: One-Tap Sample Game owns a specific new mechanic and does not replace an existing domain.
- Successful output: A sample completion event.
### 02 · Product boundary
- Onboarding makes Moonbyte approachable without forced tutorials or lengthy setup. Players can explore and return after absence.
- Owner: New Player Onboarding and Return Journeys (`onboarding`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `onboarding_quick_play`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Preview rules, play one round and return.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A short disposable game teaches interactions without economy rewards or rankings.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to hearth: uses Tavern arrival and Wayfinder.
- Relationship type: `starts_at`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: offers a first game.
- Relationship type: `demonstrates`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to mastery: shows learnable milestones.
- Relationship type: `continues_into`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: surfaces privacy and accessibility.
- Relationship type: `explains`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A sample completion event.
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

# onboarding_world_map — World Map Introduction
> Expansion II · New Player Onboarding and Return Journeys · Proposal 3 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A guided map highlights Tavern, Arcade, Atlas, Vault and Bazaar connections.
- Player need: Inspect nodes, follow a connection and finish or leave.
- Why distinct: World Map Introduction owns a specific new mechanic and does not replace an existing domain.
- Successful output: A viewed-tour flag.
### 02 · Product boundary
- Onboarding makes Moonbyte approachable without forced tutorials or lengthy setup. Players can explore and return after absence.
- Owner: New Player Onboarding and Return Journeys (`onboarding`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `onboarding_world_map`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect nodes, follow a connection and finish or leave.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A guided map highlights Tavern, Arcade, Atlas, Vault and Bazaar connections.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to hearth: uses Tavern arrival and Wayfinder.
- Relationship type: `starts_at`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: offers a first game.
- Relationship type: `demonstrates`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to mastery: shows learnable milestones.
- Relationship type: `continues_into`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: surfaces privacy and accessibility.
- Relationship type: `explains`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A viewed-tour flag.
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

# onboarding_privacy_choice — Privacy Setup Checkpoint
> Expansion II · New Player Onboarding and Return Journeys · Proposal 4 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Profile setup explains required identifiers, optional visibility and deletion controls.
- Player need: Review data, choose defaults and skip optional sharing.
- Why distinct: Privacy Setup Checkpoint owns a specific new mechanic and does not replace an existing domain.
- Successful output: A privacy preference.
### 02 · Product boundary
- Onboarding makes Moonbyte approachable without forced tutorials or lengthy setup. Players can explore and return after absence.
- Owner: New Player Onboarding and Return Journeys (`onboarding`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `onboarding_privacy_choice`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Review data, choose defaults and skip optional sharing.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Profile setup explains required identifiers, optional visibility and deletion controls.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to hearth: uses Tavern arrival and Wayfinder.
- Relationship type: `starts_at`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: offers a first game.
- Relationship type: `demonstrates`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to mastery: shows learnable milestones.
- Relationship type: `continues_into`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: surfaces privacy and accessibility.
- Relationship type: `explains`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A privacy preference.
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

# onboarding_controls — Control Familiarisation
> Expansion II · New Player Onboarding and Return Journeys · Proposal 5 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A panel demonstrates buttons, menus, modals, slash commands and text fallbacks.
- Player need: Test a control, read equivalent command and close.
- Why distinct: Control Familiarisation owns a specific new mechanic and does not replace an existing domain.
- Successful output: A help preference.
### 02 · Product boundary
- Onboarding makes Moonbyte approachable without forced tutorials or lengthy setup. Players can explore and return after absence.
- Owner: New Player Onboarding and Return Journeys (`onboarding`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `onboarding_controls`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Test a control, read equivalent command and close.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A panel demonstrates buttons, menus, modals, slash commands and text fallbacks.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to hearth: uses Tavern arrival and Wayfinder.
- Relationship type: `starts_at`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: offers a first game.
- Relationship type: `demonstrates`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to mastery: shows learnable milestones.
- Relationship type: `continues_into`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: surfaces privacy and accessibility.
- Relationship type: `explains`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A help preference.
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

# onboarding_first_quest — First Story Invitation
> Expansion II · New Player Onboarding and Return Journeys · Proposal 6 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- An optional mini-story introduces one NPC, destination and puzzle.
- Player need: Meet guide, follow clue, solve and save story record.
- Why distinct: First Story Invitation owns a specific new mechanic and does not replace an existing domain.
- Successful output: A first quest completion.
### 02 · Product boundary
- Onboarding makes Moonbyte approachable without forced tutorials or lengthy setup. Players can explore and return after absence.
- Owner: New Player Onboarding and Return Journeys (`onboarding`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `onboarding_first_quest`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Meet guide, follow clue, solve and save story record.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- An optional mini-story introduces one NPC, destination and puzzle.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to hearth: uses Tavern arrival and Wayfinder.
- Relationship type: `starts_at`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: offers a first game.
- Relationship type: `demonstrates`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to mastery: shows learnable milestones.
- Relationship type: `continues_into`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: surfaces privacy and accessibility.
- Relationship type: `explains`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A first quest completion.
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

# onboarding_interest_shelf — Interest-Based Discovery Shelf
> Expansion II · New Player Onboarding and Return Journeys · Proposal 7 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Players select broad game categories to receive catalogue links without tracking.
- Player need: Choose interests, inspect shelf and clear selections later.
- Why distinct: Interest-Based Discovery Shelf owns a specific new mechanic and does not replace an existing domain.
- Successful output: An explicit preference set.
### 02 · Product boundary
- Onboarding makes Moonbyte approachable without forced tutorials or lengthy setup. Players can explore and return after absence.
- Owner: New Player Onboarding and Return Journeys (`onboarding`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `onboarding_interest_shelf`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Choose interests, inspect shelf and clear selections later.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Players select broad game categories to receive catalogue links without tracking.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to hearth: uses Tavern arrival and Wayfinder.
- Relationship type: `starts_at`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: offers a first game.
- Relationship type: `demonstrates`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to mastery: shows learnable milestones.
- Relationship type: `continues_into`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: surfaces privacy and accessibility.
- Relationship type: `explains`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- An explicit preference set.
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

# onboarding_return_recap — Welcome-Back Recap
> Expansion II · New Player Onboarding and Return Journeys · Proposal 8 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A recap from story flags and checkpoints helps players return without guilt language.
- Player need: Inspect milestones, choose a destination or dismiss.
- Why distinct: Welcome-Back Recap owns a specific new mechanic and does not replace an existing domain.
- Successful output: A deterministic recap.
### 02 · Product boundary
- Onboarding makes Moonbyte approachable without forced tutorials or lengthy setup. Players can explore and return after absence.
- Owner: New Player Onboarding and Return Journeys (`onboarding`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `onboarding_return_recap`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect milestones, choose a destination or dismiss.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A recap from story flags and checkpoints helps players return without guilt language.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to hearth: uses Tavern arrival and Wayfinder.
- Relationship type: `starts_at`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: offers a first game.
- Relationship type: `demonstrates`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to mastery: shows learnable milestones.
- Relationship type: `continues_into`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: surfaces privacy and accessibility.
- Relationship type: `explains`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A deterministic recap.
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

# onboarding_command_guide — Command Discovery Guide
> Expansion II · New Player Onboarding and Return Journeys · Proposal 9 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- A guide lists only commands available in the installed build, separate from planned ideas.
- Player need: Search, inspect usage and read implementation status.
- Why distinct: Command Discovery Guide owns a specific new mechanic and does not replace an existing domain.
- Successful output: A live command help view.
### 02 · Product boundary
- Onboarding makes Moonbyte approachable without forced tutorials or lengthy setup. Players can explore and return after absence.
- Owner: New Player Onboarding and Return Journeys (`onboarding`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `onboarding_command_guide`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Search, inspect usage and read implementation status.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- A guide lists only commands available in the installed build, separate from planned ideas.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to hearth: uses Tavern arrival and Wayfinder.
- Relationship type: `starts_at`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: offers a first game.
- Relationship type: `demonstrates`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to mastery: shows learnable milestones.
- Relationship type: `continues_into`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: surfaces privacy and accessibility.
- Relationship type: `explains`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A live command help view.
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

# onboarding_progress_check — Getting Started Checklist
> Expansion II · New Player Onboarding and Return Journeys · Proposal 10 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- An optional checklist covers profile basics, first game and privacy controls without streaks.
- Player need: Complete chosen items, dismiss others or clear it.
- Why distinct: Getting Started Checklist owns a specific new mechanic and does not replace an existing domain.
- Successful output: Optional onboarding flags.
### 02 · Product boundary
- Onboarding makes Moonbyte approachable without forced tutorials or lengthy setup. Players can explore and return after absence.
- Owner: New Player Onboarding and Return Journeys (`onboarding`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `onboarding_progress_check`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Complete chosen items, dismiss others or clear it.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- An optional checklist covers profile basics, first game and privacy controls without streaks.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to hearth: uses Tavern arrival and Wayfinder.
- Relationship type: `starts_at`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: offers a first game.
- Relationship type: `demonstrates`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to mastery: shows learnable milestones.
- Relationship type: `continues_into`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to trust: surfaces privacy and accessibility.
- Relationship type: `explains`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- Optional onboarding flags.
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
## Advanced Accessibility and Interaction Design (`accessibility_plus`)

Accessibility applies to every surface with explicit navigation, timing and comprehension patterns beyond theme toggles.

**Domain connections:** trust via extends — uses shared accessibility foundations; arcade via applies_to — defines accessible game modes; platform via implemented_on — unifies client controls; creator via validated_by — checks community content readability.

# a11y_keyboard_map — Keyboard and Focus Map
> Expansion II · Advanced Accessibility and Interaction Design · Proposal 1 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- The visual library defines predictable focus order, visible focus and shortcuts for board actions.
- Player need: Navigate a view by keyboard and submit a standard action.
- Why distinct: Keyboard and Focus Map owns a specific new mechanic and does not replace an existing domain.
- Successful output: A tested focus map.
### 02 · Product boundary
- Accessibility applies to every surface with explicit navigation, timing and comprehension patterns beyond theme toggles.
- Owner: Advanced Accessibility and Interaction Design (`accessibility_plus`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `a11y_keyboard_map`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Navigate a view by keyboard and submit a standard action.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- The visual library defines predictable focus order, visible focus and shortcuts for board actions.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses shared accessibility foundations.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: defines accessible game modes.
- Relationship type: `applies_to`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to platform: unifies client controls.
- Relationship type: `implemented_on`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to creator: checks community content readability.
- Relationship type: `validated_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A tested focus map.
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

# a11y_screen_reader — Screen Reader Board Contract
> Expansion II · Advanced Accessibility and Interaction Design · Proposal 2 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Boards expose semantic cell labels, phase, legal actions and result changes.
- Player need: Read board summary, navigate controls and hear state updates.
- Why distinct: Screen Reader Board Contract owns a specific new mechanic and does not replace an existing domain.
- Successful output: A semantic view model.
### 02 · Product boundary
- Accessibility applies to every surface with explicit navigation, timing and comprehension patterns beyond theme toggles.
- Owner: Advanced Accessibility and Interaction Design (`accessibility_plus`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `a11y_screen_reader`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Read board summary, navigate controls and hear state updates.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Boards expose semantic cell labels, phase, legal actions and result changes.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses shared accessibility foundations.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: defines accessible game modes.
- Relationship type: `applies_to`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to platform: unifies client controls.
- Relationship type: `implemented_on`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to creator: checks community content readability.
- Relationship type: `validated_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A semantic view model.
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

# a11y_text_density — Readable Density Controls
> Expansion II · Advanced Accessibility and Interaction Design · Proposal 3 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Players choose compact, standard or spacious text layouts.
- Player need: Preview densities, choose one and retain globally.
- Why distinct: Readable Density Controls owns a specific new mechanic and does not replace an existing domain.
- Successful output: A text-density preference.
### 02 · Product boundary
- Accessibility applies to every surface with explicit navigation, timing and comprehension patterns beyond theme toggles.
- Owner: Advanced Accessibility and Interaction Design (`accessibility_plus`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `a11y_text_density`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Preview densities, choose one and retain globally.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Players choose compact, standard or spacious text layouts.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses shared accessibility foundations.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: defines accessible game modes.
- Relationship type: `applies_to`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to platform: unifies client controls.
- Relationship type: `implemented_on`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to creator: checks community content readability.
- Relationship type: `validated_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A text-density preference.
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

# a11y_timing_profiles — Extended Thinking Profiles
> Expansion II · Advanced Accessibility and Interaction Design · Proposal 4 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Turn-based games offer equivalent extended or untimed modes where rules permit.
- Player need: Inspect compatibility, choose profile and review score comparison rules.
- Why distinct: Extended Thinking Profiles owns a specific new mechanic and does not replace an existing domain.
- Successful output: A timing mode attached to results.
### 02 · Product boundary
- Accessibility applies to every surface with explicit navigation, timing and comprehension patterns beyond theme toggles.
- Owner: Advanced Accessibility and Interaction Design (`accessibility_plus`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `a11y_timing_profiles`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Inspect compatibility, choose profile and review score comparison rules.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Turn-based games offer equivalent extended or untimed modes where rules permit.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses shared accessibility foundations.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: defines accessible game modes.
- Relationship type: `applies_to`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to platform: unifies client controls.
- Relationship type: `implemented_on`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to creator: checks community content readability.
- Relationship type: `validated_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A timing mode attached to results.
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

# a11y_input_alternatives — Alternative Input Patterns
> Expansion II · Advanced Accessibility and Interaction Design · Proposal 5 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Actions may use buttons, numbered choices or command syntax where supported.
- Player need: Choose an input style and submit a standard validated action.
- Why distinct: Alternative Input Patterns owns a specific new mechanic and does not replace an existing domain.
- Successful output: An input adapter preference.
### 02 · Product boundary
- Accessibility applies to every surface with explicit navigation, timing and comprehension patterns beyond theme toggles.
- Owner: Advanced Accessibility and Interaction Design (`accessibility_plus`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `a11y_input_alternatives`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Choose an input style and submit a standard validated action.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Actions may use buttons, numbered choices or command syntax where supported.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses shared accessibility foundations.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: defines accessible game modes.
- Relationship type: `applies_to`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to platform: unifies client controls.
- Relationship type: `implemented_on`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to creator: checks community content readability.
- Relationship type: `validated_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- An input adapter preference.
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

# a11y_cognitive_scaffolds — Optional Cognitive Scaffolds
> Expansion II · Advanced Accessibility and Interaction Design · Proposal 6 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Optional step summaries, clearer instructions and glossary hints aid comprehension.
- Player need: Choose support level, use it and disable when desired.
- Why distinct: Optional Cognitive Scaffolds owns a specific new mechanic and does not replace an existing domain.
- Successful output: A support profile.
### 02 · Product boundary
- Accessibility applies to every surface with explicit navigation, timing and comprehension patterns beyond theme toggles.
- Owner: Advanced Accessibility and Interaction Design (`accessibility_plus`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `a11y_cognitive_scaffolds`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Choose support level, use it and disable when desired.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Optional step summaries, clearer instructions and glossary hints aid comprehension.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses shared accessibility foundations.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: defines accessible game modes.
- Relationship type: `applies_to`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to platform: unifies client controls.
- Relationship type: `implemented_on`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to creator: checks community content readability.
- Relationship type: `validated_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A support profile.
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

# a11y_colour_patterns — Colour and Pattern Redundancy
> Expansion II · Advanced Accessibility and Interaction Design · Proposal 7 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Critical states use labels, shapes and patterns as well as colour.
- Player need: Switch palette and confirm meaning remains clear.
- Why distinct: Colour and Pattern Redundancy owns a specific new mechanic and does not replace an existing domain.
- Successful output: A visual state specification.
### 02 · Product boundary
- Accessibility applies to every surface with explicit navigation, timing and comprehension patterns beyond theme toggles.
- Owner: Advanced Accessibility and Interaction Design (`accessibility_plus`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `a11y_colour_patterns`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Switch palette and confirm meaning remains clear.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Critical states use labels, shapes and patterns as well as colour.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses shared accessibility foundations.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: defines accessible game modes.
- Relationship type: `applies_to`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to platform: unifies client controls.
- Relationship type: `implemented_on`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to creator: checks community content readability.
- Relationship type: `validated_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A visual state specification.
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

# a11y_motion_budget — Motion and Animation Budget
> Expansion II · Advanced Accessibility and Interaction Design · Proposal 8 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Animated components have documented reduced-motion alternatives and informative static states.
- Player need: Preview animation, enable reduced motion and verify equivalent information.
- Why distinct: Motion and Animation Budget owns a specific new mechanic and does not replace an existing domain.
- Successful output: A motion profile.
### 02 · Product boundary
- Accessibility applies to every surface with explicit navigation, timing and comprehension patterns beyond theme toggles.
- Owner: Advanced Accessibility and Interaction Design (`accessibility_plus`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `a11y_motion_budget`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Preview animation, enable reduced motion and verify equivalent information.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Animated components have documented reduced-motion alternatives and informative static states.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses shared accessibility foundations.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: defines accessible game modes.
- Relationship type: `applies_to`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to platform: unifies client controls.
- Relationship type: `implemented_on`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to creator: checks community content readability.
- Relationship type: `validated_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A motion profile.
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

# a11y_error_language — Actionable Error Language
> Expansion II · Advanced Accessibility and Interaction Design · Proposal 9 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Errors say what happened, next steps and whether an action committed.
- Player need: Trigger a safe invalid example and follow recovery guidance.
- Why distinct: Actionable Error Language owns a specific new mechanic and does not replace an existing domain.
- Successful output: A public-safe error contract.
### 02 · Product boundary
- Accessibility applies to every surface with explicit navigation, timing and comprehension patterns beyond theme toggles.
- Owner: Advanced Accessibility and Interaction Design (`accessibility_plus`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `a11y_error_language`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Trigger a safe invalid example and follow recovery guidance.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Errors say what happened, next steps and whether an action committed.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses shared accessibility foundations.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: defines accessible game modes.
- Relationship type: `applies_to`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to platform: unifies client controls.
- Relationship type: `implemented_on`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to creator: checks community content readability.
- Relationship type: `validated_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A public-safe error contract.
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

# a11y_content_readability — Content Readability Review
> Expansion II · Advanced Accessibility and Interaction Design · Proposal 10 of 10 · Status: design proposal, not implemented.
### 01 · Player promise
- Checks review headings, sentence complexity, contrast, alt text and spoiler warnings.
- Player need: Run review, revise flags and record approval.
- Why distinct: Content Readability Review owns a specific new mechanic and does not replace an existing domain.
- Successful output: A readability report.
### 02 · Product boundary
- Accessibility applies to every surface with explicit navigation, timing and comprehension patterns beyond theme toggles.
- Owner: Advanced Accessibility and Interaction Design (`accessibility_plus`).
- This is a proposal, not an implemented command, service or database table.
- It becomes real only through a tested vertical slice and an accurate implementation-status update.
### 03 · Discovery and entry
- Tavern Wayfinder is the default discovery surface, with contextual entrances from Atlas, Archive, guild and relevant service views.
- Stable identity: `a11y_content_readability`; display labels and command names may change without breaking stored references.
- The entry card shows participant scope, expected duration, visibility, prerequisites, rules version, costs and exact outputs.
- A link or notice never silently joins a session or commits an action.
### 04 · Player journey
- Run review, revise flags and record approval.
- The player inspects the current state and chooses a supported action, route or role.
- Rust validates the request and emits one canonical domain event.
- The result is summarised and followed by a visible return, pause or leave action.
### 05 · Mechanics
- Checks review headings, sentence complexity, contrast, alt text and spoiler warnings.
- Inputs are typed and validated; free text is used only for intentional authored material.
- Outcomes use published rules and deterministic transformations wherever possible.
- Variants are versioned and cannot silently invalidate completed records.
### 06 · Meaningful choices
- The player sees the purpose and known consequences before committing.
- Valid routes are compared openly rather than hidden behind secret prerequisites.
- Irreversible actions repeat the exact scope, cost or story choice in confirmation.
- Declining or leaving does not remove unrelated inventory or completed work.
### 07 · Graph relationship 1
- Connects to trust: uses shared accessibility foundations.
- Relationship type: `extends`.
- Concrete connection is selected by the stable idea IDs in world_graph.json.
- A graph edge is a design contract, not an automatic permission to access private data.
### 08 · Graph relationship 2
- Connects to arcade: defines accessible game modes.
- Relationship type: `applies_to`.
- Consumers request an explicit service result rather than querying another feature's private tables.
- If the target service is not implemented, the connection remains clearly marked as planned.
### 09 · Graph relationship 3
- Connects to platform: unifies client controls.
- Relationship type: `implemented_on`.
- The source event and target scope are validated before any cross-domain write.
- Failure in the consumer cannot partially rewrite this feature's completed state.
### 10 · Graph relationship 4
- Connects to creator: checks community content readability.
- Relationship type: `validated_by`.
- The relationship must be backed by a narrow Rust application-service contract.
- Private player and guild data remain scoped to the requesting actor and authorised audience.
### 11 · Input contract
- Inputs include authenticated actor context, stable target IDs, current definition version and minimum required state.
- Party and guild actions validate membership, role, visibility and authority server-side.
- Item or currency actions validate ownership, eligibility and available balance inside the mutation transaction.
- Client-supplied IDs, balances, permissions and completion flags are untrusted.
### 12 · Output contract
- A readability report.
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
