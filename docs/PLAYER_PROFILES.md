# Player Profiles — initial preference slice

- Status: Minimal preference persistence is implemented; profile statistics and full privacy controls are not.
- Storage: PostgreSQL `player_profiles` table, introduced by migration 0002.
- Ownership: one global profile row per Discord user ID, not one row per guild.

## Stored fields

| Field | Purpose |
| --- | --- |
| `discord_user_id` | Stable Discord account reference used to load the requester's own preferences. |
| `board_theme` | One fixed cosmetic board theme. Does not affect rules, score, fairness, visibility or rewards. |
| `created_at`, `updated_at` | Operational timestamps for profile creation and preference changes. |

No display name, avatar URL, biography, email address, contact list, location or message content is collected by this feature.

## Supported themes

- `classic` — default presentation.
- `forest` — green palette.
- `sunset` — warm palette.
- `lavender` — violet palette.

The Rust enum is the allow-list; PostgreSQL repeats the allow-list as a CHECK constraint. Unknown values are rejected rather than silently accepted. Themes are presentation-only and cannot influence game outcomes or progression.

## Commands

- `.profile` or `/profile` — create the default profile if needed and display the saved theme.
- `.profile forest` or `/profile theme: forest` — save a supported theme.
- `.profile delete` — show the exact confirmation phrase.
- `.profile delete confirm` — delete only the preference row; it does not delete active or completed game sessions.

The command always uses the invoking Discord user ID. It does not accept a target user's ID, so players cannot edit another player's profile through this endpoint.

## Privacy and limitations

The profile table contains a Discord account identifier and a single cosmetic preference. This is persisted data. The current `.privacy` command is informational only; private export and full account deletion are not yet implemented. `.profile delete confirm` removes only the theme preference row and must not be confused with deleting the player's game records. The repository exposes a profile-row deletion operation for service-level use and tests, but it is not a complete user-facing deletion workflow and does not delete session records, so it must not be marketed as account deletion.

Before adding more profile fields, update `docs/PRIVACY.md`, add a clear purpose and retention rule, test export/deletion coverage, and keep optional preferences separate from required session identity.