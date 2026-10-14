use crate::Error;

impl Eq for Error { }
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.etype() == other.etype()
    }
}
