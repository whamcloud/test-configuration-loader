# Unified Configuration Loader Design

## Overview

This project implements a strongly-typed configuration loader in Rust that loads
configuration values from multiple sources and merges them into a single
validated configuration structure.

The loader supports three configuration sources:

1. Default values defined in code
2. Configuration file
3. Environment variables

The final configuration is returned using a strongly-typed Rust struct.

---

# Configuration Loading Flow

The configuration loading pipeline works in the following stages:

1. Load default configuration values from code
2. Parse configuration file (if present)
3. Read environment variables
4. Merge all sources
5. Validate the final configuration
6. Return the validated `Config`

The API exposed to applications is:

```rust
let config = Config::load()?;