# ADR 0003: Discord-native Presentation and Verifiable Game Randomness

- Status: Accepted; this decision supersedes the earlier proposal for a separate React/Activity frontend.
- Date: 2026-11-04.

## Context

Moonbyte is a Discord game bot. The primary player experience should live in Discord's native interaction model: messages, embeds, buttons, select menus, modals, and shared prefix/slash commands. A separate website, React application, or Discord Activity is not required to make the bot useful and adds an independent deployment and security surface.

Moonbyte still needs a distinctive visual identity. Original full-colour pixel-art assets and custom emoji may improve selected game boards, maps and result screens, but they are presentation resources—not a second application or authority for game state.

The project also needs transparent randomness for outcomes that players may wish to verify.

## Decisions

### Discord-native presentation

- Discord is the primary and sufficient client for Moonbyte.
- Use Serenity/Poise messages, embeds, buttons, select menus and modals for interactive experiences.
- Rust domain and application services remain authoritative for game rules, session state, player identity, permissions, progression, economy and persistence.
- Use original full-colour pixel-art images or custom emoji only where they materially improve clarity or world identity.
- Every important state and action must retain a text-first Discord representation; colour or imagery alone must not carry critical information.
- Do not add a website, Next.js, React frontend, browser game client or Discord Activity without a new, concrete product requirement and an accepted architecture decision.
- Do not add art pipelines, generated assets or rendering services speculatively. Begin with Discord-native components and introduce assets as individual features need them.

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

- The bot remains one Rust/Tokio/Serenity/Poise application with PostgreSQL persistence.
- There is no frontend workspace, Node dependency tree, frontend deployment or Activity hosting requirement.
- Discord-native UI remains complete without an image asset; original art can be attached to messages when a feature justifies it.
- Rust remains the authority for game state, rules, randomness, permissions and persistence.
- Randomised games require reproducible test vectors and transcript verification before release.

## References

- [Discord message components](https://discord.com/developers/docs/components/overview)
- [Discord embeds](https://discord.com/developers/docs/resources/message#embed-object)
