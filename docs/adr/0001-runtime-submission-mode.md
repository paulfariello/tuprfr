# ADR 0001 — Runtime Submission Mode Config

## Status

Accepted

## Context

The **Submission Mode** (`open` | `moderated`) controls whether new Questions are auto-published or queued for approval. The project needs to switch between modes without a redeploy — e.g., start open to seed content, then flip to moderated once volume picks up.

## Decision

Store Submission Mode in a `settings` table in the database. A protected admin endpoint allows flipping it at runtime.

## Consequences

- No redeploy needed to change mode.
- Requires a minimal admin API endpoint protected by auth. Auth must ship before or alongside the settings endpoint — it cannot be deferred past it.
- Avoids the accidental complexity of a separate config service or feature-flag system.
