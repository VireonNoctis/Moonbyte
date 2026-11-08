# ADR 0007 — Authored Atlas routes and private player discoveries

- Status: Accepted
- Date: 2026-11-02
- Scope: Rust Atlas domain, PostgreSQL persistence, Tavern Wayfinder, player privacy contract.

## Context

Moonbyte's World Loom uses the Atlas as the canonical map shared by future Expeditions, Quests and the Archive. The full Atlas design has many proposed regions and discovery systems, but none should be presented as live until its behaviour and persistence are implemented. A first map slice is needed to prove stable location IDs, route ownership, player discovery persistence and privacy coverage without claiming the full world is playable.

## Decision

1. Begin with five reviewed, static locations and a stable, reciprocal direct-route graph owned by `src/atlas.rs`.
2. Give each player Lantern Square as a starting location on first opening of the Atlas.
3. Permit travel only along a direct route from the player's current location. An invalid or non-adjacent destination does not mutate state.
4. Persist the player's current location and personal discovery/visit history in PostgreSQL.
5. Update discovery, visit count and current location in one transaction with a row lock on the player's world-state record.
6. Make opening the map read-only after first initialisation. Travel to the current location is a no-op. Revisited locations update visit metadata but do not grant rewards.
7. Do not attach currency, achievements, items or repeatable rewards to Atlas travel.
8. Tie both Atlas tables to `player_profiles` through cascading foreign keys. Include current location and discovery rows in the privacy inventory, version 5 JSON export and confirmed deletion transaction.
9. Expose the first Atlas through private `.atlas` and `.travel` commands and the Tavern Wayfinder. Keep personal location data out of public channel responses.
10. Treat Expeditions, multi-region content, community maps and story routes as future systems that require their own reviewed contracts and tests.

## Consequences

- Stable place IDs can be referenced by future game and story systems without duplicating location names.
- The initial route graph is deterministic, inspectable and testable.
- Players retain personal discovery history across restarts.
- There is no random travel outcome, reward farming or attendance pressure.
- Account deletion removes the player's world state and discoveries.
- The first Atlas is intentionally limited and does not imply that the full World Loom map, Expeditions or Quests are live.

## Verification

Unit tests cover catalogue identity, route reciprocity, full graph reachability and reference lookup. PostgreSQL integration tests cover initialisation, valid and invalid travel, revisiting, no-op travel and persisted visit counts. Profile integration tests cover Atlas inventory, version 5 export and deletion.
