# The Chronicle — private Atlas archive

- Status: The first private Chronicle slice is implemented.
- Rust owner: `src/chronicle.rs`.
- Discord command: `.chronicle` and `/chronicle`.
- Tavern entry: `.tavern` → The Chronicle.
- PostgreSQL migration: `migrations/0010_player_chronicle.sql`.

## Player experience

The Chronicle is a personal collection of five authored lore pages. Each page is tied to one of the five starter Atlas locations. Opening the Chronicle also initialises the player's Lantern Square starting discovery; travelling to a new connected location records its page automatically.

Unlocked pages show their title, location, region and short story fragment. Undiscovered locations remain as unwritten entries. The collection is private and is delivered by DM when the command is used in a server.

## Domain and persistence rules

- Rust owns the page catalogue and stable page/location IDs.
- A page is inserted in the same PostgreSQL transaction as its first Atlas discovery.
- The unique player/page and player/location constraints prevent duplicate unlocks.
- Migration 0010 backfills pages for every Atlas location already discovered before this feature shipped.
- Revisiting a location never creates another page, currency award, power increase or repeatable reward.
- The Chronicle contains authored content only; it does not store arbitrary player-submitted text.
- Account inventory, JSON export and confirmed deletion include every unlocked page and its first-unlock timestamp.

## First-release catalogue

| Page ID | Location | Title |
| --- | --- | --- |
| `lantern_first_light` | Lantern Square | The First Light |
| `mosslight_old_stones` | Mosslight Grove | The Stones That Remember |
| `glasswater_tide_notes` | Glasswater Pier | The Tidekeeper's Measure |
| `ember_storykeepers` | The Ember Archive | A Page Kept Warm |
| `starfall_night_chart` | Starfall Observatory | A Map for Returning |

## Current boundary

This is the first Archive slice, not the complete World Loom Archive. User-created chronicles, shared guild records, collectible sets, museum displays, search, annotation and cross-region lore networks remain future work. They must each receive moderation, ownership, privacy, export/deletion and access-control contracts before implementation.