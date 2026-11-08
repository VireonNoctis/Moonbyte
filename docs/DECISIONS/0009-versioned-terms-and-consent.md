# ADR 0009 — Versioned service terms and user consent

- Status: Accepted
- Date: 2026-11-03
- Scope: Discord command availability, policy documents and player-owned privacy data.

## Context

Moonbyte needs an explicit record that a user agreed to its current Terms of Service and Privacy Policy before using ordinary bot features. A policy change must be capable of invalidating an older acceptance. Users must still be able to review the documents, check consent status, and exercise privacy rights before agreeing.

## Decision

1. Keep the current Terms and Privacy documents in version control. The current versions are defined by TERMS_VERSION and PRIVACY_VERSION in the Rust consent module.
2. Store one current acceptance record per Discord user ID in PostgreSQL. It contains both document versions, the acceptance timestamp and the optional guild where acceptance occurred.
3. Require an explicit consent accept command. Do not infer acceptance from command use, button display or merely opening either document.
4. Apply a global command check before ordinary commands. If either stored version does not match the current version, stop execution and explain how to open both full documents and accept them.
5. Exempt terms, privacy, consent status/acceptance, and privacy data/export/deletion commands from the gate so users can review the policies and manage their data at any time.
6. Include the consent record in the private data inventory, version 6 JSON export and transactional account deletion.
7. Keep the consent record global to the Discord user rather than guild-local. The optional guild ID records where the acceptance was made; it does not scope the agreement.
8. Do not create short aliases for help, terms, consent or privacy commands. Prefix shortcuts apply only to selected non-legal navigation/game commands.
9. Bump the relevant document version when a material policy change requires renewed agreement. The next ordinary command prompts for the updated agreement.

## Consequences

- Users cannot enter ordinary game, profile or progression commands without accepting both current documents.
- Users retain access to the full documents, consent status and privacy controls before and after acceptance.
- Maintainers must update the policy files and version constants together. A mismatch intentionally triggers renewed acceptance when the version constant changes.
- Acceptance metadata is personal data and is covered by the same inventory/export/deletion process.
- This versioned acknowledgement is not a substitute for legal review before a public launch.

## Verification

Tests must cover version comparison, acceptance, policy-version invalidation, inventory/export/deletion inclusion, pre-consent command exemptions, and normal-command blocking.
