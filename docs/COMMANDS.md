# Moonbyte Command Reference

The default prefix is `.`. Guild moderators with Manage Server can set a per-server prefix and optionally restrict commands to one channel. The same registered commands are available through slash commands.

## Command response UX

Substantial browse and overview results use a Discord embed title panel. Slash commands expose an optional `title` field on Craft, Tavern, Games, profile/data views, inventory/collection, Quests, Wayfarer, Rivalry, Atlas, Chronicle, travel and active-session views. The private Player Card is a read-only summary of currently live systems; unavailable future sections are not fabricated. Prefix commands accept title text after their required arguments (for example, `.craft wayfarer-lantern My Workshop`). Leave it blank to use Moonbyte's standard heading. Titles are whitespace-normalised, mention-safe and capped at Discord's 256-character title limit.

Craft keeps its current visibility: slash results are ephemeral and prefix results in a server are delivered by DM. Tavern and Games remain public browse panels. Player-owned slash views and active-session lists are ephemeral; prefix invocations in a server are delivered by DM. Expedition catalogue/status screens use the same standard title panel while preserving the existing multi-part command grammar. Game boards retain their canonical title and native buttons. Tiny confirmations and action receipts remain ordinary messages.

Slash-command inputs with a finite catalogue use Discord autocomplete dropdowns. Craft suggestions display the output item and required materials, while sending the canonical recipe ID to the handler. The same pattern is used for game selection, Atlas destinations, quest/rivalry/Expedition/Vault actions and profile visibility; `/vault` also suggests the authored chamber IDs. The parameter description explains the field; each suggestion is selectable from the dropdown. Prefix commands keep their text syntax.

Vault uses the same optional title-panel treatment: slash invocations have a `title` field, while trailing prefix text after list/status/pause/resume/leave becomes the panel title. Enter confirmations and solve answers remain untouched. Finite command arguments also receive conservative typo hints when a typed value is close. Suggestions are advisory only: Moonbyte never silently rewrites or executes a corrected recipe, game, destination or action. Ambiguous/unrelated input receives normal usage guidance instead.

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
| `.tavern [title]` | `/tavern title:` | Open the 16-destination Tavern Wayfinder. |
| `.profile [title]` | `/profile title:` | Open your private Player Card: game totals, achievements, Wayfarer/Companion, Rivalry counts, Chronicle/collection progress and Vault status, with shortcuts to live systems. |
| `.profile_edit` | `/profile_edit` | Set a display name and short bio. |
| `.settings` | `/settings` | Save profile visibility and preference flags. |
| `.stats [title]` | `/stats title:` | View completed-game totals and per-game wins/draws. |
| `.mastery [game]` | `/mastery game:` | View calculated overall or per-game ranks derived from completions and wins; no extra data is stored. |
| `.wallet [title]` | `/wallet title:` | View earned-only Moonbits and ledger-entry count. |
| `.achievements [title]` | `/achievements title:` | View five fixed achievement milestones. |
| `.inventory [title]` | `/inventory title:` | Inspect personal keepsakes with quantity and source provenance. |
| `.collection [title]` | `/collection title:` | View the five Atlas postcards and collection progress. |
| `.craft wayfarer-lantern [title]` | `/craft recipe: title:` | Recipe uses autocomplete; suggestions show the Wayfarer's Lantern and its two required materials. One account-bound craft per player. |
| `.quests [title]` | `/quests title:` | Open the optional daily objective board. |
| `.quest accept <id>` / `.quest claim <id>` | `/quest` | Opt into an objective or claim its fixed completed reward. |
| `.wayfarer [title]` | `/wayfarer title:` | View cumulative journey days. |
| `.streak [status|milestones]` | `/streak view:` | View the non-resetting journey count or the complete companion milestone ladder. |
| `.companion [title]` | `/companion title:` | View cosmetic companion milestones. |
| `.rivals [title]` | `/rivals title:` | View private rivalry invitations and records. |
| `.rival declare @player` / `accept <id>` / `decline <id>` | `/rival` | Manage mutual-consent rivalries. |

## World, Puzzle Vaults and Expeditions

