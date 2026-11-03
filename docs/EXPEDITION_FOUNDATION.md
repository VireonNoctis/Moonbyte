# Expedition Foundation v1

> Moonbyte World Loom · Implementation slice specification · 4 November 2026  
> **Status: planned design; not implemented.** The live Atlas, Chronicle and Wayfarer are prerequisites, not evidence that Expeditions already exist.

## 1. Purpose

Expeditions turn the current Atlas route graph into a bounded journey that can be completed in one sitting or paused and resumed later. The first release should prove that an authored route can coordinate persistent progress across the Atlas, Chronicle and Wayfarer without introducing a second game engine or coupling separate domains through direct table writes.

The principle is simple: **a journey is a saved, versioned sequence of meaningful checkpoints, not a timed attendance challenge.** A player can play alone, invite friends, stop at any point and return later. Missing a day never removes progress.

This document narrows the much larger Expedition and Story concepts in [World Loom Volume II](atlas/02_atlas_expeditions_and_story.md) into one deliverable vertical slice.

## 2. First route: The Lantern Road

The first route is an authored, deterministic journey:

`Lantern Square → Mosslight Grove → The Ember Archive → Starfall Observatory`

All four locations already exist in the five-location Atlas and each consecutive pair is a direct route in the current graph. The route has a stable content ID, `lantern-road`, and an explicit definition version. Content changes produce a new version; an in-progress run continues using the version it started with.

The first release uses fixed authored checkpoint text and clear player choices. It does not generate route events from hidden randomness. The route should teach players how expeditions work while deepening the places they already know.

### Checkpoint intent

| Order | Atlas location | Checkpoint purpose | Connected system |
|---:|---|---|---|
| 0 | Lantern Square | Assemble, inspect the route and confirm the journey. | Tavern / Atlas |
| 1 | Mosslight Grove | Make the first field observation and record the grove's story. | Atlas / Chronicle |
| 2 | The Ember Archive | Solve an authored, text-first archive puzzle and confirm the selected route clue. | Chronicle / Puzzle rules |
| 3 | Starfall Observatory | Complete the route, review the journey and return to the Atlas. | Atlas / Chronicle / Wayfarer |

The initial route must remain playable with Discord-native text and controls. A React/Discord Activity presentation can follow after the authoritative Rust service and authenticated visual API exist.

## 3. Player loop

1. Browse the available authored routes with `.expedition list`.
2. Inspect the route, estimated scope, player limit, required starting location and the exact personal Atlas effects before committing.
3. Start a solo run or create a recruiting group run in the current guild.
4. Invite or accept up to three additional players. Every player explicitly joins; no one is enrolled automatically.
5. Begin the route after all participants are at Lantern Square and confirm the start.
6. Advance checkpoint-by-checkpoint. The server validates the route version, current checkpoint, participant status and Atlas edge before writing.
7. At each checkpoint, show a concise recap, what changed in the player's saved Atlas/Chronicle, and the next available action.
8. Pause, leave or resume at any time. The run can remain paused indefinitely.
9. On completion, persist a private run summary and show the route's authored ending. Do not grant a random reward or an unannounced currency bonus.

A player should always be able to see the route map, participants, current checkpoint, run state and available actions before they act.

## 4. Party and guild rules

- Solo runs are always supported.
- A shared run has one to four participants, all belonging to the same Discord guild. Direct-message runs are solo only.
- A player must be at the route's starting location before joining. The interface states that checkpoint progress updates that participant's personal Atlas position and may unlock the matching private Chronicle page.
- A participant can leave without cancelling the shared run or deleting other participants' progress.
- The creator is the initial host. If the host leaves, host ownership transfers to the earliest-joined remaining participant. If no participants remain active, the run pauses safely.
- Only a participant can view private run details. Guild membership does not make a player's Chronicle or personal discovery data public.
- Group-wide progress is shared; each participant's own Atlas and Chronicle writes are made only for players who joined and consented.
- No matchmaking, public directory or unsolicited invitations in v1.

## 5. Run state machine

