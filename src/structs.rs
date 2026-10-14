use std::iter::once;
use std::sync::Arc;
use std::fmt::Display;


pub struct Report(pub(crate) Error);

impl Report {
    pub fn from(error: Error) -> Self { Self(error) }
}

impl From<Error> for Report {
    fn from(err: Error) -> Self {
        Report(err)
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Error {
    /// Where the error came from
    pub(crate) source: Option<Arc<dyn std::error::Error + Send + Sync + 'static>>,
    /// Error `id` or `error_type` same thing.
    etype: &'static str,
    /// Principal error message.
    pub(crate) message: String,
    /// **Details** of what whent wrong.
    pub(crate) details: Vec<String>,
    /// **Notes** of what to do to solve it.
    pub(crate) notes: Vec<String>,
}

impl Error {
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
        }
    }
    
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
        }
    }
    
    pub fn etype(&self) -> &'static str { self.etype }
    pub fn message(&self) -> String { self.message.to_owned() }
    pub fn details(&self) -> Vec<String> { self.details.to_owned() }
    pub fn notes(&self) -> Vec<String> { self.notes.to_owned() }
    
    pub fn is(&self, other: &str) -> bool { self.etype() == other }
}
