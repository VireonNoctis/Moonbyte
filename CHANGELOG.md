# Changelog

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
