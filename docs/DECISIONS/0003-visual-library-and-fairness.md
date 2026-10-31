# ADR 0003: Visual Library and Verifiable Game Randomness

- Status: Accepted product/architecture direction.
- Date: 2026-10-31.

## Context

Moonbyte needs a richer visual game library while remaining Discord-first, and it needs transparent randomness for game outcomes that players may wish to verify. Ordinary Discord messages and embeds cannot host a full React component tree. A web application can be embedded in Discord as an Activity through Discord's Embedded App SDK, subject to Activity configuration and platform requirements.

## Decisions

### Visual library

- Build the visual library using **Next.js + React + TypeScript**.
- Treat it as a frontend surface for game discovery, rich boards, lobbies, and Activity-capable game views.
- Use Discord's official Embedded App SDK when the frontend is launched as a Discord Activity.
- Keep a normal HTTPS browser fallback for discovery and compatible views.
- The frontend never owns authoritative game rules, wallet balances, trades, or persistent session state. Rust services remain authoritative.
- Keep bot tokens and database credentials server-side. Never expose them in browser bundles.
- Standard bot commands continue to use Discord-native messages, embeds, buttons, select menus, and modals. React does not render inside a normal embed.

### Fairness

- Use OS-backed cryptographic entropy for secret seeds.
- Use SHA-512 for a versioned, domain-separated commit–reveal commitment.
- Derive outcomes with a documented, deterministic, unbiased mapping and publish a verifier.
- Preserve transcripts for completed and aborted sessions.
- A hash is a commitment/checking primitive, not a random source and not a standalone guarantee of fairness.

### Economy boundary

- Vaults and Bazaar use earned-only, non-redeemable currency and transparent fixed rewards/prices.
- No betting, wagering, staking, paid random rolls, paid loot boxes, cash-out, or chance-based tradable rewards.

## Consequences

- The repository gains a separate visual frontend project and deployment surface once implementation begins.
- Activity integration requires Discord application configuration, URL mapping, HTTPS hosting, and Embedded App SDK testing.
- Rust remains the authority for game state, rules, randomness, permissions, and persistence.
- Randomized games require reproducible test vectors and transcript verification before release.
- A separate browser/Activity frontend adds deployment and security work; it should be introduced only as the product's planned visual library, not as an unrelated dashboard.

## References

- [Discord Activities overview](https://discord.com/developers/docs/activities/overview)
- [Discord Embedded App SDK](https://discord.com/developers/docs/developer-tools/embedded-app-sdk)
