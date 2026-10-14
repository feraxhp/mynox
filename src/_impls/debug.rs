use std::fmt::Debug;

use crate::{Report, formater::markdown::Colors};

impl Debug for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}{}{}", Colors::Red, self.0.message(), Colors::Reset)?;
        
        for s in self.0.render_std_err_auto() {
            writeln!(f, "{s}")?
        }
        
        Ok(())
    }
}
