use std::hash::{Hash, Hasher};

use crate::Error;

impl Hash for Error {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self.etype()).hash(state);
    }
}