Allowed states:

| State | Meaning | Allowed next states |
|---|---|---|
| `recruiting` | Host created a group; members may explicitly join or leave. | `active`, `cancelled` |
| `active` | Participants may advance the current checkpoint or pause/leave. | `paused`, `completed`, `cancelled` |
| `paused` | No checkpoint can advance until an authorised participant resumes. | `active`, `cancelled` |
| `completed` | Final checkpoint and completion summary are committed. | Terminal |
| `cancelled` | Host/solo player ended the run. No penalty is applied. | Terminal |

A host can start, pause or cancel a run. Any participant can request a pause; the run pauses immediately so one participant is never pressured to continue. A participant can leave from any non-terminal state. Resume is an explicit action from a remaining participant. There is no expiry timer, missed-day reset, inactivity penalty or forced group notification.

Each successful state transition increments a monotonic row version. All advances compare the expected version and checkpoint ID inside a short PostgreSQL transaction.

## 6. Rust ownership and integration

The Expedition module owns route-run lifecycle, participant membership, checkpoints and run history. It does not own Atlas locations, Chronicle pages, Wayfarer activity or wallet entries.

The application service validates a checkpoint and invokes the owning domain operations using the same PostgreSQL transaction where the source transition requires atomic effects:

- **Atlas:** verify direct-neighbour travel and record each joined participant's new location/discovery.
- **Chronicle:** unlock the authored page only for a participant's first discovery of that location. Unique constraints remain the final duplicate guard.
- **Wayfarer:** record meaningful Expedition completion as a qualifying activity source, respecting the existing one-player-per-day rule. Existing accepted quest objectives progress only when their declared source condition matches; v1 does not silently change the daily quest catalogue or rotation.
- **Profile:** expose a private read-only recent-journey summary after the run is complete.
- **Tavern:** add a destination only after the Expedition command and service are genuinely registered.

The integration vocabulary uses the canonical events already defined in [Integration Contracts](atlas/INTEGRATION_CONTRACTS.md): `expedition.started`, `expedition.checkpoint.reached` and `expedition.completed`. Events are derived from committed source state, use stable IDs and idempotency keys, and contain no raw Discord message text, private random seeds or full Discord payloads.

For v1, cross-domain calls remain within the Rust application service and PostgreSQL transaction boundary. Do not add Redis, a broker, an outbox worker or a separate service. Revisit an outbox only if a later, measured requirement introduces asynchronous consumers that cannot be safely completed in the source transaction.

## 7. PostgreSQL data outline

The route catalogue and checkpoint definitions are versioned authored content in Rust. PostgreSQL stores player/run state only.

| Record | Owns | Important constraints |
|---|---|---|
| Expedition run | Route ID/version, optional guild ID, host, state, current checkpoint, row version, created/updated/completed timestamps. | Foreign keys, valid state, valid checkpoint bounds and optimistic concurrency. |
| Expedition participant | Run ID, player ID, role, joined/left timestamps, last acknowledged checkpoint. | Unique run/player membership; one active run per player; guild scope validation. |
| Checkpoint history | Run ID, route version, checkpoint ID, completing transition ID, timestamp. | Unique run/version/checkpoint; idempotent transition key. |

Migration naming follows the ordered SQLx migration series; the next migration number must be confirmed against the repository before implementation. Store timestamps in UTC. Index active runs by player and guild, and index checkpoint history by run and order. Do not persist arbitrary player prose or raw interaction payloads.

## 8. Commands and Discord UX

Proposed prefix and slash command family:

| Command | Purpose |
|---|---|
| `.expedition list` | List available authored routes and their requirements. |
| `.expedition start <route>` | Create a solo run or recruiting group run; show a confirmation summary first. |
| `.expedition invite <user>` | Invite a guild member to a recruiting run; requires an explicit acceptance by that member. |
| `.expedition join <run>` | Join a recruiting run after checking guild and starting-location scope. |
| `.expedition status <run>` | View the route, participants, checkpoint and available actions privately. |
| `.expedition advance <choice>` | Submit the current checkpoint choice after validating membership and state. |
| `.expedition pause <run>` | Pause a run. Any participant may request a pause. |
| `.expedition resume <run>` | Resume a paused run with explicit participant action. |
| `.expedition leave <run>` | Leave a run without erasing other participants' progress. |
| `.expedition cancel <run>` | Cancel a solo run or a group run as host; terminal runs remain in private history. |

