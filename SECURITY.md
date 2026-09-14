# Security Policy

## Supported Versions

The current main branch is supported. Prior commits are not.

## Reporting a Vulnerability

Report security issues to `security@tmig.example` (replace with a real address).

Do not open a public issue.

## Response Timeline

- **48 hours**: acknowledgement of the report.
- **7 days**: initial assessment.
- **90 days**: coordinated disclosure, or sooner if a fix lands.

## Scope

The content-addressed HITL ledger, the control plane, the sub-arity
primitives, the fellowship routing, and the host boundary at `~/.pi/agent/`.

## Out of Scope

Third-party crates. Report those upstream.

## Cryptographic Primitives

- Ed25519 (RFC 8032) for signatures.
- SHA-256 (FIPS 180-4) for content addressing.
- No custom cryptography is implemented.
