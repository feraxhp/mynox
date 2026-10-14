//! # Minox
//! 
//! from **minos (greek)**, related to mind and analisis, is a error library 
//! that aims to have clear error messages for what whent wrong and usefull tips for 
//! how to procede.
//! 

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


const NOTE_DIVIDER: &'static str = "## Notes";

#[cfg(test)]
mod tests {
    use super::*;
    use color_print::cformat;

    #[test]
    fn test_error_new() {
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
    fn test_macro_error_three_arguments() {
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
    fn test_macro_error_multiple_details() {
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
    fn test_macro_error_notes_comma_arrow() {
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
    fn test_macro_error_notes_no_comma_arrow() {
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
    fn test_macro_error_details_and_notes() {
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
    fn test_macro_error_with_cformat() {
        let formatted_detail = cformat!("<red>Detected invalid token:</> '<<EOF>>'");
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
