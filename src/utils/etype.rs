impl crate::Error {
    /// Checks whether this error belongs to a hierarchical category prefix.
    ///
    /// Hierarchies use `::` as separators (e.g. `"io::fs::not_found"`).
    /// Both `"io"` and `"io::fs"` will match `"io::fs::not_found"`.
    /// Trailing `::` in `prefix` are ignored.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::error;
    ///
    /// let err = error!("io::fs::not_found", "File not found", "Could not open /tmp/test");
    ///
    /// assert!(err.is_category("io"));
    /// assert!(err.is_category("io::fs"));
    /// assert!(err.is_category("io::fs::not_found"));
    /// assert!(!err.is_category("net"));
    /// assert!(!err.is_category("io::net"));
    /// ```
    pub fn is_category(&self, prefix: &str) -> bool {
        let prefix = prefix.trim_end_matches(':');
        self.etype() == prefix || self.etype().starts_with(&format!("{prefix}::"))
    }
}
