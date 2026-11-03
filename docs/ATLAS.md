# Atlas — Five-location starter map

- Status: The initial Atlas and its first-discovery Chronicle connection are implemented. The wider Atlas domain, Expeditions, authored story quests, narrative routes and community mapping remain proposed.
- Rust owner: `src/atlas.rs`.
- Discord adapter: `src/commands.rs`.
- Tavern entry: `.tavern` → The Atlas → `.atlas`.
- PostgreSQL migration: `migrations/0006_player_atlas.sql`.

## Player experience

The Atlas is a private map of the player's current location and places they have discovered. Opening it for the first time establishes Lantern Square as the starting point. It then shows the player's known places and the directly connected routes available from the current location.

The first map is intentionally small and authored. Every location has a stable ID, region, short description and route list. Route edges are reciprocal and the complete graph is connected.

| Location ID | Name | Region | Direct routes |
| --- | --- | --- | --- |
| `lantern-square` | Lantern Square | Hearthland | Mosslight Grove, Glasswater Pier |
| `mosslight-grove` | Mosslight Grove | Verdant Reach | Lantern Square, Ember Archive, Starfall Observatory |
| `glasswater-pier` | Glasswater Pier | Western Coast | Lantern Square, Starfall Observatory |
| `ember-archive` | The Ember Archive | Cinder Vale | Mosslight Grove, Starfall Observatory |
| `starfall-observatory` | Starfall Observatory | High Astral | Mosslight Grove, Glasswater Pier, Ember Archive |

## Commands

| Command | Behavior |
| --- | --- |
| `.atlas` | Open the player's private map, known places and available direct routes. |
| `.travel mosslight-grove` | Move to a directly connected location and record the visit. |
| `.travel Mosslight Grove` | Display names are accepted in addition to stable location IDs. |

The same handlers are available as slash commands. In a server, the map and travel response are delivered by DM; they do not publicly expose a player's personal location.

## Domain and persistence rules

- Rust owns the catalogue, route graph, adjacency checks and travel state.
- Only direct edges from the current location are traversable. A caller cannot skip through an undiscovered or non-adjacent location.
- Opening the Atlas creates the player's starting location once.
- Travel, discovery insertion, visit counting and current-location update happen in one PostgreSQL transaction.
- The player-world-state row is locked before a travel mutation.
- Reopening the map does not increment visit counts.
- Requesting the current location is a no-op and does not increment the visit count.
- Revisiting a previously discovered location updates its last-visited time and visit count.
- Travel does not issue Moonbits, achievements, items, experience or any other repeatable reward.
- The static location catalogue contains no user-authored executable content. A first-time discovery also unlocks one authored page in the player's private Chronicle; revisits do not duplicate pages or create rewards.

## PostgreSQL model

| Table | Ownership and purpose |
| --- | --- |
| `player_atlas_discoveries` | One row per player/location with discovery timestamp, latest visit timestamp and visit count. |
| `player_world_state` | One row per player containing the current location and update timestamp. |
| `player_archive_fragments` | One row per player/location, storing the page ID and first-unlock timestamp. |

Both tables reference `player_profiles(discord_user_id)` with `ON DELETE CASCADE`. The current-location field also references the player's discovery row using a deferred composite foreign key. This prevents world state from pointing at a location that is not recorded as discovered.

Migration 0006 adds the schema and a history index. The player data inventory reports both the current world-state row and discovery count. Version 6 JSON export contains the policy acceptance record, current location and each discovered location's timestamps and visit count. Confirmed account deletion locks and removes the world-state row and discovery records in the same transaction as the other player-owned data.

## Tests and acceptance criteria

The Atlas tests cover:
- Unique location IDs and valid route targets.
- Reciprocal route edges.
- Reachability of all five locations from Lantern Square.
- Stable ID and display-name lookup.
- First-open starting position.
- Rejection of a non-adjacent route.
- Successful travel, return travel, and no-op travel to the current location.
- Discovery timestamps and visit counts in PostgreSQL.
- Inclusion of world state and discoveries in privacy inventory/export/deletion, alongside the versioned Terms and Privacy acceptance record.

The Atlas is not the full World Loom map. Multi-region world generation, route hazards, Expeditions, shared party discovery, guild maps, authored story, quests, NPCs, landmarks beyond these five locations and visual map rendering are not implemented by this slice. They must each be added as separately tested vertical slices with their own ownership and privacy contracts.

## Future connection points

The stable location IDs are intended to become the shared spatial references used by:
- [Expedition Foundation v1](EXPEDITION_FOUNDATION.md), which is the next planned consumer of stable Atlas route IDs and checkpoint discoveries; its runtime is not yet implemented.
- Quests, which will anchor authored objectives to locations.
- The Archive, which will preserve confirmed discoveries and lore.
- Guild projects, which may add optional shared map contributions after guild isolation is implemented.

These connections remain product design contracts until the corresponding services, persistence, permissions, export/deletion coverage and acceptance tests exist. The first bounded Expedition proposal is specified in [Expedition Foundation v1](EXPEDITION_FOUNDATION.md); it must pass those gates before being described as live.
