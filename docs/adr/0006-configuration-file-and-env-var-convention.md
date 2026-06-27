# ADR 0006 — Configuration File and Env Var Convention

## Status

Accepted

## Context

The app previously read a single `DATABASE_URL` env var. As operator-facing knobs grew (bind address, pool size, SSL mode, submission mode), we needed a structured configuration layer rather than an ad-hoc collection of env vars.

## Decision

Configuration is read from a TOML file (`tuprfr.toml` in the working directory) layered with env vars. Env vars win when both are set. The file path can be overridden with `TUPRFR_CONFIG`.

The TOML file is sectioned:

```toml
[database]
url = "postgresql://..."        # required, no default
pool_max_connections = 5        # default
ssl_mode = "prefer"             # default

[server]
bind_address = "0.0.0.0:3000"  # default
submission_mode = "moderated"   # default
```

Env vars follow the `TUPRFR_<SECTION>__<KEY>` convention (double underscore separates section from key, matching the layered config crate's nesting separator). Examples: `TUPRFR_DATABASE__URL`, `TUPRFR_SERVER__SUBMISSION_MODE`.

The previous `DATABASE_URL` env var is removed.

## Consequences

- Tests that previously set `DATABASE_URL` must switch to `TUPRFR_DATABASE__URL`. Tests with no config file rely entirely on env vars; they do not need a `tuprfr.toml`.
- The double-underscore nesting separator is non-obvious but is the standard convention for the `config` crate's env var source. It is the reason for this ADR.
- `database.url` is the only required value. A missing config file with all defaults set via env vars is valid. A config file with no `database.url` and no `TUPRFR_DATABASE__URL` is a startup error.
- Adding new knobs goes into the appropriate section; the precedence rules (env over file) apply uniformly without special cases.
