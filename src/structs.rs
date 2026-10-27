use std::collections::HashSet;
use std::iter::once;
use std::sync::Arc;
use std::fmt::Display;

/// A diagnostic reporting wrapper around [`Error`].
///
/// `Report` formats the underlying error using [`Error::render_std_err_auto`] when printed
/// via `{:?}` ([`std::fmt::Debug`]), or plain text when printed via `{}` ([`std::fmt::Display`]).
///
/// This makes `Report` ideal as the return error type for `fn main() -> Result<(), mynox::Report>`,
/// producing beautifully styled terminal diagnostics automatically when the application exits with an error.
///
/// # Examples
///
/// ```rust
/// use mynox::{error, Report};
///
/// fn run() -> Result<(), Report> {
///     let err = error!(
///         "app::init",
///         "Failed to initialize application",
///         "Configuration file is missing"
///         =>
///         "Create a default 'config.toml' in the current directory"
///     );
///     Err(Report::from(err))
/// }
///
/// let res = run();
/// assert!(res.is_err());
/// ```
pub struct Report(pub(crate) Error);

impl Report {
    /// Creates a new `Report` wrapping the given [`Error`].
    pub fn from(error: Error) -> Self { Self(error) }
}

impl From<Error> for Report {
    fn from(err: Error) -> Self {
        Report(err)
    }
}

/// The core error type representing a structured diagnostic error.
///
/// An `Error` consists of:
/// - An **`etype`**: a static string identifier (e.g. `"net::http::bind"`) used for matching and categorization.
/// - A **`message`**: the high-level, human-readable summary of what went wrong.
/// - One or more **`details`**: specific diagnostic facts explaining what happened.
/// - Zero or more **`notes`**: actionable recommendations, tips, or hints on how to resolve the error.
/// - An optional **`source`**: an underlying causal error implementing [`std::error::Error`].
///
/// # Creation
///
/// Errors are typically constructed using the [`error!`](crate::error) macro, but can also be
/// created directly with [`Error::new`] or [`Error::from`].
///
/// # Examples
///
/// ```rust
/// use mynox::Error;
///
/// let err = Error::new(
///     "io::fs::not_found",
///     "Configuration file missing",
///     "File '/etc/myapp/config.toml' not found",
///     ["Permission was granted, but inode does not exist"],
///     ["Check the file path or run 'myapp init' to generate it"],
/// );
///
/// assert_eq!(err.etype(), "io::fs::not_found");
/// assert_eq!(err.message(), "Configuration file missing");
/// assert!(err.is("io::fs::not_found"));
/// ```
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Error {
    /// Where the error came from
    pub(crate) source: Option<Arc<dyn std::error::Error + Send + Sync + 'static>>,
    /// Error `id` or `error_type` same thing.
    etype: &'static str,
    /// Principal error message.
    pub(crate) message: String,
    /// **Details** of what went wrong.
    pub(crate) details: Vec<String>,
    /// **Notes** of what to do to solve it.
    pub(crate) notes: Vec<String>,
    /// **Errors** a list of errors of a collection, allways empty unless his etype is `mynox::collection`
    pub(crate) errors: HashSet<Error>,
}

impl Error {
    /// Constructs a new `Error` without an underlying causal source.
    ///
    /// # Arguments
    ///
    /// - `etype`: Static string categorizing the error (e.g. `"io::fs::not_found"`).
    /// - `message`: High-level summary of the error.
    /// - `details`: Primary diagnostic detail.
    /// - `explanation`: Additional details iterator.
    /// - `notes`: Actionable notes/tips iterator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::{Error, empty_iter};
    ///
    /// let err = Error::new(
    ///     "auth::token::invalid",
    ///     "Invalid credentials",
    ///     "Signature verification failed",
    ///     empty_iter!(),
    ///     ["Ensure your JWT secret is up to date"],
    /// );
    ///
    /// assert_eq!(err.etype(), "auth::token::invalid");
    /// ```
    pub fn new<M, P, V, B>(
        etype: &'static str,
        message: M,
        details: P,
        explanation: V,
        notes: B,
    ) -> Self
    where
        M: Display,
        P: Display,
        V: IntoIterator,
        V::Item: Into<String>,
        B: IntoIterator,
        B::Item: Into<String>,
    {
        Self {
            source: None,
            etype,
            message: message.to_string(),
            details: once(details.to_string()).chain(explanation.into_iter().map(Into::into)).collect(),
            notes: notes.into_iter().map(Into::into).collect(),
            errors: HashSet::new(),
        }
    }
    
    /// Constructs a new `Error` wrapping an underlying causal `source`.
    ///
    /// The causal error will be accessible via [`std::error::Error::source`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::io;
    /// use mynox::{Error, empty_iter};
    ///
    /// let io_err = io::Error::new(io::ErrorKind::NotFound, "entity not found");
    /// let err = Error::from(
    ///     io_err,
    ///     "io::fs::read",
    ///     "Failed to load file",
    ///     "Cannot access path",
    ///     empty_iter!(),
    ///     ["Ensure file exists"],
    /// );
    ///
    /// assert!(std::error::Error::source(&err).is_some());
    /// ```
    pub fn from<E, M, P, V, B>(
        source: E,
        etype: &'static str,
        message: M,
        details: P,
        explanation: V,
        notes: B,
    ) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
        M: Display,
        P: Display,
        V: IntoIterator,
        V::Item: Into<String>,
        B: IntoIterator,
        B::Item: Into<String>,
    {
        Self {
            source: Some(Arc::new(source)),
            etype,
            message: message.to_string(),
            details: once(details.to_string()).chain(explanation.into_iter().map(Into::into)).collect(),
            notes: notes.into_iter().map(Into::into).collect(),
            errors: HashSet::new(),
        }
    }
    
    /// Returns the static error type/identifier.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::error;
    ///
    /// let err = error!("net::timeout", "Request timed out", "Server took longer than 5s");
    /// assert_eq!(err.etype(), "net::timeout");
    /// ```
    pub fn etype(&self) -> &'static str { self.etype }

    /// Returns a copy of the high-level error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::error;
    ///
    /// let err = error!("net::timeout", "Request timed out", "Server took longer than 5s");
    /// assert_eq!(err.message(), "Request timed out");
    /// ```
    pub fn message(&self) -> String { self.message.to_owned() }

    /// Returns a copy of the diagnostic details list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::error;
    ///
    /// let err = error!("db::query", "Query failed", "Syntax error at line 1", "Table does not exist");
    /// assert_eq!(err.details().len(), 2);
    /// ```
    pub fn details(&self) -> Vec<String> { self.details.to_owned() }

    /// Returns a copy of the actionable notes list.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::error;
    ///
    /// let err = error!(
    ///     "db::query",
    ///     "Query failed",
    ///     "Table 'users' does not exist"
    ///     =>
    ///     "Run migrations using 'cargo run -- migrate'"
    /// );
    /// assert_eq!(err.notes().len(), 1);
    /// ```
    pub fn notes(&self) -> Vec<String> { self.notes.to_owned() }
    
    /// Checks if this error's type exactly matches `other`.
    ///
    /// For hierarchical matching (e.g. matching `"io::fs"` against `"io::fs::not_found"`),
    /// see [`Error::is_category`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::error;
    ///
    /// let err = error!("auth::expired", "Session expired", "Token is no longer valid");
    /// assert!(err.is("auth::expired"));
    /// assert!(!err.is("auth::forbidden"));
    /// ```
    pub fn is(&self, other: &str) -> bool { self.etype() == other }
}
