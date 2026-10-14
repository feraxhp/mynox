use mynox::{empty_iter, error, Error};
use std::collections::HashSet;
use std::io;

#[test]
fn test_error_new_with_empty_iter() {
    let error = Error::new(
        "grp::parsing::url",
        "Error parsing a url",
        "Invalid character '^' at position 15",
        empty_iter!(),
        empty_iter!(),
    );

    assert_eq!(error.etype(), "grp::parsing::url");
    assert_eq!(error.message(), "Error parsing a url");
    assert_eq!(error.details(), vec!["Invalid character '^' at position 15"]);
    assert!(error.notes().is_empty());
    assert!(error.is("grp::parsing::url"));
    assert!(!error.is("grp::parsing::json"));
    assert!(std::error::Error::source(&error).is_none());
}

#[test]
fn test_macro_error_three_arguments() {
    let error = error!(
        "grp::fs::read",
        "Failed to read application configuration",
        "Path '/etc/myapp/config.yaml' does not exist"
    );

    assert_eq!(error.etype(), "grp::fs::read");
    assert_eq!(error.message(), "Failed to read application configuration");
    assert_eq!(
        error.details(),
        vec!["Path '/etc/myapp/config.yaml' does not exist"]
    );
    assert!(error.notes().is_empty());
    assert!(error.is("grp::fs::read"));
}

#[test]
fn test_macro_error_with_multiple_details() {
    let error = error!(
        "grp::parsing::json",
        "Syntax error while parsing JSON payload",
        "Unexpected token ',' at line 8, column 12",
        "Expected object key or closing curly brace '}'",
        "Trailing commas are forbidden by JSON specification",
    );

    assert_eq!(error.etype(), "grp::parsing::json");
    assert_eq!(
        error.message(),
        "Syntax error while parsing JSON payload"
    );
    assert_eq!(
        error.details(),
        vec![
            "Unexpected token ',' at line 8, column 12",
            "Expected object key or closing curly brace '}'",
            "Trailing commas are forbidden by JSON specification",
        ]
    );
    assert!(error.notes().is_empty());
}

#[test]
fn test_macro_error_notes_with_comma_arrow() {
    let error = error!(
        "grp::net::bind",
        "Could not bind listener to address",
        "Port 80 is privileged and requires elevated permissions",
        =>
        "Try running the application with sudo or root privileges",
        "Try using an unprivileged port like 8080 or 3000 instead",
    );

    assert_eq!(error.etype(), "grp::net::bind");
    assert_eq!(error.message(), "Could not bind listener to address");
    assert_eq!(
        error.details(),
        vec!["Port 80 is privileged and requires elevated permissions"]
    );
    assert_eq!(
        error.notes(),
        vec![
            "Try running the application with sudo or root privileges",
            "Try using an unprivileged port like 8080 or 3000 instead",
        ]
    );
}

#[test]
fn test_macro_error_notes_without_comma_arrow() {
    let error = error!(
        "grp::db::pool",
        "Database connection pool exhausted",
        "Timed out waiting 3000ms for an available connection from pool 'main'",
        =>
        "Try increasing the 'max_connections' limit in database.toml",
        "Ensure active transactions are released after query execution",
    );

    assert_eq!(error.etype(), "grp::db::pool");
    assert_eq!(error.message(), "Database connection pool exhausted");
    assert_eq!(
        error.details(),
        vec!["Timed out waiting 3000ms for an available connection from pool 'main'"]
    );
    assert_eq!(
        error.notes(),
        vec![
            "Try increasing the 'max_connections' limit in database.toml",
            "Ensure active transactions are released after query execution",
        ]
    );
}

#[test]
fn test_macro_error_both_details_and_notes() {
    let error = error!(
        "grp::parsing::url",
        "Error parsing a url",
        "Invalid character '^' at position 15",
        "Scheme parsed as 'https://' successfully",
        "Host segment contains illegal character '^'",
        "Parser halted prior to query string evaluation"
        =>
        "Try removing the character '^' at position 15",
        "Ensure hostname adheres to RFC 1035 domain label syntax",
    );

    assert_eq!(error.etype(), "grp::parsing::url");
    assert_eq!(error.message(), "Error parsing a url");
    assert_eq!(
        error.details(),
        vec![
            "Invalid character '^' at position 15",
            "Scheme parsed as 'https://' successfully",
            "Host segment contains illegal character '^'",
            "Parser halted prior to query string evaluation",
        ]
    );
    assert_eq!(
        error.notes(),
        vec![
            "Try removing the character '^' at position 15",
            "Ensure hostname adheres to RFC 1035 domain label syntax",
        ]
    );
}