Slash commands should expose structured Discord choices rather than require fragile free-form parsing. Use ephemeral/private responses for personal run details. Public channel messages should reveal only the fact that a group activity exists, never participant progress or private Atlas data unless each participant separately opts in.

## 9. Privacy and lifecycle

Every new player-owned record must appear in the privacy inventory and versioned JSON export before the feature is enabled. Export only the requester's own participation, checkpoint history and timestamps; do not expose other participants' private data by default.

Confirmed deletion removes the requester's participant rows and player-linked details. It must not erase valid history belonging to other participants. If the requester was host, transfer host ownership to the earliest remaining participant. If the requester was the only participant, remove or anonymise the run according to the current retention decision. If no participants remain in a group run, close it safely. The deletion transaction must leave the run and participant constraints valid.

Do not add new collection categories unrelated to the Expedition. Do not log checkpoint choices or message bodies beyond the minimum stable choice ID required to resume and audit the run.

## 10. Safety, accessibility and engagement

- A player can pause or leave at any time without losing completed checkpoints.
- No countdowns, attendance streaks, punitive expiry, random loot, fake scarcity or FOMO.
- No wagering, paid entry, tradable chance reward, cash-out or real-money value.
- Give a plain-text alternative for every map, sprite, button state and route clue.
- Do not communicate state only through colour; label the current checkpoint and available actions.
- Honour reduced-motion preferences in a future visual surface; Discord-native text must remain complete without motion.
- Keep puzzle instructions age-appropriate, non-graphic and solvable without external links.

## 11. Delivery plan

1. Finalise the route definition, state transitions and privacy rules. Keep the route content in source control with a stable definition version.
2. Add the PostgreSQL migration, repository methods and transaction boundaries. Test constraints against an isolated PostgreSQL database.
3. Implement the Rust Expedition state machine and application-service hand-offs to Atlas, Chronicle and Wayfarer.
4. Add prefix/slash commands and confirmation/consent UX. Keep the existing command gate and guild routing behavior intact.
5. Extend privacy inventory, JSON export and confirmed deletion; update the Privacy Policy only when the exact schema and retention behavior are known.
6. Add restart recovery, duplicate/concurrent action tests, guild isolation, accessibility checks and a full CI run.
7. Update the README, command reference, architecture, Atlas, roadmap and final vision from the verified shipped behavior—not before.

## 12. Release acceptance criteria

- [ ] The route definition is versioned and all route links are verified against the existing Atlas catalogue.
- [ ] The same Rust state machine supports solo and shared runs.
- [ ] Every participant explicitly consents before their Atlas state can change.
- [ ] Run creation, invitation, join, advance, pause, resume, leave, cancel and completion validate authorization and state.
- [ ] State transitions and checkpoint effects are durable, atomic where applicable, and idempotent.
- [ ] Concurrent advances cannot skip, duplicate or reorder checkpoints.
- [ ] Runs recover after a process restart and can remain paused indefinitely.
- [ ] Host departure, participant departure, guild mismatch and deletion leave valid ownership and history.
- [ ] Atlas, Chronicle and Wayfarer effects use their owning service boundaries.
- [ ] Export, inventory, deletion, retention and log-redaction tests cover all new records.
- [ ] Discord delivery failure does not reverse a committed checkpoint.
- [ ] Plain-text Discord UX is complete and no Activity/browser client is required.
- [ ] README, architecture, commands, privacy, Atlas, final vision and roadmap accurately distinguish planned from implemented behavior.

A checkbox is complete only after its test and documentation evidence is present. Writing the specification or registering a command alone does not make Expeditions a live feature.
