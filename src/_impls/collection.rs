use crate::Error;
use crate::error;

impl Error {
    /// Returns `true` if this error represents a collection of multiple errors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::{error, OptionError};
    ///
    /// let err1 = error!("io::not_found", "File not found", "Path does not exist");
    /// let err2 = error!("net::timeout", "Network timeout", "Host unreachable");
    ///
    /// let collection = [err1, err2].into_iter().collect::<OptionError>().into_option().unwrap();
    /// assert!(collection.is_collection());
    /// ```
    pub fn is_collection(&self) -> bool { self.is(crate::collection) }
    
    /// Checks if this error or any sub-error in a collection matches the given category prefix.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::{error, OptionError};
    ///
    /// let err1 = error!("io::fs::read", "Read error", "Access denied");
    /// let err2 = error!("net::timeout", "Network timeout", "Host unreachable");
    ///
    /// let collection = [err1, err2].into_iter().collect::<OptionError>().into_option().unwrap();
    /// assert!(collection.contains_category("io"));
    /// assert!(collection.contains_category("net"));
    /// assert!(!collection.contains_category("db"));
    /// ```
    pub fn contains_category(&self, prefix: &str) -> bool {
        match self.is_collection() {
            true => self.errors.iter().any(|e| e.is_category(prefix)),
            false => self.is_category(prefix),
        }
    }
    
    /// Checks if this error or any sub-error in a collection exactly matches the given `etype`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::{error, OptionError};
    ///
    /// let err1 = error!("io::fs::read", "Read error", "Access denied");
    /// let err2 = error!("net::timeout", "Network timeout", "Host unreachable");
    ///
    /// let collection = [err1, err2].into_iter().collect::<OptionError>().into_option().unwrap();
    /// assert!(collection.contains_exact("io::fs::read"));
    /// assert!(!collection.contains_exact("io::fs"));
    /// ```
    pub fn contains_exact(&self, etype: &str) -> bool {
        match self.is_collection() {
            true => self.errors.iter().any(|e| e.is(etype)),
            false => self.is(etype),
        }
    }
    
    /// Retrieves a reference to a sub-error by its exact `etype`.
    ///
    /// If `self` is a single error, it returns `Some(self)` if the `etype` matches.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::{error, OptionError};
    ///
    /// let err1 = error!("io::fs::read", "Read error", "Access denied");
    /// let err2 = error!("net::timeout", "Network timeout", "Host unreachable");
    ///
    /// let collection = [err1, err2].into_iter().collect::<OptionError>().into_option().unwrap();
    /// let found = collection.get_error("io::fs::read");
    /// assert!(found.is_some());
    /// assert_eq!(found.unwrap().message(), "Read error");
    /// ```
    pub fn get_error(&self, etype: &str) -> Option<&Error> {
        match self.is_collection() {
            true => self.errors.get(etype),
            false => self.is(etype).then_some(self),
        }
    }
    
    /// Returns an iterator over the static `etype` identifiers contained in this error or collection.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::{error, OptionError};
    ///
    /// let err1 = error!("io::fs::read", "Read error", "Access denied");
    /// let err2 = error!("net::timeout", "Network timeout", "Host unreachable");
    ///
    /// let collection = [err1, err2].into_iter().collect::<OptionError>().into_option().unwrap();
    /// let types: Vec<_> = collection.get_etypes().collect();
    /// assert_eq!(types.len(), 2);
    /// assert!(types.contains(&"io::fs::read"));
    /// assert!(types.contains(&"net::timeout"));
    /// ```
    pub fn get_etypes(&self) -> Box<dyn Iterator<Item = &'static str> + '_> {
        match self.is_collection() {
            true => Box::new(self.errors.iter().map(|e| e.etype())),
            false => Box::new(std::iter::once(self.etype())),
        }
    }
}

/// An optional error collection accumulator for use with [`FromIterator`].
///
/// Collecting an iterator of [`Error`] into `OptionError` produces:
/// - [`OptionError::None`] if the iterator is empty.
/// - [`OptionError::Some`] containing the single error directly if the iterator had exactly 1 error (or 1 unique `etype`).
/// - [`OptionError::Some`] containing a collection error if the iterator had multiple unique errors.
///
/// # Examples
///
/// ```rust
/// use mynox::{error, OptionError, Error};
///
/// // Empty iterator yields None
/// let empty: Vec<Error> = vec![];
/// assert!(empty.into_iter().collect::<OptionError>().into_option().is_none());
///
/// // Single error yields that exact error without wrapping in a collection
/// let single = vec![error!("auth::failed", "Auth error", "Invalid token")];
/// let res = single.into_iter().collect::<OptionError>().into_option().unwrap();
/// assert!(!res.is_collection());
/// assert_eq!(res.etype(), "auth::failed");
/// ```
#[derive(Debug, Clone)]
pub enum OptionError {
    Some(Error),
    None,
} 

impl OptionError {
    /// Converts this `OptionError` into a standard [`Option<Error>`].
    pub fn into_option(self) -> Option<Error> { self.into() }
}

impl From<OptionError> for Option<Error> {
    fn from(value: OptionError) -> Self {
        match value {
            OptionError::Some(error) => Some(error),
            OptionError::None => None,
        }
    }
}

impl FromIterator<Error> for OptionError {
    fn from_iter<T: IntoIterator<Item = Error>>(iter: T) -> Self {
        let mut errors = iter.into_iter().peekable();
        
        let first = match errors.next() {
            Some(e) => e,
            None => return OptionError::None,
        };
        
        let peek = errors.peek();
        if peek.is_none() { return OptionError::Some(first); };
        
        let mut error = error!(
            crate::collection,
            "Multiple errors happend",
            "Here is a resume of them",
        );
        
        error = concat(error, first);
        
        while let Some(other) = errors.next() {
            error = concat(error, other)
        }
        
        if error.errors.len() == 1 {
            OptionError::Some(error.errors.into_iter().take(1).next().unwrap())
        } else {
            OptionError::Some(error)
        }
    }
}

fn concat(mut error: Error, other: Error) -> Error {
    match other.is_collection() {
        true => {
            error.errors.extend(other.errors);
            error
        },
        false => {
            error.errors.insert(other);
            error
        },
    }
}