#[test]
fn test_macro_error_with_cformat_expression() {
    let formatted_detail = format!("\x1b[31mDetected token:\x1b[0m '<<EOF>>'");
    let error = error!(
        "grp::auth::jwt",
        "JWT token validation failed",
        "Signature check returned mismatch against public key",
        formatted_detail.clone(),
        =>
        "Try refreshing the signing keys cache from JWKS endpoint",
        "Verify that the Authorization header contains a Bearer token",
    );

    assert_eq!(error.etype(), "grp::auth::jwt");
    assert_eq!(error.message(), "JWT token validation failed");
    assert_eq!(
        error.details(),
        vec![
            "Signature check returned mismatch against public key",
            &formatted_detail,
        ]
    );
    assert_eq!(
        error.notes(),
        vec![
            "Try refreshing the signing keys cache from JWKS endpoint",
            "Verify that the Authorization header contains a Bearer token",
        ]
    );
}

#[test]
fn test_error_display_and_equality() {
    let err1 = error!(
        "grp::parsing::url",
        "Error parsing a url",
        "Invalid character '@' in hostname"
    );
    let err2 = error!(
        "grp::parsing::url",
        "Another URL parse failure",
        "Missing port digits after ':'"
    );
    let err3 = error!(
        "grp::parsing::json",
        "Error parsing a url",
        "Invalid character '@' in hostname"
    );

    // Display implementation outputs the principal error message
    assert_eq!(format!("{}", err1), "Error parsing a url");

    // PartialEq is based on etype
    assert_eq!(err1, err2);
    assert_ne!(err1, err3);
}

#[test]
fn test_error_from_source() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
    let error = Error::from(
        io_err,
        "grp::io::permission",
        "Failed to write configuration file",
        "Access denied when opening '/etc/myapp/config.yaml'",
        ["Process runs under non-root UID 1000"],
        ["Try running with elevated privileges (sudo)", "Check directory permissions with 'ls -la'"],
    );

    assert_eq!(error.etype(), "grp::io::permission");
    assert_eq!(error.message(), "Failed to write configuration file");
    assert_eq!(
        error.details(),
        vec![
            "Access denied when opening '/etc/myapp/config.yaml'",
            "Process runs under non-root UID 1000"
        ]
    );
    assert_eq!(
        error.notes(),
        vec![
            "Try running with elevated privileges (sudo)",
            "Check directory permissions with 'ls -la'"
        ]
    );

    let src = std::error::Error::source(&error);
    assert!(src.is_some());
    assert_eq!(src.unwrap().to_string(), "permission denied");
}

#[test]
fn test_error_hash_and_hashset() {
    let err1 = error!(
        "grp::auth::unauthorized",
        "Token expired",
        "Expired at timestamp 123456"
    );
    let err2 = error!(
        "grp::auth::unauthorized",
        "Different message",
        "Different detail"
    );
    let err3 = error!(
        "grp::auth::forbidden",
        "Access denied",
        "User lacks admin role"
    );

    let mut set = HashSet::new();
    set.insert(err1.clone());
    // Since err2 has the same etype as err1, inserting err2 should not increase the set size
    set.insert(err2);
    assert_eq!(set.len(), 1);
    assert!(set.contains(&err1));

    set.insert(err3.clone());
    assert_eq!(set.len(), 2);
    assert!(set.contains(&err3));
}

#[test]
fn test_error_ordering() {
    let err_b = error!("b::operation::failed", "Error B", "Detail B");
    let err_a = error!("a::operation::failed", "Error A", "Detail A");
    let err_c = error!("c::operation::failed", "Error C", "Detail C");

    assert!(err_a < err_b);
    assert!(err_b < err_c);
    assert!(err_a <= err_b);
    assert!(err_c >= err_b);

    let mut list = vec![err_c.clone(), err_a.clone(), err_b.clone()];
    list.sort();

    assert_eq!(list[0].etype(), "a::operation::failed");
    assert_eq!(list[1].etype(), "b::operation::failed");
    assert_eq!(list[2].etype(), "c::operation::failed");
}

#[test]
fn test_error_as_ref() {
    let error = error!(
        "grp::io::not_found",
        "File missing",
        "Path does not exist"
    );

    let as_ref_str: &str = error.as_ref();
    assert_eq!(as_ref_str, "File missing");
    assert_eq!(as_ref_str, error.message());
}

