# 0013 — First Puzzle Vault vertical slice

- Status: Accepted
- Scope: One authored, solo Puzzle Vault on the existing Atlas
- Product version: Initial 1.0.0 baseline; this ADR is a design decision, not a product release
- Depends on: PostgreSQL, Atlas discovery, account-bound inventory, Chronicle and the command presentation adapter

## Decision

Build the first Vault as a bounded, authored, deterministic challenge called **Mosslight Resonance**. It is a real Discord-native activity, not a website, Activity or browser game. The first release is solo-only; party and Guild Vaults remain separate future work.

The Vault is entered from the Tavern and the Atlas. Entry requires the player to have discovered Mosslight Grove. The player sees the rules, three-room structure, expected short session length, save behavior and exact reward before starting.

## Room design and verification

The first blueprint is immutable at content version 1. Each room has a stable ID, authored clues, a finite accepted answer set, one verified solution and a post-solve explanation.

1. **Waystone Order** — order Fern, Star and Moon using the published clues: Fern precedes Star; Star precedes Moon. The only accepted sequence is Fern → Star → Moon.
2. **Tideglass Code** — find a three-digit code with distinct digits from 1 through 5. The first digit is twice the second, the third is one more than the second, and the digits sum to nine. The unique solution is 423.
3. **Keeper's Seal** — combine the three room marks in the displayed order to enter the fixed seal phrase. The authored phrase and its exact clue are published as part of the room definition; the verifier accepts only the canonical normalized answer.

The content registry must validate stable IDs, non-empty clues, non-empty explanations, answer normalization and solution fixtures before a blueprint can be registered. Unit tests enumerate the finite Tideglass Code candidates and prove uniqueness. A room cannot be published if its solution is missing, duplicated or ambiguous.

Wrong answers produce a short hint or explain which clue to revisit; they do not remove progress, consume items, award random outcomes, impose a timer or lock the player out. The answer is never silently corrected. After solving, show the explanation and record the room checkpoint.

## State and recovery

- One active run per player and blueprint version; a completed run can be replayed for practice, but the reward is one-time.
- State transitions are owned by a Rust Vault service and persisted in PostgreSQL.
- Each run stores the owner, blueprint ID/version, current room, lifecycle state, row version and timestamps. Completed room checkpoints are stored with stable room IDs and completion timestamps.
- The state machine is Available → Active → Paused/Completed, with Leave as a non-destructive exit. A player can resume from the last committed room at any time; no attendance deadline exists.
- Mutations lock the run row, validate the owner and expected version, and commit before Discord delivery. Stale or repeated button actions refresh the current board and cannot solve a room twice.
- The first implementation does not store raw message text or a history of incorrect guesses. Only the current state and completed checkpoints are retained.

## Reward and interconnections

Completion grants exactly one account-bound **Mosslight Seal** (keepsake.mosslight-seal) with source provenance referencing the run and blueprint version. The item is not tradeable, sellable, spendable or randomly generated. A unique provenance/idempotency constraint prevents duplicate grants on retries or replay.

On completion, the Vault writes its own completion record and uses owning service boundaries to:
- add the keepsake to Inventory;
- add a private Chronicle page for the Mosslight Resonance discovery;
- record the verified completion for Wayfarer progress;
- make the completion visible in the player's private Archive/collection projection.

These effects must be committed atomically with the completion record or be delivered through an idempotent transactional hand-off. The initial implementation should prefer one short PostgreSQL transaction over introducing a broker or background service.

No Moonbits are granted in this first slice. A fixed collectible is sufficient to prove the Vault → Inventory → Chronicle → Wayfarer → Archive chain without expanding the currency ledger.

## Discord interface

Use one unified /vault and prefix .vault command with sub-actions: list, enter <blueprint>, status, solve <room> <answer>, pause and leave. The same Rust application service handles both invocation styles.

- list shows the available blueprint, prerequisites, room count, expected time, fixed reward and save behavior.
- enter requires explicit confirmation and returns the first room.
- status reconstructs the current room and saved checkpoint from PostgreSQL.
- solve accepts a text answer and offers a select-menu/button alternative where it improves accessibility.
- pause and leave preserve completed rooms and do not revoke earned rewards.
- A compact title embed identifies the Vault and current room. Private clues and progress are ephemeral/private; no public board exposes a player's private run.
- Invalid room IDs and near-match action names receive conservative “Did you mean …?” guidance, never automatic execution.

The first slice is solo-only. Do not create a party lobby, public leaderboard, editor, random room order or Guild Vault command yet.

## Privacy and migration requirements

The migration adds player-owned run and checkpoint records with foreign keys, unique constraints and indexes for active-run lookup. It must not store raw Discord message content or unrelated profile fields.

Before the feature is marked implemented:
- add Vault run/checkpoint categories to the private data inventory and JSON export;
- delete all runs and checkpoints owned by the player during confirmed account deletion;
- include the account-bound reward and source provenance in the existing inventory export/deletion contract;
- test guild-independent personal ownership and prevent another Discord user from reading or mutating a run;
- test transaction rollback, concurrent solves, duplicate completion, replay, restart recovery and stale Discord controls;
- test that leaving or pausing never loses a committed checkpoint.

## Explicitly deferred

Party Vaults, Guild Vaults, Mastery trials, generated puzzles, player-authored Vaults, random room order, timers, leaderboards, tradable relics, keys and Bazaar integration are not part of this first vertical slice. They require separate ownership, moderation, fairness, privacy and acceptance decisions.
