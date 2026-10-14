use std::fmt::Display;

use crate::{Context, Error, empty_iter};


impl<T, E> Context<T, E> for Result<T, E> {
    fn context_raw<M, D>(self, etype: &'static str, message: M, detail: D) -> Result<T, Error>
    where
        M: Into<String>,
        D: Into<String>,
    {
        self.map_err(|_| {
            Error::new(etype, message.into(), detail.into(), empty_iter!(), empty_iter!())
        })
    }

    fn context_display<D>(self, etype: &'static str, detail: D) -> Result<T, Error>
    where
        E: Display,
        D: Into<String>,
    {
        self.map_err(|e| {
            Error::new(etype, e.to_string(), detail.into(), empty_iter!(), empty_iter!())
        })
    }

    fn context<D>(self, etype: &'static str, detail: D) -> Result<T, Error>
    where
        E: std::error::Error + Send + Sync + 'static,
        D: Into<String>,
    {
        self.map_err(|e| {
            let message = e.to_string();
            Error::from(e, etype, message, detail.into(), empty_iter!(), empty_iter!())
        })
    }
}
