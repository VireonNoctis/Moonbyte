# Changelog

## 2026-11-05 — Pre-launch operations and Workshop crafting

- Added fixed Atlas material sources at Mosslight Grove and Glasswater Pier, with migration backfill for existing discoveries.
- Added the one-time Wayfarer's Lantern recipe with deterministic ingredients, player-level advisory locking, transactional input consumption and account-bound output provenance.
- Added the private craft command and Wayfarer Workshop Tavern destination.
- Added a PostgreSQL readiness probe to the ping command.
- Added manual custom-format PostgreSQL backup and checksum-verified restore scripts that refuse non-empty targets.
- Added CI checks for shell syntax and explicit Rust compiler-version reporting.
- Restored document/policy version labels to the v1.0.0 pre-launch baseline; database migration and JSON export schema numbers remain sequential technical identifiers.
- Clarified throughout the README and canonical docs that no Discord application has been registered or deployed.

## 2026-11-05 — Atlas keepsake inventory and collection

- Added five account-bound postcards, one for each Atlas location, with stable item IDs and source provenance.
- Grants are deterministic and idempotent, written in the same PostgreSQL transaction as Atlas discovery and Chronicle updates.
- Backfilled existing Atlas discoveries so current players receive the matching keepsakes.
- Added private `.inventory` and `.collection` commands and two Tavern destinations.
- Extended privacy inventory, version 10 export, account deletion, integration tests and architecture documentation.
- Corrected the implementation map and roadmap to mark existing numbered game-board controls as complete.

## 2026-11-04 — Discord-native board controls

- Added numbered buttons to Tic-tac-toe, Connect Four and Memory Match boards.
- Routed button input through the authoritative game runtime, with consent, guild/channel, participant, turn and phase validation.
- Bound buttons to the displayed action count so old/replayed controls refresh instead of applying another move; terminal controls are disabled.
- Kept text commands as a fallback and Word Scramble as a text-answer game.
