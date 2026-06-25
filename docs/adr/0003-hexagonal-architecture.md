# ADR 0003 — Hexagonal Architecture (Ports & Adapters)

## Status

Accepted

## Context

Single Rust crate serving both HTML (HTMX) and future JSON endpoints. Multiple adapters needed (sqlx DB, tower-sessions, Axum HTTP). Domain logic must be testable without framework setup.

## Decision

Hexagonal architecture: `domain/` and `application/` have zero dependencies on axum, sqlx, or askama. Ports are traits defined in `domain/ports/`. Adapters in `adapters/inbound/http/` and `adapters/outbound/db/` implement those traits.

## Consequences

- Domain and use cases are unit-testable with mock adapters, no DB or HTTP setup required.
- Adding a new inbound adapter (CLI, cron job) or outbound adapter (Postgres → Redis) touches only the adapter layer.
- More upfront structure than a layered approach. Worth it because the port/adapter seams are exactly where the DB and framework swaps will happen.
