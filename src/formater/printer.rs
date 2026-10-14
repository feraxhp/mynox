use color_print::cformat;

use crate::formater::types::Color;
use crate::formater::types::Plain;
use crate::traits::Printer;


impl Printer for Color {
    fn as_tip(&self)             -> String { cformat!("<g>✓</> {}", self) }
    fn as_tip_cotinuation(&self) -> String { cformat!("  {}", self) }
    fn as_command(&self)         -> String { cformat!("  <c>•</><g> {}</>", self) }
    
    fn as_ref<'a>(&'a self) -> &'a String { &self.0 }
}

impl Printer for Plain {
    fn as_tip(&self)             -> String {  format!("✓ {}", self) }
    fn as_tip_cotinuation(&self) -> String {  format!("  {}", self) }
    fn as_command(&self)         -> String {  format!("  • {}", self) }
    
    fn as_ref<'a>(&'a self) -> &'a String { &self.0 }
}
