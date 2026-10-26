//! # Mynox
//!
//! **Mynox** (from *minos*, related to mind and analysis) is an error handling and diagnostic
//! library for Rust designed to produce clear, actionable, and visually distinct error reports.
//!
//! Rather than presenting users with opaque error strings or raw backtraces, Mynox structures
//! diagnostics into three clear levels of information:
//!
//! 1. **Message**: A high-level, human-readable summary of what operation failed.
//! 2. **Details**: Precise diagnostic facts explaining why the failure occurred.
//! 3. **Notes**: Actionable suggestions, tips, or shell commands guiding the user on how to resolve the issue.
//!
//! ## Core Features
//!
//! - **Expressive Macro**: Create rich errors in one line using [`error!`].
//! - **Actionable Notes**: Separate facts from remedies using the `=>` operator in the macro.
//! - **Causal Chains**: Preserve underlying root-cause errors via `source:` compatible with [`std::error::Error::source`].
//! - **Context Propagation**: Ergonomically convert any foreign `Result<T, E>` with the [`Context`] trait and `?`.
//! - **Progressive Enrichment**: Add details and notes at each level of your stack using the [`Extend`] trait.
//! - **Hierarchical Error Types**: Query errors cleanly with [`Error::is`] and [`Error::is_category`] (e.g. `"io::fs"`).
//! - **Terminal-Aware Styling**: Colored output with markdown formatting (`` `code` ``, `*italic*`, `**bold**`),
//!   respecting `NO_COLOR` and terminal detection.
//! - **Idiomatic `main()` Return**: Wrap errors in [`Report`] to output formatted diagnostics on application exit.
//!
//! ## Quick Start
//!
//! ```rust
//! use mynox::{error, Report};
//!
//! fn read_config(path: &str) -> Result<String, mynox::Error> {
//!     if !std::path::Path::new(path).exists() {
//!         return Err(error!(
//!             "io::fs::not_found",
//!             "Failed to read application configuration",
//!             "The specified path does not exist on disk",
//!             "Default search directories (/etc, ~/.config) were also exhausted"
//!             =>
//!             "Create the configuration file at the requested path",
//!             "Or run with '--init' to generate a default config file"
//!         ));
//!     }
//!     Ok("config content".into())
//! }
//!
//! fn main() -> Result<(), Report> {
//!     let config = read_config("missing.toml").map_err(Report::from);
//!     assert!(config.is_err());
//!     Ok(())
//! }
//! ```
//!
//! ## Propagating Errors with `Context`
//!
//! Wrap external errors like [`std::io::Error`] into `mynox::Error` while preserving the root cause:
//!
//! ```rust
//! use mynox::Context;
//! use std::fs::File;
//!
//! fn open_file(path: &str) -> Result<File, mynox::Error> {
//!     File::open(path).context("io::fs::open", format!("Could not open '{path}'"))
//! }
//!
//! assert!(open_file("/nonexistent/file").is_err());
//! ```
//!
//! ## Progressive Enrichment with `Extend`
//!
//! Enrich errors as they bubble up through the call stack:
//!
//! ```rust
//! use mynox::{error, Extend};
//!
//! fn query_db() -> Result<(), mynox::Error> {
//!     Err(error!("db::timeout", "Database query failed", "Connection timed out after 5000ms"))
//! }
//!
//! fn execute_operation() -> Result<(), mynox::Error> {
//!     query_db()
//!         .detail("Operation: FetchUserProfile(user_id=42)")
//!         .note("Check database connectivity and network latency")
//! }
//!
//! let err = execute_operation().unwrap_err();
//! assert_eq!(err.details().len(), 2);
//! assert_eq!(err.notes().len(), 1);
//! ```
//!
//! ## Hierarchical Categories
//!
//! ```rust
//! use mynox::error;
//!
//! let err = error!("net::http::tls_handshake", "Connection aborted", "Certificate expired");
//!
//! assert!(err.is_category("net"));
//! assert!(err.is_category("net::http"));
//! assert!(err.is_category("net::http::tls_handshake"));
//! assert!(!err.is_category("io"));
//! ```

pub mod formater;
pub mod structs;
pub mod _impls;
pub mod macros;
pub mod utils;
mod termianal;
mod traits;

pub use self::structs::Error;
pub use self::structs::Report;

pub use self::traits::Show;
pub use self::traits::Extend;
pub use self::traits::Context;
pub use self::traits::Formater;


const NOTE_DIVIDER: &'static str = "# Notes";

#[cfg(test)]
mod tests {
    use crate::formater::parser::Colors;
    
    use super::*;
    
    #[test]
    fn error_new() {
        let error = Error::new(
            "grp::parsing::url", 
            "Error parsing a url", 
            "Invalid character '^' at position 15", 
            empty_iter!(), 
            empty_iter!(),
        );
        assert_eq!(error.message(), "Error parsing a url");
        assert_eq!(error.etype(), "grp::parsing::url");
        assert_eq!(error.details(), vec!["Invalid character '^' at position 15"]);
        assert!(error.notes().is_empty());
        assert!(error.is("grp::parsing::url"));
    }

