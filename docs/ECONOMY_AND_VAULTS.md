# Moonbyte Economy, Vaults & Bazaar

- Status: Approved direction for product design; no economy or marketplace system is implemented yet.
- This document defines a game economy, not a real-money financial service.
- All currency is earned in Moonbyte, has no cash value, cannot be bought, sold, withdrawn, or redeemed, and is never used for wagering.

## 1. Design principles

Moonbyte's economy should make exploration, crafting, collection, and player trading more expressive. It must not become a gambling product or a pressure system.

Hard rules:

- No real-money gambling, betting, stakes, wagering, cash-out, prize pools, or player-to-player betting.
- No paid currency, paid random rolls, paid loot boxes, or chance-based purchases.
- No random reward that can be bought, opened with purchasable currency, or traded for cash value.
- No pay-to-win upgrades. Competitive advantages must be earned through play and balanced by game rules.
- Rewards and costs must be visible before a player commits.
- Players can enjoy the game without trading, maintaining a shop, or logging in every day.
- No punitive decay, forced timers, artificial scarcity, or FOMO-based shop rotations.

## 2. Currency model

Use a small, transparent currency model rather than many confusing tokens.

### Moonbits — soft earned currency

Moonbits are earned through defined, non-random activities:

- Completing a game session, with reasonable daily caps against farming.
- Finishing quests and story chapters.
- Solving optional daily or weekly challenges.
- Completing guild cooperation milestones.
- Earning achievements for skill, exploration, or mastery.

The award amount is shown before claiming wherever feasible. Repeated low-effort matches, AFK behavior, self-trading, and collusion must not create an unlimited farming loop.

Moonbits:

- Are not purchasable with money.
- Cannot be withdrawn, transferred outside Moonbyte, or exchanged for real-world value.
- Cannot be staked or risked in a game.
- Have no speculative market or exchange rate.
- Are separate from any Discord monetisation or SKU system.

### Other progression

Use non-tradable experience, achievement marks, titles, profile frames, and collection completion for status. Keep competitive power separate from cosmetic prestige.

## 3. Vault system

Vaults are structured exploration and challenge experiences—not paid chests or random-reward boxes.

### Vault types

1. **Puzzle Vaults** — multi-room logic challenges with clues, switches, and a verified solution path.
2. **Expedition Vaults** — a map-based sequence of rooms with choices, optional detours, and a recoverable checkpoint.
3. **Relic Vaults** — curated lore and collectible discoveries with published completion rewards.
4. **Guild Vaults** — cooperative server challenges where members contribute clues or solve different rooms.
5. **Mastery Vaults** — skill tests tied to a particular game, with clear difficulty and a fixed reward.
6. **Archive Vaults** — replayable records of completed chapters, discovered lore, and collection milestones.
7. **Crafting Vaults** — resource and recipe puzzles that unlock documented crafting blueprints.
8. **Seasonal Vaults** — optional themed content that remains available for a generous period and does not punish absence.

### Vault loop

1. Browse vault descriptions, difficulty, time expectations, and exact reward preview.
2. Select a vault and read its rules.
3. Start solo or invite a party where supported.
4. Solve room objectives, clues, and encounter rules.
5. Save at explicit checkpoints and leave without losing completed progress.
6. Receive the published completion reward and add discoveries to the Archive.
7. Review a clear run summary and optional personal best.

### Keys and access

If keys are used, they are deterministic progression items earned from quests, milestones, or crafting recipes. Never sell keys, require a paid random item, or consume a scarce key for an undisclosed outcome. Prefer reusable access once a vault is unlocked.

### Vault content and rewards

- Every reward is listed before entry.
- The reward is deterministic for the vault version and completion condition.
- Optional routes may offer a choice between known rewards, not a hidden random roll.
- Failed attempts can provide progress hints or an alternate route; they must not confiscate unrelated inventory.
- Vault difficulty is based on puzzle/game complexity, not on making players wait or repeat content.
- Guild vaults show contribution clearly and prevent duplicate reward claims.

### Personal Vault

Each player can have a personal storage space for earned collectibles, blueprints, cosmetics, discovered relics, and saved loadouts. Capacity should not be a monetisation lever. Use sensible limits only where they improve organisation or technical safety, and explain them clearly.

## 4. Bazaar

The Bazaar is a player-to-player trading hall for eligible, earned, non-premium items.

### Bazaar features

