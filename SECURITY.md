# Security Policy

## Reporting a vulnerability

Please do not disclose suspected vulnerabilities in public issues, pull requests, or discussions.

Use GitHub's **Report a vulnerability** feature in the repository's Security tab when private vulnerability reporting is enabled. If that option is unavailable, first ask the maintainers through a privately published contact route for a secure reporting channel. Do not share exploit details or sensitive information publicly.

Include a concise description, affected versions or commit, impact, and reproducible steps where safe. Do not include real user data, credentials, or secrets in the report.

## Scope and response

Security reports are reviewed by project maintainers. Please allow reasonable time for investigation and remediation. Do not assume a report is accepted as a vulnerability until maintainers confirm it.

## Privacy and data handling

Read [docs/PRIVACY.md](docs/PRIVACY.md) for Moonbyte's planned data practices and [the product status](README.md) before assuming a described control is already implemented.

Before production release, maintainers must enable private vulnerability reporting, document the actual hosting environment and backup controls, review database access, and verify privacy export/deletion behavior.

Do not claim end-to-end encryption, encryption at rest, anonymity, or zero logging unless the actual code and deployment configuration prove the claim.
