# 0012. Deterministic account-bound crafting

- Status: Accepted
- Date: 2026-11-05

## Context

Inventory and provenance need a first real consumer before additional materials, a Vault or a Bazaar are introduced. The first recipe should prove that source-backed inputs can be consumed safely without random yields, duplicate output or trade semantics.

## Decisions

- The first recipe is a single authored recipe: one Mosslight Silk plus one Tideglass creates one Wayfarer's Lantern.
- Each material is granted once by the first discovery of its named Atlas location. The Atlas remains authoritative for discovery.
- Existing discoveries receive the same materials through migration 0013. Replaying a discovery cannot grant a duplicate material source.
- Crafting is account-bound and one-time per recipe. No purchase, transfer, trading, random yield or spending is enabled.
- Input rows are locked and consumed in deterministic acquisition order; output is inserted in the same PostgreSQL transaction.
- A per-player transaction advisory lock serialises craft attempts. A unique output source reference makes the completed recipe idempotent.
- Craft command output is private. The inventory remains the source of truth for ownership and provenance.
- Privacy export and account deletion continue to operate over the shared inventory table.

## Consequences

- More recipes must declare exact inputs, outputs and eligibility in authored Rust data.
- New material sources must be linked to verified domain transitions and must be idempotent.
- The Bazaar remains blocked until item eligibility, moderation, escrow, settlement and concurrency contracts are implemented separately.
