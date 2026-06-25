# ADR 0002 — In-Memory Session Store for Game State

## Status

Accepted

## Context

The Game Loop requires tracking which Questions a Session has already Voted on or Skipped, to avoid re-showing them. This state is ephemeral — losing it on restart is acceptable for a game.

## Decision

Use `tower-sessions` with `MemoryStore`. Store seen Question IDs in the session. Votes are also written to the DB for count integrity. The session owns game flow state; the DB owns vote integrity.

## Consequences

- No extra DB join to filter seen Questions — IDs come from the session.
- State is lost on server restart (acceptable).
- Single-server only. Scaling horizontally requires swapping `MemoryStore` for a Redis-backed store — `tower-sessions` makes this a one-line change.
