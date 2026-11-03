# The Wayfarer Thread

The Wayfarer Thread connects completed play and Atlas exploration to a lightweight player journey.

## Daily quests

The board assigns three objectives per player and PostgreSQL calendar date from a fixed catalogue:

| Quest | Objective | Fixed Moonbits |
| --- | --- | ---: |
| Warm Up the Arcade | Complete one game | 5 |
| A Few More Rounds | Complete three games | 8 |
| Find Your Rhythm | Win one game | 8 |
| Try Another Table | Complete two distinct game types | 10 |
| A New Corner of the Map | Discover one new Atlas location | 7 |

Players opt into individual objectives with \`.quest accept <id>\`. Game completions and first-time Atlas discoveries update accepted objectives automatically. Completed rewards are claimed with \`.quest claim <id>\` and credited through the immutable wallet ledger. The unique player/date/quest key prevents duplicate rewards.

The quest selection is stable for the day. PostgreSQL's \`gen_random_uuid()\` supplies a server-generated UUID seed. Moonbyte uses domain-separated SHA-512 digests and rejection sampling when mapping digest bits to selection indices; no modulo bias is introduced. The persisted SHA-512 fingerprint is displayed as a short rotation label and included in player export. The seed remains server-side and is not exported. SHA-512 is a hash, not encryption and not a source of randomness by itself.

## Activity and companion

A completed game or a first-time Atlas discovery creates at most one activity-day record for the player/date. Replaying a game or revisiting a location does not create additional days. The cumulative total unlocks cosmetic stages for Mallow:

| Active days | Stage |
| ---: | --- |
| 1 | New Companion |
| 3 | Pathfinder |
| 7 | Trail Scout |
| 14 | Wayfinder |
| 30 | Starbound |
| 60 | Constellation Keeper |
| 100 | Elder Star |

The journey is cumulative. Missing a day never subtracts activity or resets a milestone. Companions do not alter game rules, stats, matching or rewards.

## Rivalry Hall

\`.rival declare @player\` sends a private invitation to an existing Moonbyte player. The recipient alone can accept or decline with the invitation ID. Both players independently declaring each other activates the relationship because both have opted in. Only completed two-player sessions after activation are counted. Records are shown privately with \`.rivals\`; there is no public rivalry leaderboard, wager, stake, random payout or penalty.

## Data controls

The version 7 private player export includes Wayfarer activity dates, quest progress/status/reward data, rotation fingerprints, and rivalry invitation/match records. It omits the private quest seed. Account deletion removes the activity, quest, rotation and rivalry rows as part of the existing PostgreSQL deletion transaction. See [the Privacy Policy](PRIVACY.md).