    #[test]
    fn macro_error_three_arguments() {
        let error = error!(
            "io::fs::file_not_found",
            "Unable to locate configuration file",
            "Path '/etc/mynox/config.toml' does not exist on disk"
        );
        assert_eq!(error.etype(), "io::fs::file_not_found");
        assert_eq!(error.message(), "Unable to locate configuration file");
        assert_eq!(
            error.details(),
            vec!["Path '/etc/mynox/config.toml' does not exist on disk"]
        );
        assert!(error.notes().is_empty());
    }

    #[test]
    fn macro_error_multiple_details() {
        let error = error!(
            "syntax::json::unexpected_token",
            "Failed to deserialize request payload",
            "Unexpected token ',' at line 4, column 18",
            "Expected a valid JSON value or closing bracket",
            "Found trailing comma after object member",
        );
        assert_eq!(error.etype(), "syntax::json::unexpected_token");
        assert_eq!(error.message(), "Failed to deserialize request payload");
        assert_eq!(
            error.details(),
            vec![
                "Unexpected token ',' at line 4, column 18",
                "Expected a valid JSON value or closing bracket",
                "Found trailing comma after object member",
            ]
        );
        assert!(error.notes().is_empty());
    }

    #[test]
    fn macro_error_notes_comma_arrow() {
        let error = error!(
            "net::http::invalid_port",
            "Failed to bind server to specified address",
            "Port 70000 is outside the valid TCP range (1-65535)",
            =>
            "Try using a standard unprivileged port such as 8080 or 3000",
            "Check your PORT environment variable in your .env configuration",
        );
        assert_eq!(error.etype(), "net::http::invalid_port");
        assert_eq!(error.message(), "Failed to bind server to specified address");
        assert_eq!(
            error.details(),
            vec!["Port 70000 is outside the valid TCP range (1-65535)"]
        );
        assert_eq!(
            error.notes(),
            vec![
                "Try using a standard unprivileged port such as 8080 or 3000",
                "Check your PORT environment variable in your .env configuration",
            ]
        );
    }

    #[test]
    fn macro_error_notes_no_comma_arrow() {
        let error = error!(
            "db::pool::timeout",
            "Database connection pool exhausted",
            "All 20 connections are currently busy after waiting 5000ms"
            =>
            "Try increasing the 'max_connections' pool setting in database.toml",
            "Ensure that previous database transactions are committed or dropped promptly",
        );
        assert_eq!(error.etype(), "db::pool::timeout");
        assert_eq!(error.message(), "Database connection pool exhausted");
        assert_eq!(
            error.details(),
            vec!["All 20 connections are currently busy after waiting 5000ms"]
        );
        assert_eq!(
            error.notes(),
            vec![
                "Try increasing the 'max_connections' pool setting in database.toml",
                "Ensure that previous database transactions are committed or dropped promptly",
            ]
        );
    }

    #[test]
    fn macro_error_details_and_notes() {
        let error = error!(
            "grp::parsing::url",
            "Error parsing a url",
            "Invalid character '^' at position 15",
            "Scheme parsed as 'https://' successfully",
            "Host segment contains illegal character '^'",
            "Parser aborted before query parameters could be evaluated"
            =>
            "Try removing the character '^' at position 15",
            "Ensure the URL adheres to standard RFC 3986 format",
        );
        assert_eq!(error.etype(), "grp::parsing::url");
        assert_eq!(error.message(), "Error parsing a url");
        assert_eq!(
            error.details(),
            vec![
                "Invalid character '^' at position 15",
                "Scheme parsed as 'https://' successfully",
                "Host segment contains illegal character '^'",
                "Parser aborted before query parameters could be evaluated",
            ]
        );
        assert_eq!(
            error.notes(),
            vec![
                "Try removing the character '^' at position 15",
                "Ensure the URL adheres to standard RFC 3986 format",
            ]
        );
    }

    #[test]
    fn macro_error_with_cformat() {
        let formatted_detail = format!("{}Detected invalid token: {}'<<EOF>>'", Colors::Red, Colors::Reset);
        let error = error!(
            "auth::token::expired",
            "Authentication token has expired",
            "Token claim 'exp' is earlier than current timestamp",
            formatted_detail.clone(),
            =>
            "Try requesting a new JWT token using your refresh token",
            "Re-authenticate if the session has completely lapsed",
        );
        assert_eq!(error.etype(), "auth::token::expired");
        assert_eq!(error.message(), "Authentication token has expired");
        assert_eq!(
            error.details(),
            vec![
                "Token claim 'exp' is earlier than current timestamp",
                &formatted_detail,
            ]
        );
        assert_eq!(
            error.notes(),
            vec![
                "Try requesting a new JWT token using your refresh token",
                "Re-authenticate if the session has completely lapsed",
            ]
        );
    }
}
