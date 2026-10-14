use std::fmt::Display;

/// Provides visual styling markers for tips, commands, and notes.
///
/// Implemented for any type implementing [`Display`]. Calling these methods prefixes the formatted
/// text with internal tags (`aaa:`, `aab:`, `aac:`) that the renderer transforms into styled
/// bullet points, checkmarks, or terminal commands.
///
/// # Examples
///
/// ```rust
/// use mynox::Formater;
///
/// let cmd = "cargo build --release".as_command();
/// let tip = "Ensure all dependencies are satisfied".as_tip();
/// ```
pub trait Formater {
    /// Formats the item as an actionable CLI command (rendered with a bullet `•` and highlighted).
    fn as_command(self) -> String;

    /// Formats the item as a primary tip (rendered with a checkmark `✓`).
    fn as_tip(self) -> String;

    /// Formats the item as a continuation line of a previous tip.
    fn as_tip_cotinuation(self) -> String;
}

pub(crate) trait Printer {
    fn as_ref<'a>(&'a self) -> &'a String;
    fn as_command(&self) -> String;
    fn as_tip(&self) -> String;
    fn as_tip_cotinuation(&self) -> String;
}

/// Allows printing formatted diagnostic lines directly to stdout or stderr.
///
/// This trait is implemented for iterators over [`String`], such as those returned
/// by [`Error::render_plain`](crate::Error::render_plain) or
/// [`Error::render_color`](crate::Error::render_color).
///
/// # Examples
///
/// ```rust
/// use mynox::{error, Show};
///
/// let err = error!("cli::arg", "Missing argument", "Argument '--config' is required");
/// // err.render_plain().println();
/// ```
pub trait Show {
    /// Prints all items to standard output without a trailing newline.
    fn print(&mut self);

    /// Prints all items to standard output followed by a newline.
    fn println(&mut self) {
        self.print();
        print!("\n");
    }

    /// Prints all items to standard error without a trailing newline.
    fn eprint(&mut self);

    /// Prints all items to standard error followed by a newline.
    fn eprintln(&mut self) {
        self.print();
        eprint!("\n");
    }
}

/// Extension trait for [`Result`] to convert errors into structured [`Error`](crate::Error) instances.
///
/// Allows ergonomic propagation with the `?` operator while attaching category codes (`etype`),
/// human-friendly messages, and diagnostic details.
///
/// # Examples
///
/// ```rust
/// use mynox::Context;
/// use std::fs::File;
///
/// fn open_config() -> Result<File, mynox::Error> {
///     File::open("/nonexistent/file.toml")
///         .context("io::fs::read", "Failed to open application configuration")
/// }
///
/// assert!(open_config().is_err());
/// ```
pub trait Context<T, E> {
    /// Fallback context for errors that implement neither [`std::error::Error`] nor [`Display`].
    ///
    /// Accepts a custom `message` and `detail`. The original error is discarded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::Context;
    ///
    /// let res: Result<(), ()> = Err(());
    /// let err = res.context_raw("custom::failure", "Raw failure occurred", "No additional error payload").unwrap_err();
    /// assert_eq!(err.etype(), "custom::failure");
    /// ```
    fn context_raw<M, D>(self, etype: &'static str, message: M, detail: D) -> Result<T, crate::Error>
    where
        M: Into<String>,
        D: Into<String>;

    /// Context for errors that implement [`Display`] but not [`std::error::Error`].
    ///
    /// Uses `E::to_string()` as the error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::Context;
    ///
    /// let res: Result<(), &str> = Err("syntax parse error");
    /// let err = res.context_display("parser::syntax", "Failed at column 12").unwrap_err();
    /// assert_eq!(err.message(), "syntax parse error");
    /// ```
    fn context_display<D>(self, etype: &'static str, detail: D) -> Result<T, crate::Error>
    where
        E: Display,
        D: Into<String>;

    /// Full context for standard errors implementing [`std::error::Error`].
    ///
    /// Preserves the original error as the causal source (accessible via [`std::error::Error::source`]),
    /// while using its `Display` representation as the error `message`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::Context;
    /// use std::io;
    ///
    /// let io_err: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::NotFound, "file missing"));
    /// let err = io_err.context("fs::read", "Could not locate file").unwrap_err();
    /// assert_eq!(err.etype(), "fs::read");
    /// assert!(std::error::Error::source(&err).is_some());
    /// ```
    fn context<D>(self, etype: &'static str, detail: D) -> Result<T, crate::Error>
    where
        E: std::error::Error + Send + Sync + 'static,
        D: Into<String>;
}

/// Extension trait for [`Result<T, Error>`](Result) to progressively attach details and notes.
///
/// Useful for enriching errors along the call stack with contextual details or suggestions.
///
/// # Examples
///
/// ```rust
/// use mynox::{error, Extend};
///
/// let res: Result<(), mynox::Error> = Err(error!("db::query", "Query failed", "Syntax error near 'FROM'"));
/// let res = res
///     .detail("Attempted query: SELECT * FROM ;")
///     .note("Verify SQL syntax conforms to PostgreSQL dialect");
///
/// let err = res.unwrap_err();
/// assert_eq!(err.details().len(), 2);
/// assert_eq!(err.notes().len(), 1);
/// ```
pub trait Extend<T> {
    /// Appends a diagnostic detail to an existing error in `Result<T, Error>`.
    fn detail<S: Into<String>>(self, detail: S) -> Result<T, crate::Error>;

    /// Appends multiple diagnostic details to an existing error.
    fn details<B>(self, details: B) -> Result<T, crate::Error>
    where
        B: IntoIterator,
        B::Item: Into<String>,
    ;

    /// Lazily computes and appends multiple diagnostic details using a closure.
    fn with_details<F, B>(self, f: F) -> Result<T, crate::Error>
    where
        F: FnOnce() -> B,
        B: IntoIterator,
        B::Item: Into<String>,
    ;

    /// Appends an actionable note/suggestion to an existing error.
    fn note<S: Into<String>>(self, note: S) -> Result<T, crate::Error>;

    /// Appends multiple actionable notes/suggestions to an existing error.
    fn notes<B>(self, notes: B) -> Result<T, crate::Error>
    where
        B: IntoIterator,
        B::Item: Into<String>,
    ;

    /// Lazily computes and appends multiple actionable notes/suggestions using a closure.`
    fn with_notes<F, B>(self, f: F) -> Result<T, crate::Error>
    where
        F: FnOnce() -> B,
        B: IntoIterator,
        B::Item: Into<String>,
    ;
}