- **Player stalls:** a simple shop page with a custom name, short description, and fixed-price listings.
- **Fixed-price listings:** seller sets an exact Moonbit price; buyer sees the total before confirming.
- **Barter offers:** exchange a clearly listed set of eligible items without hidden valuation.
- **Wanted board:** players post requests for specific craftable or collectible items.
- **Crafting orders:** a player requests an item using a published recipe and agreed reward.
- **Guild bazaar:** optional guild-scoped trading view with separate filters and moderation.
- **Item catalogue:** inspect item origin, rarity classification, trade eligibility, and source (quest, craft, vault, or achievement).
- **Trade history:** private, searchable records of a player's completed trades and current listings.
- **Market insights:** aggregate reference prices based on completed trades, with small-sample warnings and no speculative investment framing.

Do not introduce real-money purchases, third-party sales, cash conversion, or gambling-style auction/bid games. Fixed prices and explicit barter keep transactions understandable.

### Safe transaction lifecycle

1. Seller creates a listing and chooses the exact price/item amount.
2. The platform validates ownership and eligibility, then reserves listed items in escrow.
3. Buyer reviews item, quantity, seller, total cost, and any fee.
4. Buyer explicitly confirms; the transaction executes atomically.
5. The seller receives the currency and the buyer receives the item.
6. Both players can inspect the receipt and report a problem.

The database must prevent duplicated purchases, double-spending, self-purchases, race conditions, and item duplication. Expired or cancelled listings return reserved items exactly once.

### Bazaar protections

- Per-player listing limits and reasonable listing expiry.
- No self-trading, circular trades for reward farming, or duplicate purchase claims.
- Rate limits for creating, editing, and cancelling listings.
- Clear confirmation for every irreversible trade.
- Immutable transaction IDs and an auditable ledger.
- Report, block, and moderator review flows.
- No public exposure of private profile or account information.
- Guild configuration must not permit cross-guild data leakage.
- No hidden fees: display fees before confirmation, and keep the fee schedule stable and documented.

## 5. Collection, crafting and ownership

- Collections are catalogues of discovered items, creatures, relics, cards, recipes, and cosmetics.
- Items carry stable IDs, provenance, tradeability, and versioned definitions.
- Crafted items derive from published recipes and known inputs.
- Unique achievement rewards and account-bound items can be marked non-tradable.
- Cosmetics never grant hidden competitive advantages.
- Item balance changes must preserve user trust; do not silently delete legitimate earned assets.

## 6. Luck-themed games — no gambling

Moonbyte may include casual chance-based mini-games for entertainment, such as a no-stake coin flip or dice result, but these must never function as gambling.

Strict boundaries:

- No entry fee, stake, wager, pot, payout, or loss of currency/items.
- No chance-based reward of currency, tradable items, keys, or competitive power.
- No player-versus-player betting, odds screens, house edge, or betting advice.
- No cash-out or real-world value.
- Scores, animations, and non-tradable cosmetic presentation are acceptable only when they do not create a wagering loop.

If a proposed design needs a stake or payout to be interesting, redesign it as a skill, puzzle, or score-only game instead.

## 7. Fairness and randomness

Read [FAIRNESS.md](FAIRNESS.md) for the SHA-512 commit–reveal design. The economy must not use random rolls to decide purchases, market prices, item payouts, or progression awards. Vault and quest rewards are disclosed and deterministic.

## 8. Data model outline

Likely tables/entities once implementation begins:

- `wallets`: player ID, non-negative balance, version.
- `ledger_entries`: immutable transaction ID, player, amount delta, reason, source ID, timestamp, idempotency key.
- `items`: stable item ID, definition version, owner, provenance, tradeability.
- `vault_definitions`: versioned vault content and published rewards.
- `vault_runs`: player/party, vault version, checkpoint, state, result.
- `bazaar_listings`: seller, item/quantity, fixed price, state, expiry.
- `trade_transactions`: buyer, seller, escrow state, receipt, idempotency key.
- `trade_events`: immutable state transitions and moderation references.

Never store currency balances without a ledger. Balance updates and ledger entries must share a single SQLite transaction. Use constraints and unique idempotency keys to prevent duplicate credits or debits.

## 9. Release sequence

1. Deterministic rewards and player inventory.
2. Personal Vault as storage and collection archive.
3. Solo Puzzle Vault vertical slice with fixed rewards.
4. Crafting and item provenance.
5. Fixed-price Bazaar listings and atomic escrow.
6. Guild Vault and guild Bazaar only after guild-isolation tests pass.
7. Moderation, reporting, trade history, and operational tools.

No economy or marketplace is production-ready until double-spend, duplicate interaction, race, rollback, deletion/export, and guild-isolation tests pass.
