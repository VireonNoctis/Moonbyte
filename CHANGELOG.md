# Changelog

## 2026-11-05 — Atlas keepsake inventory and collection

- Added five account-bound postcards, one for each Atlas location, with stable item IDs and source provenance.
- Grants are deterministic and idempotent, written in the same PostgreSQL transaction as Atlas discovery and Chronicle updates.
- Backfilled existing Atlas discoveries so current players receive the matching keepsakes.
- Added private `.inventory` and `.collection` commands and two live Tavern destinations.
- Extended privacy inventory, version 10 export, account deletion, integration tests and architecture documentation.
- Corrected the implementation map and roadmap to mark existing numbered game-board controls as complete.

## 2026-11-04 — Discord-native board controls

- Added numbered buttons to Tic-tac-toe, Connect Four and Memory Match boards.
- Routed button input through the authoritative game runtime, with consent, guild/channel, participant, turn and phase validation.
- Bound buttons to the displayed action count so old/replayed controls refresh instead of applying another move; terminal controls are disabled.
- Kept text commands as a fallback and Word Scramble as a text-answer game.

## 2026-11-04 — Structured operational tracing

- Added compact `tracing` output with an environment filter and a conservative default filter.
- Added minimised lifecycle and cleanup events without player/guild IDs, message contents, credentials, private payloads or seed material.
- Replaced raw runtime error output with a generic failure event to avoid leaking dependency connection details.
- Corrected the Tavern footer so its live Atlas and Expedition destinations are not described as planned.

## 2026-11-04 — Discord-first cleanup and implementation alignment

- Confirmed Discord-native commands/messages as the required player experience; removed the unused visual-library workspace from the branch.
- Documented the live Lantern Road Expedition, twelve Tavern destinations and version 9 privacy export accurately.
- Consolidated duplicated feature documents into one Implementation Map and separated product vision, architecture, operations and roadmap sources.
- Kept the ten World Loom volumes as the detailed proposal catalogue rather than duplicating their ideas across feature files.
- Added graceful shutdown handling for Ctrl-C and Unix SIGTERM.
- CI run 476 for the preceding branch head passed repository validation, PostgreSQL-backed tests, formatting and Clippy.

## 2026-11-03 — Wayfarer progression and connected player records

- Added optional daily objectives, fixed disclosed rewards and idempotent ledger claims.
- Added cumulative journey days, cosmetic companion milestones and mutual-consent private rivalries.
- Connected game completion and first-time Atlas discoveries to Wayfarer progress within their source transactions.
- Extended privacy inventory, version 9 export, deletion and Tavern registration.
