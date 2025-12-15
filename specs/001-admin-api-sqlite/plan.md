# Implementation Plan: Admin Backend API with SQLite Logging

**Branch**: `001-admin-api-sqlite` | **Date**: 2025-12-15 | **Spec**: specs/001-admin-api-sqlite/spec.md
**Input**: Feature specification from `/specs/001-admin-api-sqlite/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement the full后台管理 API 集合（见 `specs/api.md`），使用 Rust + axum +
serde，SQLite 持久化。要求全链路日志：请求 ID、鉴权、响应、以及所有 SQL
语句/参数/影响行数。接口响应统一 code/message/data，JSON 字段 camelCase，CORS
允许所有 origin。所有接口必须具备成功/失败单测，`cargo check` 零警告。

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust (latest stable as of 2025-12-15)  
**Primary Dependencies**: axum, serde/serde_json, anyhow, thiserror, tokio, sqlx (SQLite), tower-http (CORS/logging), tracing/tracing-subscriber  
**Storage**: SQLite at `./.cms_backend/cms_backend.db` with migrations  
**Testing**: cargo test (unit per interface), possible http integration via axum TestServer pattern  
**Target Platform**: Linux-compatible server (CLI binary), HTTP API  
**Project Type**: single backend service (Rust)  
**Performance Goals**: p95 < 800ms for listed APIs in test env; logging overhead acceptable but bounded  
**Constraints**: CORS allow all origins; camelCase JSON; no unwrap/expect; deps pinned to 2025-12-15 versions  
**Scale/Scope**: Single-node SQLite; moderate traffic; full API surface per `specs/api.md`

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Rust + explicit typing: ✅ Planned Rust stable with strict annotations.
- axum + serde + anyhow/thiserror + conversion traits: ✅ axum router, serde JSON,
  conversion traits at boundaries.
- camelCase JSON: ✅ will configure serde rename + contract docs.
- Dependencies pinned to 2025-12-15: ✅ to be pinned in Cargo.toml.
- Error handling via `Result`/`?`, no unwrap/expect: ✅ enforced.
- Unit tests for success/failure per interface: ✅ planned in testing strategy.
- `cargo check` clean each stage: ✅ required gate.

## Project Structure

### Documentation (this feature)

```text
specs/001-admin-api-sqlite/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
└── tasks.md            # produced by /speckit.tasks
```

### Source Code (repository root)

```text
src/
├── main.rs
├── api/                # axum routers, handlers per domain
├── domain/             # domain models, DTOs, conversion traits
├── services/           # business logic
├── infra/              # sqlite pool, migrations, logging, cors
└── auth/               # token validation, middleware

tests/
├── unit/               # handler/service unit tests (success+failure)
└── integration/        # http-level flows using axum test server
```

**Structure Decision**: Single Rust backend project with layered modules; tests
separated into unit/integration.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| _None_ |  |  |
