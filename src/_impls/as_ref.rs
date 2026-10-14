use crate::Error;

impl AsRef<str> for Error {
    fn as_ref(&self) -> &str {
        &self.message
    }
}
