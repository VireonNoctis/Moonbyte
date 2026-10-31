# 0002. Privacy by design and data minimisation

- Status: Accepted
- Date: 2026-10-31
- Owners: Project maintainer

## Context

Moonbyte stores game progress and player state, and receives Discord interaction payloads. A gaming bot can over-collect through logs, analytics, leaderboards, and retained message content. A clear privacy stance is required before implementation.

## Decision

- Store only the minimum Discord identifiers and game data needed for enabled features.
- Do not sell or rent player data or use it for advertising, behavioural profiling, or AI model training.
- Do not add advertising trackers or third-party analytics.
- Do not retain raw message content as general analytics or transcripts.
- Provide a privacy command for overview, data inventory, export, and confirmed deletion.
- Provide guild-scoped controls and enforce cross-guild data isolation.
- Adopt explicit retention targets for sessions, logs, guild configuration, and backups.
- Do not claim end-to-end encryption or encryption at rest unless the implementation and deployment prove it.
- Keep the policy synchronised with schema, logs, deployment, and external providers.

## Alternatives considered

Broad usage analytics were rejected as unnecessary. Full command transcripts were rejected because they create disproportionate privacy risk. End-to-end encryption claims were rejected because Discord and the bot must process game interactions and stored state.

## Consequences

The application needs explicit data ownership and repository queries, privacy export and deletion workflows, retention jobs, log redaction, and integration tests for guild isolation. Some diagnostic investigations will have less data available; observability must rely on minimised metadata and correlation IDs.
