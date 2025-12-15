<!--
Sync Impact Report
- Version change: none → 1.0.0
- Modified principles: initialized
- Added sections: Core Principles, Technology Constraints, Workflow & Quality Gates, Governance
- Removed sections: none
- Templates updated: plan-template.md ✅; spec-template.md ⚠️ not needed; tasks-template.md ⚠️ not needed; agent-file-template.md ⚠️ not needed; checklist-template.md ⚠️ not needed; commands templates ⚠️ not present
- Follow-up TODOs: none
-->

# cms-backend Constitution

## Core Principles

### I. Rust Axum/Serde with Strict Typing
All backend services MUST use Rust with axum for HTTP routing and serde for
serialization. Code MUST carry explicit types end-to-end, including request/response
DTOs. JSON produced by the backend MUST be camelCase.

### II. Safe Error Handling (No unwrap/expect)
`unwrap`/`expect` are forbidden in production paths. Errors MUST propagate with `?`
and be normalized via anyhow + thiserror (or equivalent) into consistent HTTP
responses. Centralized handlers SHOULD map errors to API-friendly payloads.

### III. Trait-Based Conversions for Data Shapes
Cross-layer data transformations (DB ↔ domain ↔ DTO) SHOULD implement standard
traits (From/TryFrom/TryInto/FromStr/Into) to ensure explicit, testable conversion
paths and reduce ad-hoc mapping bugs.

### IV. Dependency Freshness
Dependencies MUST target the latest available versions as of 2025-12-15. When
adding/updating crates, prefer the newest stable release and document notable
compatibility implications.

### V. Testing Discipline & CI Gates
Every API/endpoint MUST have unit tests. Run `cargo check` (and fix warnings) at
each development stage. Favor TDD where practical; no feature is complete without
tests covering success and failure paths.

## Technology Constraints

- Language/stack: Rust + axum + serde; error handling via anyhow/thiserror.
- JSON contract: camelCase keys for all backend-generated payloads.
- Error policy: propagate with `?`; forbid `unwrap`/`expect` in production code.
- Conversions: prefer trait implementations for data shape transitions.
- Dependencies: track and adopt latest stable crate versions (as of 2025-12-15).

## Development Workflow & Quality Gates

- Run `cargo check` before committing and at every phase gate.
- Add/maintain unit tests for each API/endpoint; include failure-path coverage.
- Ensure request/response DTOs are fully typed and camelCase serialized.
- Reviews must verify adherence to Principles I–V and that no `unwrap`/`expect`
  slipped into production paths.

## Governance

- This constitution governs backend development for cms-backend.
- Compliance is required for all PRs; reviewers must block if principles are
  violated.
- Amendments require documentation of changes, rationale, and a semantic version
  bump below.
- Versioning: MAJOR for breaking/removing principles; MINOR for new principles or
  material expansions; PATCH for clarifications.
- Ratification date is the original adoption; Last Amended is updated on change.

**Version**: 1.0.0 | **Ratified**: 2025-12-15 | **Last Amended**: 2025-12-15
