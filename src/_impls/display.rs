use std::fmt::Display;

use crate::Error;
use crate::Report;


impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() { 
            for s in self.render_plain() {
                write!(f, "{}", s)?
            };
        }
        else { write!(f, "{}", self.message())?; }
        
        Ok(())
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}", self.0)
    }
}
