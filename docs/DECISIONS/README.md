# Architecture Decision Records

ADRs record decisions that affect multiple modules or are expensive to reverse. They explain rationale; current implementation status belongs in [Implementation Map](../IMPLEMENTATION.md).

## Accepted decisions

- [0001 — Product identity and original technology foundation](0001-moonbyte-foundation.md) — original application and Rust/Discord stack; the initial database choice was later superseded.
- [0002 — Privacy by design and data minimisation](0002-privacy-by-design.md).
- [0003 — Discord-native presentation and verifiable randomness](0003-discord-native-and-fairness.md).
- [0004 — PostgreSQL as the session and application store](0004-postgresql-storage.md).
- [0005 — Live-only Tavern navigation and configurable emoji IDs](0005-tavern-wayfinder-and-emoji-registry.md).
- [0006 — Earned-only Moonbits and first-completion ledger](0006-earned-only-moonbits-ledger.md).
- [0007 — Authored Atlas routes and private player discoveries](0007-authored-atlas-and-player-discoveries.md).
- [0008 — Guild command routing and prefixes](0008-guild-command-routing.md).
- [0009 — Versioned Terms and user consent](0009-versioned-terms-and-consent.md).
- [0010 — Private Chronicle pages from first Atlas discoveries](0010-private-chronicle-fragments.md).
- [0011 — Provenance-backed account-owned inventory](0011-provenance-backed-inventory.md).

When a decision is replaced, retain the historical record and clearly identify the newer source. Do not create an ADR for a local implementation detail.
