# ADR 0008 — Guild command routing and prefixes

- Status: Accepted
- Date: 2026-11-03
- Scope: Rust Discord adapter, PostgreSQL guild configuration and command dispatch.

## Context

A single process-wide prefix is not sufficient for a multi-server bot. Server moderators need a prefix that fits their server and may optionally keep bot commands inside one designated channel. These settings must apply consistently to prefix and slash commands, remain isolated by guild ID, and be recoverable if the selected channel is no longer suitable.

## Decision

1. Keep DEFAULT_PREFIX as the process-wide fallback and preserve the period as its documented default.
2. Persist server overrides in guild_command_settings, keyed by Discord guild ID. Each row stores the prefix, optional command-channel ID, last editor ID and timestamps.
3. Load settings into an in-process read cache at startup. Dynamic prefix resolution and command-channel checks read this cache rather than querying PostgreSQL for every message.
4. Expose one prefix command in both prefix and slash forms. It requires Discord's MANAGE_GUILD permission.
5. Support show, set <prefix> [#channel], channel <#channel|off> and reset. Prefixes are limited to 1–8 non-whitespace characters; channel IDs must resolve to a channel in the invoking guild.
6. When a command channel is configured, all commands other than prefix must be invoked there. The prefix command remains available anywhere so moderators can recover or change the setting.
7. DMs continue to use the process-wide default prefix and have no guild channel restriction.
8. Keep guild routing data separate from player exports and player deletion. Bot-removal cleanup remains a production operations requirement.

## Consequences

- Each server can use a prefix that does not affect any other server.
- Prefix and slash command use follows one consistent command-channel rule.
- Ordinary messages do not generate PostgreSQL reads.
- A process restart reloads settings from PostgreSQL; changing settings updates the current process cache immediately.
- Multiple simultaneously running bot processes would need shared cache invalidation before being treated as a supported deployment topology.

## Verification

Unit tests cover prefix validation and command-input parsing. PostgreSQL CI verifies migration and store integration. The global command check and dynamic-prefix callback are wired into the same Poise framework so both invocation styles consume the same guild settings.
