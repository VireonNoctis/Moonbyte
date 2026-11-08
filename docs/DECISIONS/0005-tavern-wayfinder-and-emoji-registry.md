# ADR 0005 — Live-only Tavern navigation and configurable emoji IDs

- Status: Accepted
- Date: 2026-11-02
- Scope: Rust command adapter, Tavern presentation, Discord custom emoji configuration.

## Context

The Moonbyte World Loom describes a connected world with the Tavern as its social home and discovery entry point. The full Hearth, Atlas, Guild, Vault and Bazaar domains are still proposed. Advertising proposed destinations as working would create false product expectations and make the home screen unreliable.

The new visual identity also uses five original Discord emoji PNGs that are uploaded separately by the operator. Their Discord custom-emoji IDs do not exist until upload, and must not be invented or hard-coded in command strings.

## Decision

1. Implement the first Tavern Wayfinder as a read-only navigation surface.
2. Keep its destinations in one typed Rust registry with stable internal IDs, human labels, descriptions and command routes.
3. Register only destinations backed by real commands. The initial list is the Common Tables (`games`), player profile (`profile`) and game record (`stats`).
4. Browsing the Tavern must not create sessions, mutate progress, claim rewards or change preferences.
5. Keep custom emoji names in `src/emoji.rs`; store only optional numeric IDs in one place. Generate static Discord markup after validating both the emoji name and numeric ID.
6. Use Unicode fallbacks while IDs are unset or invalid. Uploading and pasting IDs later must not require changing call sites.
7. Add new Wayfinder destinations only when their owning domain service, permission rules, persistence, privacy behavior and tests exist.

## Consequences

- The initial Tavern works immediately without needing uploaded custom emojis.
- Uploading the PNGs later is a configuration-only edit to `src/emoji.rs`.
- A missing emoji ID cannot produce broken custom-emoji syntax.
- The current Tavern is not a persistent social room, party system, or world simulation. Those features require their own design and implementation.
- The larger World Loom remains a design proposal; the initial navigation does not mark its full domains as implemented.

## Verification

Unit tests cover the five emoji upload names, custom ID validation, Unicode fallback, the live destination registry, command routing and safe prefix rendering.
