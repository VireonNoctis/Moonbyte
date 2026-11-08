# ADR 0010 — Private Chronicle pages from first Atlas discoveries

- Status: Accepted
- Date: 2026-11-03
- Scope: Atlas discovery events, personal Archive persistence, privacy export/deletion and Tavern navigation.

## Context

The starter Atlas already persists a player's current location and first discoveries. The World Loom needs these discoveries to lead into the Archive without turning travel into a repeatable reward loop or exposing personal exploration history.

## Decision

1. Ship five reviewed, immutable lore pages, one for each starter Atlas location.
2. Unlock a page exactly once when its player/location discovery is recorded, in the same PostgreSQL transaction.
3. Backfill page rows from existing Atlas discovery records when migration 0010 is applied.
4. Keep page IDs and location IDs stable and enforce one page per player/location with database constraints.
5. Show a private collection through `.chronicle` and add it to the Tavern only after its command is registered.
6. Include page IDs, linked locations and unlock timestamps in the player inventory, version 8 JSON export and confirmed account deletion.
7. Do not attach Moonbits, power, competitive advantages or repeatable rewards to Chronicle unlocks.
8. Treat this as the first Archive slice; user-created, shared and guild chronicles remain proposals.

## Consequences

- Atlas discovery now has a persistent lore outcome without additional grind or currency.
- Existing explorers receive pages for locations they have already charted.
- The Chronicle remains private and follows the existing account export/deletion lifecycle.
- The broader Archive still requires independent contracts for community content, moderation and shared ownership.

## Verification

Unit tests verify complete one-to-one catalogue coverage. PostgreSQL integration tests verify starting-page creation, first-discovery unlock, duplicate prevention on revisits, export/inventory coverage and transactional deletion.