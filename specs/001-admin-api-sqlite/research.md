# Research: Admin Backend API with SQLite Logging

## Decisions

- **Storage**: SQLite at `./.cms_backend/cms_backend.db` with migrations; suits single-node scope and low ops overhead.
- **Web stack**: axum + serde; aligns with constitution and supports typed handlers.
- **Error handling**: anyhow for context propagation, thiserror for typed domain errors; no unwrap/expect.
- **Logging/Tracing**: tracing + tower-http HTTP trace; include request ID, method/path/status/latency; SQL logged via sqlx with statements, params, rows/affected.
- **CORS**: allow all origins (per instruction) via tower-http CORS layer.
- **JSON style**: camelCase via serde rename.
- **Testing**: cargo test with unit coverage for handlers/services success+failure; integration via axum TestServer; ensure cargo check clean.
- **Performance target**: p95 < 800ms in test env; logging overhead acceptable but monitored.
- **Dependencies**: pin to latest stable as of 2025-12-15.

## Rationale

- SQLite fits requirement and simplifies deployment; migrations ensure schema parity with contracts.
- axum/serde are idiomatic Rust web choices and match constitution.
- tracing stack provides structured logs; SQL visibility satisfies audit/log mandates.
- CORS all-origins meets instruction and reduces front-end setup friction.
- Typed errors + `Result` flow uphold no-panic rule and testability.

## Alternatives Considered

- Postgres: richer concurrency but exceeds stated scope and ops needs.
- Actix-web / warp: viable, but axum chosen for simplicity and constitution alignment.
- Diesel ORM: strong typing, but sqlx preferred for async and direct SQL logging control.

## Remaining Questions

- None (all inputs specified).

