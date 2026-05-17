# Design — Unified Configuration Loader (Improved)

## Goals

- Strongly-typed, hierarchical configuration for safety and clarity.
- Deterministic precedence: defaults < file < environment.
- Clear, actionable errors and easy unit testing.
- Support for TOML/YAML files and optional hot-reload.

## Modules

- `partial` — `PartialConfig` with `Option` fields used to deserialize file/env layers.
- `defaults` — safe, conservative in-code defaults.
- `file` — optional file loader (TOML/YAML), detects format by extension.
- `env` — environment variable reader using `DC_` prefix.
- `merge` — pure, deterministic merge (overlay wins; `None` never clears `Some`).
- `validate` — builds final `Config` with strict validation rules.
- `error` — structured `ConfigError` for actionable messages.
- `hot_reload` — optional feature (behind `hot-reload`) implementing live reload via `notify`.

## Precedence

1. Defaults (lowest)
2. Configuration file (optional)
3. Environment variables (highest)

Environment variables always take precedence. File layer is optional and missing
files are ignored unless an explicit path is given.

## Error strategy

- Use `thiserror` to expose precise error variants (parse errors include source,
  missing keys include an `env` hint).
- No panics in library code; errors are returned via `Result`.

## Validation

- Required fields (e.g. `database.url`) must be present in at least one layer.
- Numeric fields are checked for sensible ranges (e.g. `port != 0`).

## Trade-offs

- Chose nested, typed config (Database/Server/Logging) for clarity and scalability.
- Maintained `PartialConfig` to keep file/env parsing uniform and merging trivial.
- Hot-reload is optional to avoid adding runtime dependencies unless requested.
