use crate::Extend;
use crate::Error;


impl<T> Extend<T> for Result<T, Error> {
    fn deatil<S: Into<String>>(self, detail: S) -> Result<T, crate::Error> {
        self.map_err(|mut e| {
            e.details.push(detail.into());
            e
        })
    }

    fn details<B>(self, details: B) -> Result<T, crate::Error>
    where
        B: IntoIterator,
        B::Item: Into<String>,
     {
        self.map_err(|mut e| {
            e.details.extend(details.into_iter().map(|s| s.into()));
            e
        })
    }

    fn with_details<F, B>(self, f: F) -> Result<T, crate::Error>
    where
        F: FnOnce() -> B,
        B: IntoIterator,
        B::Item: Into<String>,
     {
        self.map_err(|mut e| {
            let details = f();
            e.details.extend(details.into_iter().map(|s| s.into()));
            e
        })
    }

    fn note<S: Into<String>>(self, note: S) -> Result<T, crate::Error> {
        self.map_err(|mut e| {
            e.notes.push(note.into());
            e
        })
    }

    fn notes<B>(self, notes: B) -> Result<T, crate::Error>
    where
        B: IntoIterator,
        B::Item: Into<String>,
     {
        self.map_err(|mut e| {
            e.notes.extend(notes.into_iter().map(|s| s.into()));
            e
        })
    }

    fn with_notes<F, B>(self, f: F) -> Result<T, crate::Error>
    where
        F: FnOnce() -> B,
        B: IntoIterator,
        B::Item: Into<String>,
     {
        self.map_err(|mut e| {
            let notes = f();
            e.notes.extend(notes.into_iter().map(|s| s.into()));
            e
        })
    }
}
