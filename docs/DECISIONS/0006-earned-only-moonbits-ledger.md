# ADR 0006 — Earned-only Moonbits and first-completion ledger

- Status: Accepted
- Date: 2026-11-02
- Scope: PostgreSQL wallet storage, game-completion transaction, player privacy contract.

## Context

The World Loom and economy specification call for an earned-only currency, an immutable ledger, explicit rewards and duplicate-claim protection. A full economy, inventory, Vault and Bazaar are not ready. Introducing a spendable balance or marketplace before those systems have atomic transaction, escrow, privacy and recovery tests would be premature.

## Decision

1. Introduce a minimal PostgreSQL wallet and immutable ledger without enabling spending, transfers, purchases, cash-out or trading.
2. Award a fixed 10 Moonbits to each participant on their first completed session of each game type.
3. Do not base the milestone on winning. It acknowledges completion, including a draw or a completed solo activity.
4. Enforce one award per player/game type with a unique idempotency key: `first-game-completion:<game_id>`.
5. Insert the ledger entry and increment the wallet balance in the same PostgreSQL transaction as the final game snapshot, result row and per-game statistics.
6. Keep ledger entries after their source session expires. The source session ID is retained as provenance, not as a foreign key.
7. Extend privacy inventory, JSON export and account deletion before exposing the wallet.
8. Keep the first reward reason as the only permitted ledger reason in the initial schema. New reward types require a reviewed migration, explicit policy and idempotency tests.

## Consequences

- A player can earn no more than one first-completion reward for each game type.
- Replaying the same game type increases game statistics but cannot create more Moonbits.
- The balance and immutable ledger are auditable and transactionally consistent for the implemented milestone.
- The wallet can be inspected privately with `.wallet` and appears in the Tavern as The Ledger.
- Moonbits cannot be bought, withdrawn, cashed out, staked, transferred outside Moonbyte or spent in this release.
- Personal Vaults, item ownership, spending, trading, Bazaar escrow and guild economy remain future work.

## Verification

PostgreSQL integration tests cover the fixed reward, repeat-completion protection, wallet balance/ledger count, wallet export and account deletion. Formatting and Clippy remain CI gates.
