# Moonbyte Command Reference

The default prefix is `.`. Guild moderators with Manage Server can set a per-server prefix and optionally restrict commands to one channel. The same registered commands are available through slash commands.

## Command response UX

Substantial slash-command results use a compact Discord embed title panel so the command context remains visible above the result. The shared Rust `send_title_panel` adapter is the reusable presentation path; Craft currently uses the **Wayfarer Workshop · Crafting** title. Apply it to browse-style outputs such as Games, Atlas, Expeditions, Quests and collection views when their response is a substantial panel, but do not add decorative embeds to tiny confirmations or weaken private-response handling. Slash panels are ephemeral by default; prefix commands retain normal message visibility.

Finite command arguments should provide conservative typo suggestions when one known value is clearly close (for example, a misspelled recipe ID). Suggestions are advisory only: Moonbyte never silently rewrites or executes a corrected command. Ambiguous or unrelated input receives the normal usage guidance instead.

## General and consent

| Prefix | Slash | Purpose |
|---|---|---|
| `.help` | `/help` | Show commands registered in this build. |
| `.about` | `/about` | Describe Moonbyte and current implementation scope. |
| `.ping` | `/ping` | Confirm handler responsiveness and process uptime. |
| `.terms` | `/terms` | Receive the current Terms of Service. |
| `.privacy` | `/privacy` | Receive the current Privacy Policy. |
| `.consent` | `/consent` | Check agreement status; use `accept` after reviewing both policies. |
| `.prefix` | `/prefix` | View or configure this server's prefix and command channel. |

New users must review both current policies and run `.consent accept` before ordinary commands unlock. Legal and privacy controls remain available before acceptance. Material policy changes require renewed consent.

## Player and progress

| Prefix | Slash | Purpose |
|---|---|---|
| `.tavern` | `/tavern` | Open the 15-destination Tavern Wayfinder. |
| `.profile` | `/profile` | View a private player profile. |
| `.profile_edit` | `/profile_edit` | Set a display name and short bio. |
| `.settings` | `/settings` | Save profile visibility and preference flags. |
| `.stats` | `/stats` | View completed-game totals and per-game wins/draws. |
| `.wallet` | `/wallet` | View earned-only Moonbits and ledger-entry count. |
| `.achievements` | `/achievements` | View five fixed achievement milestones. |
| `.inventory` | `/inventory` | Inspect personal keepsakes with quantity and source provenance. |
| `.collection` | `/collection` | View the five Atlas postcards and collection progress. |
| `.craft wayfarer-lantern` | `/craft` | Use one Mosslight Silk and one Tideglass to craft the account-bound Wayfarer's Lantern once. |
| `.quests` | `/quests` | Open the optional daily objective board. |
| `.quest accept <id>` / `.quest claim <id>` | `/quest` | Opt into an objective or claim its fixed completed reward. |
| `.wayfarer` | `/wayfarer` | View cumulative journey days. |
| `.companion` | `/companion` | View cosmetic companion milestones. |
| `.rivals` | `/rivals` | View private rivalry invitations and records. |
| `.rival declare @player` / `accept <id>` / `decline <id>` | `/rival` | Manage mutual-consent rivalries. |

## World and Expeditions

| Prefix | Slash | Purpose |
|---|---|---|
| `.atlas` | `/atlas` | View personal location, discoveries and direct routes. |
| `.travel <location>` | `/travel` | Move to an adjacent Atlas location. |
| `.chronicle` | `/chronicle` | Privately read lore pages unlocked through first discoveries. |
| `.expedition` | `/expedition` | List routes, start/join/begin, inspect status, advance, pause, resume, leave or cancel. |

Lantern Road is the one implemented route: Lantern Square → Mosslight Grove → The Ember Archive → Starfall Observatory. Start solo with `.expedition start lantern-road solo` or create a same-server recruiting group with `.expedition start lantern-road group`. Other players join explicitly using `.expedition join <run-id>`. Shared runs support up to four participants. All participants must be at Lantern Square before the host begins. Use `.expedition status <run-id>`; advance with `.expedition advance <run-id> <checkpoint-id> <choice>`. Participants may pause, resume or leave. If the host leaves, ownership transfers to the earliest remaining active participant. Runs do not expire for inactivity.

## Games and sessions

| Prefix | Slash | Purpose |
|---|---|---|
| `.games` | `/games` | Browse the four playable games and their controls. |
| `.play <game>` | `/play` | Start a solo game or create a two-player lobby. |
| `.join <session>` | `/join` | Join an open two-player lobby in the same server. |
| `.action <session> <input>` | `/action` | Submit a validated move or answer. |
| `.session <session>` | `/session` | Inspect a session you participate in. |
| `.sessions` | `/sessions` | List up to ten recent active sessions in the current server or DM scope. |
| `.leave <session>` | `/leave` | End a session you participate in. |

Current action formats: Tic-tac-toe uses 1-based `row,column`; Connect Four uses a 1-based column from 1–7; Memory Match uses a 1-based position from 1–8; Word Scramble accepts an answer. The first three games also expose numbered Discord buttons, with text commands retained as a fallback. Game state remains server-authoritative and stored in PostgreSQL.

## Prefix configuration

- `.prefix` — view current settings.
- `.prefix set ! #bot-commands` — change prefix and optionally channel.
- `.prefix channel #bot-commands` — change only the allowed command channel.
- `.prefix channel off` — allow commands in all channels.
- `.prefix reset` — restore the process default.
  
The configuration command remains available from any channel so a server can recover from an incorrect channel restriction. DMs use the process default. Settings are guild-scoped and are not part of player exports.

## Privacy controls

1. `.privacy_data` privately lists the requester's current stored data categories, including inventory provenance.
2. `.privacy_export` sends a private JSON export. Version 10 includes the requester's own Expedition participation/checkpoint history and account-bound postcard, material and crafted-item provenance and omits other participants' IDs and private quest seeds.
3. `.privacy_delete` explains the exact deletion scope; it does not delete yet.
4. `.privacy_delete DELETE` confirms deletion.

Deleting a player removes that player's owned records, including inventory provenance, and shared game-session snapshots containing their Discord ID. The command warns that deleting a shared snapshot also ends that session for its co-participants. It cannot delete Discord's own records, copies held by other participants or external infrastructure backups. See [Privacy Policy](PRIVACY.md) and [Implementation Map](IMPLEMENTATION.md).
