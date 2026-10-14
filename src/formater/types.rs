use std::fmt::Display;

use crate::formater::markdown::Colors;
use crate::formater::markdown::parse_markdown_inline;


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

impl Color { pub(crate) fn new(string: String, base: Colors) -> Self { Self(process(string, base)) } }
impl Plain { pub(crate) fn new(string: String) -> Self { Self(string) } }

pub(crate) fn process(string: String, base_color: Colors) -> String {
    let base_seq = base_color.get_escape_sequence();
    let parsed = parse_markdown_inline(&string, base_seq);
    
    format!("{}{}\x1b[0m", base_seq, parsed)
}
