# Migration from Lunar

- Source project: [VireonNoctis/Lunar](https://github.com/VireonNoctis/Lunar)
- Status: Migration inventory and policy; no code has been imported by this document.
- Last updated: 2026-10-31.

## 1. Migration principle

Moonbyte is a separate product. Lunar is its foundation in selected game concepts, game ideas, and implementation lessons—not a service Moonbyte calls at runtime.

The migration target is idiomatic Rust domain logic behind Moonbyte's own game/session interfaces. Avoid copying entire cogs or carrying over old Discord-specific architecture merely to preserve the old file layout.

## 2. Candidate inventory

| Lunar area | Candidate behavior | Moonbyte decision |
| --- | --- | --- |
| Akinator command | Interactive yes/no guessing. | Audit external service/API dependency and terms. Rebuild independently or defer if unsuitable. |
| Coin flip | Random outcome. | Good first migration candidate; non-wagering only. |
| Fun commands: dice | Dice-style random outcome. | Good first migration candidate; strict validation and deterministic tests. |
| Fun commands: 8-ball | Random party response. | Candidate for quick-play/party category with appropriate content. |
| Fun commands: rate | Social rating prompt. | Review abuse, targeting, and privacy implications. |
| Gacha/card system | Collectible cards and progression. | Audit data model, reward odds, and dependencies. If retained, redesign as transparent earned collection with no paid pulls or wagers. |

This is a starting inventory, not a complete source audit. Inspect each implementation, import, data model, and test before committing to a port.

## 3. What must not be inherited automatically

- Lunar's running bot process or deployment.
- Lunar website accounts, OAuth, or account-link requirements.
- Lunar's production database or schema.
- Unrelated anime/manga, media-discovery, profile, or community features.
- Legacy command permission gates that do not fit the standalone gaming product.
- External scraping, undocumented APIs, or dependencies with unclear terms.
- Unreviewed user data, tokens, or assets.

## 4. Safe migration workflow

1. Inspect source files, imports, data models, configuration, tests, and behavior.
2. Identify dependencies and whether licences and service terms permit the planned use.
3. Separate game rules from Discord event and message handling.
4. Write rules as independent Rust types and deterministic functions.
5. Add tests for normal, invalid, boundary, and repeated actions.
6. Adapt the game to Moonbyte's session contract.
7. Add Discord adapter tests and verify cleanup/recovery.
8. Record intentional behavior changes in game documentation.
9. Verify migration does not import unnecessary user data or message history.

## 5. Data migration

Do not connect to Lunar's live database. Do not import production user records by default.

A later migration of user-owned collection data requires a documented source and target schema, user-visible opt-in, data minimisation, validation, a tested rollback/discard strategy, and confirmation that source data and assets may lawfully be used.

The initial product starts with new Moonbyte player records and new SQLite storage.

## 6. Definition of migration complete

A migrated game is complete only when source behavior and licence are reviewed, rules are independently tested, the Discord adapter uses Moonbyte's session runtime, privacy requirements are satisfied, and the game operates with Lunar offline.
