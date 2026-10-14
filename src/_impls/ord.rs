use std::cmp::Ordering;

use crate::Error;

impl Ord for Error {
    fn cmp(&self, other: &Self) -> Ordering {
        self.etype().cmp(other.etype())
    }
}

impl PartialOrd for Error {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
