use std::fmt::Debug;

use crate::Report;

impl Debug for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.0.render_std_err_auto() {
            write!(f, "{s}")?
        }
        
        Ok(())
    }
}
