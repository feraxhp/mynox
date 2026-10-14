use crate::Extend;
use crate::Error;


impl Error {
    pub fn extend_detail<S: Into<String>>(mut self, detail: S) -> Error {
        self.details.push(detail.into());
        self
    }

    pub fn extend_details<B>(mut self, details: B) -> Error
    where
        B: IntoIterator,
        B::Item: Into<String>,
     {
        self.details.extend(details.into_iter().map(|s| s.into()));
        self
    }

    pub fn extend_details_with<F, B>(mut self, f: F) -> Error
    where
        F: FnOnce() -> B,
        B: IntoIterator,
        B::Item: Into<String>,
     {
        let details = f();
        self.details.extend(details.into_iter().map(|s| s.into()));
        self
    }

    pub fn extend_note<S: Into<String>>(mut self, note: S) -> Error {
        self.notes.push(note.into());
        self
    }

    pub fn extend_notes<B>(mut self, notes: B) -> Error
    where
        B: IntoIterator,
        B::Item: Into<String>,
     {
        self.notes.extend(notes.into_iter().map(|s| s.into()));
        self
    }

    pub fn extend_notes_with<F, B>(mut self, f: F) -> Error
    where
        F: FnOnce() -> B,
        B: IntoIterator,
        B::Item: Into<String>,
     {
        let notes = f();
        self.notes.extend(notes.into_iter().map(|s| s.into()));
        self
    }
}


impl<T> Extend<T> for Result<T, Error> {
    fn detail<S: Into<String>>(self, detail: S) -> Result<T, crate::Error> {
        self.map_err(|e| { e.extend_detail(detail) })
    }

    fn details<B>(self, details: B) -> Result<T, crate::Error>
    where
        B: IntoIterator,
        B::Item: Into<String>,
     {
        self.map_err(|e| { e.extend_details(details) })
    }

    fn with_details<F, B>(self, f: F) -> Result<T, crate::Error>
    where
        F: FnOnce() -> B,
        B: IntoIterator,
        B::Item: Into<String>,
     {
        self.map_err(|e| { e.extend_details_with(f) })
    }

    fn note<S: Into<String>>(self, note: S) -> Result<T, crate::Error> {
        self.map_err(|e| { e.extend_note(note) })
    }

    fn notes<B>(self, notes: B) -> Result<T, crate::Error>
    where
        B: IntoIterator,
        B::Item: Into<String>,
     {
        self.map_err(|e| { e.extend_notes(notes) })
    }

    fn with_notes<F, B>(self, f: F) -> Result<T, crate::Error>
    where
        F: FnOnce() -> B,
        B: IntoIterator,
        B::Item: Into<String>,
     {
        self.map_err(|e| { e.extend_notes_with(f) })
    }
}
