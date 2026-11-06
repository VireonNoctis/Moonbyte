# 0011. Provenance-backed account-owned inventory

- Status: Accepted
- Date: 2026-11-05

## Context

Atlas discovery already creates a durable personal record and a private Chronicle page. The next connected progression layer needs a small, reliable inventory foundation before gathering, crafting or a Bazaar can exist. A speculative economy or random-drop system would create ownership, fairness and privacy problems before those domains have contracts.

## Decisions

- PostgreSQL is the source of truth for player-owned inventory records.
- Every item has a stable item ID, quantity, source type, source reference and acquisition timestamp.
- The first inventory release is limited to five authored Atlas postcards, one per location.
- A postcard is awarded only from an Atlas discovery transition and is recorded in the same PostgreSQL transaction as the discovery and Chronicle effect.
- Repeated delivery of the same source transition is idempotent. Migration 0012 backfills postcards for existing Atlas discoveries.
- These postcards are account-bound. No purchase, transfer, trade, sale, chance-based drop or spending path exists.
- Inventory is private by default and is included in data inventory, versioned export and confirmed account deletion.
- Future materials and crafting must be deterministic, source-verified and provenance-preserving. Bazaar eligibility requires a separate accepted design and atomic settlement implementation.

## Consequences

- The inventory module owns item definitions and inventory reads/writes; Atlas remains the authority for whether a location was discovered.
- Cross-domain writes occur only through the owning InventoryStore method inside the source transaction.
- The collection view is a projection over owned item records, not a second source of truth.
- New inventory-backed features must extend export, deletion, access-control tests and retention documentation before shipping.
