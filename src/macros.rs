/// Helper macro that generates an empty array of `&'static str` (`[""; 0]`).
///
/// Useful when constructing an [`Error`](crate::Error) directly with [`Error::new`](crate::Error::new)
/// or [`Error::from`](crate::Error::from) when no additional details or notes are needed.
///
/// # Examples
///
/// ```rust
/// use mynox::{Error, empty_iter};
///
/// let err = Error::new("app::start", "Start failed", "Missing argument", empty_iter!(), empty_iter!());
/// assert!(err.notes().is_empty());
/// ```
#[macro_export]
macro_rules! empty_iter { () => {{ [""; 0] }}; }

/// Constructs a structured [`Error`](crate::Error) with concise syntax.
///
/// The `error!` macro is the primary and most expressive way to instantiate errors in `mynox`.
/// It supports defining the error type identifier (`etype`), a high-level summary `message`,
/// one or multiple `details`, optional actionable `notes` (delimited by `=>`), and an optional
/// underlying causal `source: <expr>`.
///
/// # Syntax Variations
///
/// 1. **Basic 3-argument syntax:**
///    ```rust
///    use mynox::error;
///    let err = error!("io::fs::not_found", "File not found", "Could not locate 'config.toml'");
///    assert_eq!(err.etype(), "io::fs::not_found");
///    ```
///
/// 2. **Multiple details:**
///    ```rust
///    use mynox::error;
///    let err = error!(
///        "syntax::json",
///        "Failed to parse JSON",
///        "Unexpected comma at line 4, column 18",
///        "Expected string key or closing curly brace"
///    );
///    assert_eq!(err.details().len(), 2);
///    ```
///
/// 3. **Actionable notes separated by `=>`:**
///    ```rust
///    use mynox::error;
///    let err = error!(
///        "net::http::bind",
///        "Failed to bind socket",
///        "Port 80 requires privileged access"
///        =>
///        "Run with elevated privileges (sudo)",
///        "Or bind to an unprivileged port like 8080"
///    );
///    assert_eq!(err.notes().len(), 2);
///    ```
///
/// 4. **Preserving an underlying causal error (`source:`):**
///    ```rust
///    use std::io;
///    use mynox::error;
///
///    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
///    let err = error!(
///        source: io_err,
///        "app::fs::permission",
///        "Cannot access configuration file",
///        "Permission denied for '/etc/secret.key'"
///        =>
///        "Adjust file permissions with 'chmod 600 /etc/secret.key'"
///    );
///
///    assert!(std::error::Error::source(&err).is_some());
///    ```
#[macro_export]
macro_rules! error {
    ($etype:expr, $message:expr, $detail:expr) => {{
        $crate::Error::new($etype, $message, $detail, [""; 0], [""; 0])
    }};
    (
        $etype:expr, 
        $message:expr, 
        $detail:expr, 
            $( $content:expr ),* $(,)?
    ) => {{
        $crate::Error::new($etype, $message, $detail, [
            $( $content, )*
        ], [""; 0])
    }};
    (
        $etype:expr, 
        $message:expr, 
        $detail:expr,  =>
            $( $notes:expr ),* $(,)?
    ) => {{
        $crate::Error::new($etype, $message, $detail, [""; 0], [
            $( $notes, )*
        ])
    }};
    (
        $etype:expr, 
        $message:expr, 
        $detail:expr =>
            $( $notes:expr ),* $(,)?
    ) => {{
        $crate::Error::new($etype, $message, $detail, [""; 0], [
            $( $notes, )*
        ])
    }};
    (
        $etype:expr, 
        $message:expr, 
        $detail:expr, 
            $( $content:expr ),* $(,)? =>
            $( $notes:expr ),* $(,)?
    ) => {{
        $crate::Error::new($etype, $message, $detail, [
            $( $content, )*
        ], [
            $( $notes, )*
        ])
    }};
    (source: $source:expr, $etype:expr, $message:expr, $detail:expr) => {{
        $crate::Error::from($source, $etype, $message, $detail, [""; 0], [""; 0])
    }};
    (
        source: $source:expr, 
        $etype:expr, 
        $message:expr, 
        $detail:expr, 
            $( $content:expr ),* $(,)?
    ) => {{
        $crate::Error::from($source, $etype, $message, $detail, [
            $( $content, )*
        ], [""; 0])
    }};
    (
        source: $source:expr, 
        $etype:expr, 
        $message:expr, 
        $detail:expr,  =>
            $( $notes:expr ),* $(,)?
    ) => {{
        $crate::Error::from($source, $etype, $message, $detail, [""; 0], [
            $( $notes, )*
        ])
    }};
    (
        source: $source:expr, 
        $etype:expr, 
        $message:expr, 
        $detail:expr =>
            $( $notes:expr ),* $(,)?
    ) => {{
        $crate::Error::from($source, $etype, $message, $detail, [""; 0], [
            $( $notes, )*
        ])
    }};
    (
        source: $source:expr, 
        $etype:expr, 
        $message:expr, 
        $detail:expr, 
            $( $content:expr ),* $(,)? =>
            $( $notes:expr ),* $(,)?
    ) => {{
        $crate::Error::from($source, $etype, $message, $detail, [
            $( $content, )*
        ], [
            $( $notes, )*
        ])
    }};
}
