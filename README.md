# Mynox

[![Crates.io](https://img.shields.io/crates/v/mynox.svg)](https://crates.io/crates/mynox)
[![Documentation](https://docs.rs/mynox/badge.svg)](https://docs.rs/mynox)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENCE)

> **Mynox** (from *minos* [Greek], relating to mind and analysis) is an error-handling and diagnostic library for Rust designed to produce clear, actionable, and visually distinct error reports.

---

## Overview

Traditional error handling often presents developers and users with cryptic one-line errors or impenetrable stack traces. **Mynox** structures failures into three distinct, human-centered levels:

1. **Message**: A high-level, human-readable summary of what operation failed.
2. **Details**: Specific diagnostic facts explaining why the failure occurred.
3. **Notes**: Actionable suggestions, tips, or shell commands guiding the user on how to resolve the problem.

```text
* Failed to bind server to specified address
  Port 80 requires elevated permissions
  Interface 0.0.0.0 is already bound by another process

## Notes
✓ Run with elevated privileges (sudo)
  • sudo ./server --port 80
✓ Or choose an unprivileged port (e.g. 8080)
```

## Features

- **Concise `error!` Macro**: Construct rich errors in a single line with `=>` to separate diagnostic facts from actionable tips.
- **Causal Error Chains**: Preserve root-cause errors with `source: <expr>`, fully compatible with `std::error::Error::source`.
- **`Context` Propagation**: Convert foreign errors in `Result<T, E>` using `?` while attaching domain codes and messages.
- **Progressive `Extend` Trait**: Incrementally attach details and notes as errors bubble up through application layers.
- **Hierarchical Categorization**: Classify errors by taxonomy (e.g., `"net::http::bind"`) and match categories with `is_category("net::http")`.
- **Terminal Styling & NO_COLOR**: Automatic ANSI color detection, inline markdown support (`` `code` ``, `**bold**`, `*italic*`), and compliance with the `NO_COLOR` standard.
- **Application Exit via `Report`**: Clean integration with `fn main() -> Result<(), Report>` for beautiful CLI error output.

---

## Installation

Add `mynox` to your `Cargo.toml`:

```toml
[dependencies]
mynox = "0.1"
```

---

## Quick Start

### 1. Returning errors from `main()` with `Report`

```rust
use mynox::{error, Report};

fn run() -> Result<(), mynox::Error> {
    let port = 80;
    if port < 1024 {
        return Err(error!(
            "net::http::privileged_port",
            "Failed to start server",
            format!("Port {port} is restricted to privileged users")
            =>
            "Run the application with 'sudo'",
            "Or configure an unprivileged port such as 8080 in 'config.toml'"
        ));
    }
    Ok(())
}

fn main() -> Result<(), Report> {
    run().map_err(Report::from)
}
```

When run, `Report` renders the error automatically to `stderr` with appropriate ANSI terminal colors and clear note separators.

---

## Core Concepts

### The `error!` Macro

The `error!` macro provides flexible syntax variations to suit any diagnostic requirement:

#### 1. Basic 3-argument syntax
```rust
use mynox::error;

let err = error!(
    "io::fs::not_found",
    "Configuration file not found",
    "Path '/etc/myapp/config.toml' does not exist on disk"
);
```

#### 2. Multiple details
```rust
use mynox::error;

let err = error!(
    "syntax::json::unexpected_token",
    "Failed to parse configuration payload",
    "Unexpected token ',' at line 4, column 18",
    "Expected string key or closing curly brace"
);
```

#### 3. Actionable notes with `=>`
```rust
use mynox::error;

let err = error!(
    "db::pool::exhausted",
    "Database connection pool exhausted",
    "All 20 pooled connections were busy after waiting 5000ms"
    =>
    "Increase 'max_connections' in your database configuration",
    "Verify that transactions are properly committed and closed"
);
```

#### 4. Preserving causal sources (`source:`)
```rust
use std::io;
use mynox::error;

let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");

let err = error!(
    source: io_err,
    "io::fs::access_denied",
    "Failed to open certificates file",
    "Process running under non-root UID 1000"
    =>
    "Adjust file permissions with 'chmod 600 cert.pem'"
);

// The causal source is preserved and accessible:
assert!(std::error::Error::source(&err).is_some());
```

---

### Propagating Foreign Errors with `Context`

The `Context` trait simplifies adapting external errors (such as `std::io::Error`) with the `?` operator:

```rust
use mynox::Context;
use std::fs::File;

fn load_credentials(path: &str) -> Result<File, mynox::Error> {
    File::open(path)
        .context("io::fs::read", format!("Could not read credential file at '{path}'"))
}
```

Three variants are available:
- **`context`**: For types implementing `std::error::Error + Send + Sync + 'static` (preserves the source).
- **`context_display`**: For types implementing only `Display`.
- **`context_raw`**: For unconstrained types, accepting a custom message and detail.

---

### Progressive Enrichment with `Extend`

As errors propagate up your call stack, you can add extra context without losing the original error definition:

```rust
use mynox::{error, Extend};

fn fetch_user(id: u64) -> Result<(), mynox::Error> {
    Err(error!("db::query", "Query execution failed", "Connection timed out"))
}

fn handle_request(user_id: u64) -> Result<(), mynox::Error> {
    fetch_user(user_id)
        .detail(format!("While handling request for user ID: {user_id}"))
        .note("Check network latency to the primary database replica")
}
```

Available methods:
- `.detail(...)` / `.details(...)` / `.with_details(|| ...)`
- `.note(...)` / `.notes(...)` / `.with_notes(|| ...)`

---

### Hierarchical Categories

Error types (`etype`) use `::` as category separators, enabling flexible filtering:

```rust
use mynox::error;

let err = error!("io::fs::not_found", "File missing", "Path does not exist");

// Exact match
assert!(err.is("io::fs::not_found"));

// Category hierarchy
assert!(err.is_category("io"));
assert!(err.is_category("io::fs"));
assert!(!err.is_category("net"));
```

---

### Terminal Rendering & Markdown Formatting

Mynox automatically formats error descriptions using inline markdown and ANSI styles:

- `` `code` `` rendered with highlighted magenta styling.
- `**bold**` rendered in blue.
- `*italic*` rendered in italics.

#### Automatic Color Detection

- If `stderr` is not a TTY (e.g. redirected to a file: `app 2> err.log`), plain uncolored text is emitted.
- If the `NO_COLOR` environment variable is set (any non-empty value), color output is suppressed.
- If `CLICOLOR_FORCE` is set, color is forced on.

#### Manual Rendering

```rust
use mynox::error;

let err = error!("auth::expired", "Session expired", "Token is no longer valid");

// Plain lines iterator
for line in err.render_plain() {
    println!("{line}");
}

// ANSI colored lines iterator
for line in err.render_color() {
    println!("{line}");
}

// Auto-detect based on stdout or stderr terminal
let _stdout_lines = err.render_std_out_auto();
let _stderr_lines = err.render_std_err_auto();
```

---

## Comparison: `Error` vs `Report`

| Feature | `Error` | `Report` |
|:---|:---|:---|
| **Role** | Core error data container | Diagnostic reporting wrapper |
| **`Display` (`{}`)** | Prints only the high-level `message` | Prints the full diagnostic report (`{:#}`) |
| **`Display` (`{:#}`)** | Prints the full plain report | Prints the full plain report |
| **`Debug` (`{:?}`)** | Standard Rust debug representation | Renders styled report to `stderr` (auto-color) |
| **Typical Usage** | Function signatures, error matching | `fn main() -> Result<(), Report>` |

---

## License

This project is licensed under the [MIT License](LICENCE).
