# ADR 0005 — Two-Tier Database and Server Lifecycle for E2E Tests

## Status

Accepted

## Context

The Playwright E2E suite needs a running PostgreSQL instance and a running tuprfr server. Two properties pull in opposite directions:

- Bootstrapping PostgreSQL and applying the schema migration is slow and need happen only once per run.
- Each test needs an isolated database state and a fresh server. Game flow state lives in an in-memory `MemoryStore` (see [ADR 0002](0002-session-store.md)), so a Session leaks across tests unless the server restarts between them.

Playwright's `webServer` config cannot satisfy this. Playwright gates `webServer` readiness *before* it runs `globalSetup`, so a server started by `webServer` boots before `globalSetup` has created the database — it hangs on connect, times out, and `globalSetup` never runs. `webServer` also starts the server once for the whole run, which cannot reset per-test session state.

## Decision

Split the lifecycle across two tiers.

Global tier (`global-setup.ts`, once per run):

- Bootstrap the PostgreSQL container, create the database, apply the initial migration via `sqlx migrate run`, build the server binary.
- Teardown stops and removes the container.

Per-test tier (an auto fixture in `fixtures.ts`):

- Setup: clean the database, seed it, start the server, wait for it to listen.
- Teardown: stop the server, clean the database.

The server is not managed by Playwright's `webServer`; the fixture spawns and kills it. Tests run with `workers: 1` and `fullyParallel: false` — a shared database and a single server port make serial execution mandatory.

Because the migration runs once in the global tier, the server's startup `sqlx::migrate!()` finds `_sqlx_migrations` already populated and skips, so no migration runs twice.

## Consequences

- Container bootstrap and migration cost is paid once; seed, clean, and server restart are paid per test.
- Each test gets a clean database and a fresh `MemoryStore`, so no Session, Vote, or seen-Question state leaks between tests.
- The suite runs serially. Wall-clock time scales linearly with test count; parallelism would require a database and server port per worker.
- The server connects over `127.0.0.1`, not `localhost` — `/etc/hosts` resolves `localhost` to `::1` only, while the container publishes the port on IPv4.
- `sqlx-cli` is a dev-shell dependency, used by the global tier to apply the migration.
