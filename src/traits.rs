use std::fmt::Display;


pub trait Formater {
    fn as_command(self) -> String;
    fn as_tip(self) -> String;
    fn as_tip_cotinuation(self) -> String;
}

pub(crate) trait Printer {
    fn as_ref<'a>(&'a self) -> &'a String;
    fn as_command(&self) -> String;
    fn as_tip(&self) -> String;
    fn as_tip_cotinuation(&self) -> String;
}

pub trait Show {
    fn print(&mut self);
    fn println(&mut self) {
        self.print();
        print!("\n");
    }
    fn eprint(&mut self);
    fn eprintln(&mut self) {
        self.print();
        eprint!("\n");
    }
}

pub trait Context<T, E> {
    /// Fallback for errors that implement neither Error nor Display
    fn context_raw<M, D>(self, etype: &'static str, message: M, detail: D) -> Result<T, crate::Error>
    where
        M: Into<String>,
        D: Into<String>;

    /// For errors that only implement Display (no source error stored)
    fn context_display<D>(self, etype: &'static str, detail: D) -> Result<T, crate::Error>
    where
        E: Display,
        D: Into<String>;

    /// Full context for standard errors, preserving the source error
    fn context<D>(self, etype: &'static str, detail: D) -> Result<T, crate::Error>
    where
        E: std::error::Error + Send + Sync + 'static,
        D: Into<String>;
}

pub trait Extend<T> {
    fn deatil<S: Into<String>>(self, detail: S) -> Result<T, crate::Error>;
    fn details<B>(self, details: B) -> Result<T, crate::Error>
    where
        B: IntoIterator,
        B::Item: Into<String>,
    ;
    fn with_details<F, B>(self, f: F) -> Result<T, crate::Error>
    where
        F: FnOnce() -> B,
        B: IntoIterator,
        B::Item: Into<String>,
    ;
    fn note<S: Into<String>>(self, note: S) -> Result<T, crate::Error>;
    fn notes<B>(self, notes: B) -> Result<T, crate::Error>
    where
        B: IntoIterator,
        B::Item: Into<String>,
    ;
    fn with_notes<F, B>(self, f: F) -> Result<T, crate::Error>
    where
        F: FnOnce() -> B,
        B: IntoIterator,
        B::Item: Into<String>,
    ;
}
