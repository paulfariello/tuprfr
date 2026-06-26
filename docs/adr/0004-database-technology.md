# ADR 0004 — PostgreSQL as the Sole Database Backend

## Status

Accepted

## Context

sqlx's `query_as!` macro validates SQL against exactly one database backend at compile time. Using SQLite for tests and PostgreSQL for production would require maintaining two migration trees and risk dialect drift. The domain model uses UUIDs as primary keys, which PostgreSQL supports natively.

## Decision

PostgreSQL is the sole database backend — for development, tests, and production. Integration tests spin up a clean PostgreSQL instance via `testcontainers` + `testcontainers-modules`, dropped automatically at test teardown. Unit tests in the domain and application layers use mock port implementations and require no database.

SQLite is not used anywhere in this project.

## Consequences

- Single `migrations/` directory, one migration dialect, no backend-specific SQL branches.
- `query_as!` compile-time checking always validates against PostgreSQL. Offline mode (`.sqlx/` cache) allows compilation without a live DB in CI.
- UUID columns declared as `UUID` (native PostgreSQL type). `uuid::Uuid` in Rust via sqlx's `uuid` feature.
- UUID generation happens in Rust (`Uuid::new_v4()`), not in SQL, to keep query logic backend-agnostic.
- Integration tests require Docker. CI must have a Docker daemon available.
- `flake.nix` must include `postgresql` in dev packages.
