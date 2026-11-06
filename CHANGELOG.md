# Changelog

## 2026-11-07 — Opt-in Guild Project foundation

- Added a server-scoped Lantern Road Survey with Manage Server setup, explicit member join/leave and private aggregate status.
- Count only opted-in actors' checkpoints in active same-server shared Lantern Road Expeditions; contribution writes are idempotent and share the Expedition transaction.
- Kept contribution rows free of contributor IDs. Export personal memberships (schema v12), remove memberships and actor links on deletion, and preserve anonymous server progress.
- Remove server-owned project state on actual bot removal; temporary guild unavailability does not trigger cleanup.
- Added PostgreSQL integration coverage for opt-in, non-member exclusion, guild isolation, completion, deletion/anonymisation and uninstall cleanup.


## 2026-11-07 — Consistent privacy exports and deletion concurrency

- Run every player-export read through one read-only PostgreSQL `REPEATABLE READ` transaction so related records cannot be mixed across concurrent commits.
- Add a PostgreSQL regression test that commits a wallet and matching ledger entry between the export's balance and ledger reads, proving the export stays on its original snapshot.
- Make the profile-creation/deletion race test deterministic by queuing a stale creation behind the deletion transaction's advisory lock and requiring the post-deletion consent re-check to reject it.
- Align the privacy, architecture, implementation and roadmap documents with the verified data-integrity contract.

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