#[test]
fn test_macro_error_with_source_three_arguments() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "entity not found");
    let error = error!(
        source: io_err,
        "grp::fs::read",
        "Failed to read application configuration",
        "Path '/etc/myapp/config.yaml' does not exist"
    );

    assert_eq!(error.etype(), "grp::fs::read");
    assert_eq!(error.message(), "Failed to read application configuration");
    assert_eq!(
        error.details(),
        vec!["Path '/etc/myapp/config.yaml' does not exist"]
    );
    assert!(error.notes().is_empty());
    assert!(error.is("grp::fs::read"));

    let src = std::error::Error::source(&error);
    assert!(src.is_some());
    assert_eq!(src.unwrap().to_string(), "entity not found");
}

#[test]
fn test_macro_error_with_source_and_multiple_details() {
    let io_err = io::Error::new(io::ErrorKind::ConnectionReset, "connection reset by peer");
    let error = error!(
        source: io_err,
        "grp::net::tcp",
        "TCP connection lost unexpectedly",
        "Connection reset while awaiting packet payload",
        "Peer closed connection during TLS handshake",
        "Retried 3 times before giving up",
    );

    assert_eq!(error.etype(), "grp::net::tcp");
    assert_eq!(error.message(), "TCP connection lost unexpectedly");
    assert_eq!(
        error.details(),
        vec![
            "Connection reset while awaiting packet payload",
            "Peer closed connection during TLS handshake",
            "Retried 3 times before giving up",
        ]
    );
    assert!(error.notes().is_empty());

    let src = std::error::Error::source(&error);
    assert!(src.is_some());
    assert_eq!(src.unwrap().to_string(), "connection reset by peer");
}

#[test]
fn test_macro_error_with_source_and_notes_comma_arrow() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let error = error!(
        source: io_err,
        "grp::net::bind",
        "Could not bind listener to address",
        "Port 80 is privileged and requires elevated permissions",
        =>
        "Try running the application with sudo or root privileges",
        "Try using an unprivileged port like 8080 or 3000 instead",
    );

    assert_eq!(error.etype(), "grp::net::bind");
    assert_eq!(error.message(), "Could not bind listener to address");
    assert_eq!(
        error.details(),
        vec!["Port 80 is privileged and requires elevated permissions"]
    );
    assert_eq!(
        error.notes(),
        vec![
            "Try running the application with sudo or root privileges",
            "Try using an unprivileged port like 8080 or 3000 instead",
        ]
    );

    let src = std::error::Error::source(&error);
    assert!(src.is_some());
    assert_eq!(src.unwrap().to_string(), "access denied");
}

#[test]
fn test_macro_error_with_source_and_notes_without_comma_arrow() {
    let io_err = io::Error::new(io::ErrorKind::TimedOut, "connection timed out");
    let error = error!(
        source: io_err,
        "grp::db::pool",
        "Database connection pool exhausted",
        "Timed out waiting 3000ms for an available connection from pool 'main'"
        =>
        "Try increasing the 'max_connections' limit in database.toml",
        "Ensure active transactions are released after query execution",
    );

    assert_eq!(error.etype(), "grp::db::pool");
    assert_eq!(error.message(), "Database connection pool exhausted");
    assert_eq!(
        error.details(),
        vec!["Timed out waiting 3000ms for an available connection from pool 'main'"]
    );
    assert_eq!(
        error.notes(),
        vec![
            "Try increasing the 'max_connections' limit in database.toml",
            "Ensure active transactions are released after query execution",
        ]
    );

    let src = std::error::Error::source(&error);
    assert!(src.is_some());
    assert_eq!(src.unwrap().to_string(), "connection timed out");
}

#[test]
fn test_macro_error_with_source_details_and_notes() {
    let io_err = io::Error::new(io::ErrorKind::AddrInUse, "address already in use");
    let error = error!(
        source: io_err,
        "net::http::bind",
        "Failed to bind socket",
        "Port 80 requires privileged access",
        "Interface 0.0.0.0 is already bound by another process"
        =>
        "Run with sudo or change port to 8080",
        "Check if another instance is already running with 'lsof -i :80'",
    );

    assert_eq!(error.etype(), "net::http::bind");
    assert_eq!(error.message(), "Failed to bind socket");
    assert_eq!(
        error.details(),
        vec![
            "Port 80 requires privileged access",
            "Interface 0.0.0.0 is already bound by another process",
        ]
    );
    assert_eq!(
        error.notes(),
        vec![
            "Run with sudo or change port to 8080",
            "Check if another instance is already running with 'lsof -i :80'",
        ]
    );

    let src = std::error::Error::source(&error);
    assert!(src.is_some());
    let src = src.unwrap();
    assert_eq!(src.to_string(), "address already in use");
    assert!(src.downcast_ref::<io::Error>().is_some());
}