| Prefix | Slash | Purpose |
|---|---|---|
| `.atlas [title]` | `/atlas title:` | View personal location, discoveries and direct routes. |
| `.travel <location> [title]` | `/travel location: title:` | Location autocomplete shows each place and region; route reachability is checked before travel. |
| `.chronicle [title]` | `/chronicle title:` | Privately read lore pages and your mutual Rivalry Log. |
| `.vault [action] [target] [details] [title]` | `/vault action: target: details: title:` | Open Mosslight Resonance, enter after confirmation, inspect progress, solve a chamber, pause, resume or leave. Action and chamber fields use Discord autocomplete; browse/status actions accept a custom panel title. Solve answers remain one canonical token, with optional title text after the answer. |
| `.expedition` | `/expedition` | List routes, start/join/begin, inspect status, advance, pause, resume, leave or cancel. |

Mosslight Resonance is a solo three-chamber Puzzle Vault unlocked by discovering Mosslight Grove. Open it with `.vault list`; use `.vault enter mosslight-resonance confirm` to explicitly begin or resume a saved run. The chambers are Waystone Order, Tideglass Code and Keeper's Seal. Submit the current chamber with `.vault solve <room-id> <answer>`. A correct answer stores a private save stone; incorrect guesses are checked in memory, never saved, and return a short authored hint. `.vault pause`, `.vault leave` and `.vault resume` preserve progress; runs do not expire. Completion grants one fixed account-bound Mosslight Seal, a private Chronicle page and a Wayfarer activity event. Practice replays do not grant a second keepsake.

The Atlas panel and Tavern Wayfinder both link to the Vault; the Atlas displays the entry after Mosslight Grove is discovered.

Lantern Road is the one implemented route: Lantern Square → Mosslight Grove → The Ember Archive → Starfall Observatory. Start solo with `.expedition start lantern-road solo` or create a same-server recruiting group with `.expedition start lantern-road group`. Other players join explicitly using `.expedition join <run-id>`. Shared runs support up to four participants. All participants must be at Lantern Square before the host begins. Use `.expedition status <run-id>`; advance with `.expedition advance <run-id> <checkpoint-id> <choice>`. Participants may pause, resume or leave. If the host leaves, ownership transfers to the earliest remaining active participant. Runs do not expire for inactivity.

## Server projects

| Prefix | Slash | Purpose |
|---|---|---|
| `.guild start` | `/guild action:start` | Open the server's Lantern Road Survey board. Requires Manage Server or Administrator. Repeating setup preserves progress. |
| `.guild join` | `/guild action:join` | Explicitly enrol yourself; only your later shared Expedition checkpoints count. |
| `.guild leave` | `/guild action:leave` | Opt out of future contributions without removing earned server progress. |
| `.guild status` | `/guild action:status` | View aggregate progress and your own enrolment state privately; member names are never shown. |

The first project requires two opted-in checkpoint events from active same-server shared Lantern Road Expeditions. Solo runs and activity before enrolment do not count. Contribution records have no contributor ID. Leaving or deleting an account does not roll back anonymous server progress; actual bot removal deletes the server board.

## Games and sessions

| Prefix | Slash | Purpose |
|---|---|---|
| `.games [title]` | `/games title:` | Browse the four playable games and their controls. |
| `.play <game>` | `/play game:` | Game choice autocomplete shows the available games and player mode; starts a solo game or two-player lobby. |
| `.join <session>` | `/join` | Join an open two-player lobby in the same server. |
| `.action <session> <input>` | `/action` | Submit a validated move or answer. |
| `.session <session>` | `/session` | Inspect a session you participate in. |
| `.sessions [title]` | `/sessions title:` | List up to ten recent active sessions in the current server or DM scope. |
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
2. `.privacy_export` sends a private JSON export. Version 12 includes the requester's own Expedition participation/checkpoint history, Puzzle Vault runs and solved chamber checkpoints, account-bound postcard/material/crafted-item/Vault-keepsake provenance, and opt-in Guild Project memberships. It omits other participants' IDs, private quest seeds and unsaved Vault guesses.
3. `.privacy_delete` explains the exact deletion scope; it does not delete yet.
4. `.privacy_delete DELETE` confirms deletion.

Deleting a player removes that player's owned records, including inventory provenance and Guild Project memberships, and anonymises their Expedition checkpoint actor links. Anonymous server project aggregates remain. Shared game-session snapshots containing their Discord ID are removed. The command warns that deleting a shared snapshot also ends that session for its co-participants. It cannot delete Discord's own records, copies held by other participants or external infrastructure backups. See [Privacy Policy](PRIVACY.md) and [Implementation Map](IMPLEMENTATION.md).
