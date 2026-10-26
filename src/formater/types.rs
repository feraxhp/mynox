use std::fmt::Display;

use crate::formater::markdown::Colors;
use crate::formater::markdown::decorator_parser;


pub(super) struct Color(pub(crate) String);
pub(super) struct Plain(pub(crate) String);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&self.0[4..]).fmt(f)
    }
}
impl Display for Plain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&self.0[4..]).fmt(f)
    }
}

impl Color { pub(crate) fn new(string: String, base: Colors) -> Self { Self(process(string, base, true)) } }
impl Plain { pub(crate) fn new(string: String) -> Self { Self(process(string, Colors::None, false)) } }

pub(crate) fn process(string: String, base_color: Colors, colored: bool) -> String {
    let base_seq = base_color.get_escape_sequence(colored);
    let parsed = decorator_parser(&string, base_seq, colored);
    
    format!("{}{}\x1b[0m", base_seq, parsed)
}
