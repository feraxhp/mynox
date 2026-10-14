use std::borrow::Borrow;

use crate::Error;


impl Borrow<str> for Error {
    fn borrow(&self) -> &str {
        self.etype()
    }
}
