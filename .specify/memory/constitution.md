<!--
Sync Impact Report:
- Version: 0.0.0 → 1.0.0
- Modified principles: n/a (initial definition)
- Added sections: Core Principles, Platform Constraints, Workflow Quality Gates, Governance
- Removed sections: none
- Templates updated: ✅ .specify/templates/plan-template.md | ✅ .specify/templates/spec-template.md | ✅ .specify/templates/tasks-template.md
- Follow-up TODOs: None
-->

# CBS Backend Constitution

## Core Principles

### Rust-First with Explicit Typing
All backend code MUST be written in Rust with strict, explicit type annotations.
Public surfaces (APIs, modules, traits) MUST avoid implicit type inference that
could weaken contracts. Unsafe code is prohibited unless explicitly justified
and reviewed.

### Axum + Serde Stack with Traitful Conversions
Web interfaces MUST use axum for routing and serde for (de)serialization.
Error handling MUST use anyhow for fallible flows and thiserror for structured
error types. Data transformations MUST prefer standard conversion traits (From,
TryFrom, Into, FromStr) to keep boundaries well-typed and testable.

### CamelCase JSON Contracts
All JSON payloads produced by the backend MUST use camelCase keys. Request and
response schemas MUST be documented and validated to ensure contract stability.

### Current Dependencies as of 2025-12-15
All dependencies MUST be pinned to the latest stable versions available on
2025-12-15. New dependencies added after this date MUST be reviewed for
compatibility and explicitly recorded in changelogs.

### Safe Error Propagation and Mandatory Tests
`unwrap` and `expect` MUST NOT be used; prefer `?` for propagation and handle
errors centrally. Every interface (endpoint, handler, service function) MUST
have unit tests that cover success and failure paths.

## Platform Constraints

- Rust web stack uses axum + serde with anyhow/thiserror for error management.
- JSON contracts are camelCase; non-conforming payloads are violations.
- Dependency updates after 2025-12-15 require explicit review and documentation.

## Workflow Quality Gates

- Every development stage MUST run `cargo check` with zero warnings before
merging or release.
- Each new or changed interface MUST add or update unit tests; PRs lacking tests
are blocked.
- Error handling code MUST demonstrate propagation via `Result` rather than
panic-based flows.

## Governance

- This constitution supersedes other process documents for backend work.
- Amendments require written rationale, reviewer approval, and a migration plan
when breaking changes are introduced.
- Versioning follows semantic rules: major for principle removals/breaking
governance, minor for new principles or expanded guidance, patch for
clarifications.
- Compliance is reviewed in PRs; violations require documented justification in
the change description and must be resolved before release.

**Version**: 1.0.0 | **Ratified**: 2025-12-15 | **Last Amended**: 2025-12-15
