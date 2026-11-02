# Tavern and Wayfinder

- Status: The initial Tavern home and Wayfinder are implemented.
- Command: `.tavern` and `/tavern`.
- Owner: `src/tavern.rs` (presentation/domain navigation data) and `src/commands.rs` (Discord adapter).
- Emoji registry: `src/emoji.rs`.

## Purpose

The Tavern is the home and discovery entry point in the Moonbyte World Loom. The first implementation is deliberately a navigation-only vertical slice: it makes existing player capabilities discoverable from one place without pretending that the full Hearth, Atlas, Guild, Vault or Bazaar systems are live.

## Live Wayfinder destinations

The single `AVAILABLE_DESTINATIONS` registry in `src/tavern.rs` contains the six destinations that are backed by registered commands:

| Stable destination ID | Tavern label | Live command | Current capability |
| --- | --- | --- | --- |
| `arcade` | The Common Tables | `.games` / `/games` | Browse four playable games; start solo games or multiplayer lobbies. |
| `player-profile` | Your Place | `.profile` / `/profile` | View the player's private profile and preferences. |
| `player-record` | The Chronicle | `.stats` / `/stats` | View completed-game totals, wins and draws. |
| `moonbits-ledger` | The Ledger | `.wallet` / `/wallet` | View the player's earned-only Moonbits balance and first-completion milestones. |
| `achievement-gallery` | The Gallery | `.achievements` / `/achievements` | View the five fixed achievement milestones. |
| `world-atlas` | The Atlas | `.atlas` / `/atlas` | View the player's current place, discovered places and directly connected routes. |

The Wayfinder is generated from this registry rather than maintaining a second handwritten list. Every destination has a stable internal ID, a title, a description and a registered command route. Tests verify the current command targets and visible destinations. The Ledger points only to the read-only wallet summary; it does not enable spending or trading.

Browsing the Tavern does not create a game, join a lobby, claim a reward, or alter saved player state. Starting or changing an activity always requires a separate explicit command.

## What is not live yet

The following World Loom proposals are not implemented by the Wayfinder itself and must not be presented as playable destinations until their domain behavior, persistence and tests exist:

- Tavern tables, host controls, parties, guild rooms, rumour circles and NPC interactions.
- The wider Atlas graph beyond the five-location starter map, region-specific discovery layers, expeditions and story chapters.
- Quests, Puzzle Vaults, inventory, crafting, collections and homesteads.
- Moonbits spending, transfers, immutable marketplace ledger operations, reward definitions beyond first-completion milestones, and Vault checkpoints.
- Bazaar listings, escrow, barter, contracts, settlement and dispute handling.

The full design remains in [the World Loom master atlas](atlas/README.md), [Hearth/Bazaar/Communities volume](atlas/01_hearth_bazaar_and_communities.md), [Atlas/Expeditions/Story volume](atlas/02_atlas_expeditions_and_story.md), and [Economy, Vaults & Bazaar](ECONOMY_AND_VAULTS.md).

## Atlas starter destination

The first Atlas destination is implemented as a bounded world-map slice, not the entire Atlas proposal.

- Five reviewed locations have stable IDs, regions, descriptions and reciprocal route edges.
- Each player begins at Lantern Square on first opening.
- Travel is only allowed to a directly connected location. The command accepts a location ID or name.
- Current location and personal discoveries are persisted in PostgreSQL. Reopening the Atlas and attempting to travel to the current location do not create visits.
- No travel currency, achievement or repeatable reward is issued.
- The `.atlas` response is private; browsing does not expose one player's location to a public channel.

The Atlas data is owned by the authenticated player's profile. Export, inventory and confirmed deletion cover `player_world_state` and `player_atlas_discoveries`. Expeditions, multi-region content, narrative routes, guild maps and player-created map content remain planned.
## Emoji registry

The bot's five upload names are declared in `src/emoji.rs`:

| Rust slot | Discord upload name |
| --- | --- |
| `ids::MOONBYTE` | `moonbyte` |
| `ids::ROCKET` | `moonbyte_rocket` |
| `ids::TREASURE` | `moonbyte_treasure` |
| `ids::CRYSTAL` | `moonbyte_crystal` |
| `ids::CHAT` | `moonbyte_chat` |

After uploading the PNGs to the target Discord server, paste each 17–20 digit numeric Discord snowflake ID into its matching `Option<&str>` slot as `Some("123456789012345678")`. Do not paste the complete Discord markup. Missing or malformed IDs use the defined Unicode fallback, so unconfigured emoji slots never generate broken mentions. The current PNGs are static, so the registry emits static custom-emoji markup.

## Engineering rules for future destinations

1. Give each destination a stable ID that is independent of its command name and display label.
2. Add a destination only after its command or UI action reaches a real, permission-checked domain service.
3. Keep world rules, permissions and persistence in Rust. Discord remains an adapter.
4. Define owner, participant and guild scope before persisting any destination-specific state.
5. Make entry and irreversible actions explicit; browsing must be side-effect free.
6. Provide exit, pause, mute and privacy controls where the activity needs them.
7. Add adapter/domain tests and update privacy inventory, export, deletion and retention when new player-owned data is introduced.
8. Link the implementation from the World Loom graph only when the feature's own acceptance tests pass.

## Verification

The implementation includes unit tests for the registered destination list, command routes, Atlas route links and private travel records, rendered prefix handling, custom emoji name/ID validation and Unicode fallback. Discord deployment smoke tests remain an operational check and are not claimed by repository unit tests.
