# 0003. Discord-native presentation and verifiable randomness

- Status: Accepted
- Date: 2026-11-04

## Context

Moonbyte is played inside Discord. Its interaction model should use Discord messages, embeds, buttons, select menus, modals and shared prefix/slash commands. The application also needs a clear authority boundary and a safe, auditable approach to randomness where a game actually requires public verification.

## Decisions

### Discord presentation

- Discord is the required and sufficient player interface.
- Use Serenity/Poise to present commands, messages and interactive components.
- Rust application/domain services own game rules, state, identity, permissions, progression, economy and persistence.
- Every component action is treated as untrusted input and revalidated against the current database state.
- Text labels and instructions remain understandable without colour or imagery.
- Use original full-colour pixel-art assets or custom emoji only when they materially improve clarity or world identity.
- Do not create a separate client or hosting surface without a new, concrete product requirement and accepted decision.

### Fairness

- Use OS-backed cryptographic entropy for secret seeds.
- Use SHA-512 for a versioned, domain-separated commit–reveal commitment.
- Derive results with a documented, deterministic, unbiased mapping and a public verifier before claiming verifiable fairness.
- Preserve transcripts for completed and aborted sessions where the feature's privacy contract permits it.
- A hash is a commitment/integrity primitive, not an entropy source or a standalone guarantee of fairness.

### Economy boundary

- Rewards and prices are disclosed and deterministic.
- No betting, gambling, wagers, stakes, paid random rewards, cash-out or chance-based tradable rewards.

## Consequences

- Moonbyte remains one Rust/Tokio/Serenity/Poise application backed by PostgreSQL.
- Discord responses remain useful even when no art asset is configured.
- Every component uses the same application service and authorization rules as its command equivalent.
- A verifiable random game cannot be released until test vectors, reveal paths and a verifier exist.
