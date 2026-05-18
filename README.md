# Unified Configuration Loader

## Summary

You have been tasked with building a **robust, strongly-typed configuration
loader** for a Rust application.

Real-world applications often need to load configuration values from multiple
sources. Your goal is to design a clean and predictable configuration system
that merges values from different sources while providing clear error handling
and validation.

## Configuration Sources

Configuration values may come from the following sources:

* Default values defined in code
* Environment variables
* A configuration file

## Requirements

### Core Functionality

* Provide a unified API for loading configuration:
  ```rust
  let config = Config::load()?;
  ```
* Configuration must be strongly typed
* Define clear and deterministic precedence rules between configuration sources
* Provide meaningful and actionable error messages

## Constraints

* No use of unwrap() or expect()
* Errors must be propagated using Result
* Avoid global mutable state
* The configuration loader must be easy to test without relying on external system state

## Validation

* Configuration values should be validated where appropriate
* Invalid or missing required configuration should result in a clear error
* Partial configuration should not silently fall back to invalid defaults

## Additional Requirements

* Your source should contain unit tests for configuration loading and validation
* All code must be formatted using the standard formatting tool
* Code must compile without clippy errors
* The solution must use safe Rust only

## Design & Reasoning (Required)

* Along with the code, include a document (for example DESIGN.md) explaining:
* How configuration sources are loaded and merged
* Precedence rules between defaults, environment variables, and files
* Error modeling and reporting strategy
* How the configuration is validated
* Trade-offs made in the design

Submissions without a design explanation will not be reviewed.

## Submission

Please fork this repository to your own GitHub account and submit a pull request
to your own repository.

Your pull request should include:
* A clear description of your approach
* Any assumptions or trade-offs made
* Instructions on how to run tests

A link to the pull request can be submitted once it is ready for review.

## Bonus

* Support for multiple configuration file formats (e.g. TOML, YAML)
* Hot-reload support for configuration changes
* Partial configuration validation
* Schema or documentation generation

## How to run

Run tests and checks locally:

```bash
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo test -- --test-threads=1
```

Example run (requires `DC_DATABASE_URL` or a config file with `[database].url`):

```bash
DC_DATABASE_URL=postgres://localhost/demo cargo run --example basic
```

