use std::{collections::HashSet, fmt::{Debug, Display}};


#[allow(unused)]
pub(crate) enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Dim,
    DarkGray,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Itallic,
    Default,
    Reset,
    None,
}

impl Colors {
    fn colored(&self) -> &'static str {
        match self {
            // Basic ANSI foreground colors
            Colors::Black         => "\x1b[30m",
            Colors::Red           => "\x1b[31m",
            Colors::Green         => "\x1b[32m",
            Colors::Yellow        => "\x1b[33m",
            Colors::Blue          => "\x1b[34m",
            Colors::Magenta       => "\x1b[35m",
            Colors::Cyan          => "\x1b[36m",
            Colors::White         => "\x1b[37m",
        
            // Aditional secuences
            Colors::Dim           => "\x1b[2m",
            Colors::Itallic       => "\x1b[3m",
            Colors::DarkGray      => "\x1b[90m",
        
            // Bright / High-intensity foreground colors
            Colors::BrightRed     => "\x1b[91m",
            Colors::BrightGreen   => "\x1b[92m",
            Colors::BrightYellow  => "\x1b[93m",
            Colors::BrightBlue    => "\x1b[94m",
            Colors::BrightMagenta => "\x1b[95m",
            Colors::BrightCyan    => "\x1b[96m",
            Colors::BrightWhite   => "\x1b[97m",
        
            // Formatting reset and empty
            Colors::Default       => "\x1b[39m",
            Colors::Reset         => "\x1b[0m",
            Colors::None          => "",
        }
    }
    
    pub(crate) fn get_escape_sequence(&self, colored: bool) -> &'static str {
        match colored {
            true => self.colored(),
            false => todo!(),
        }
    }
}

impl Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() { write!(f, "{}", self.get_escape_sequence(false)) }
        else { write!(f, "{}", self.get_escape_sequence(true)) }
    }
}

macro_rules! decorate {
    ($close:literal $color:expr, $result:expr, $chars:expr, $restore_seq:expr, $colored:expr) => {{
        $result.push_str($color.get_escape_sequence($colored));
        let mut closed = false;
        while let Some((_, inner_c)) = $chars.next() {
            if inner_c == $close {
                closed = true;
                break;
            }
            $result.push(inner_c);
        }
        if closed { $result.push_str(&$restore_seq); }
    }};
    (peek $char:literal $color:expr, $result:expr, $chars:expr, $restore_seq:expr, $colored:expr) => {{
        $chars.next(); // Consume second '*'
        $result.push_str($color.get_escape_sequence($colored));
        let mut closed = false;
        while let Some((_, inner_c)) = $chars.next() {
            if inner_c == $char {
                if let Some(&(_, $char)) = $chars.peek() {
                    $chars.next(); // Consume second $char
                    closed = true;
                    break;
                }
            }
            $result.push(inner_c);
        }
        if closed { $result.push_str(&$restore_seq); }
    }};
    ($char:literal > $simple_color:expr, $extended_color:expr, $result:expr, $chars:expr, $restore_seq:expr, $colored:expr) => {{
        if let Some(&(_, $char)) = $chars.peek() { 
            decorate!(peek $char $extended_color, $result, $chars, $restore_seq, $colored)
        } 
        else { decorate!($char $simple_color, $result, $chars, $restore_seq, $colored) }
    }};
}

pub fn decorator_parser(input: &str, base_seq: &str, colored: bool) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.char_indices().peekable();
    
    let mut restore_seq = format!("{}{}", Colors::Reset, base_seq);
    
    // let mut opened = HashSet::new();
    while let Some((_, c)) = chars.next() {
        if colored {  }
        match c {
            '_' => decorate!('_' > Colors::Itallic, Colors::Blue, result, chars, restore_seq, colored),
            '*' => decorate!('*' > Colors::Itallic, Colors::Blue, result, chars, restore_seq, colored),
            '~' => decorate!('~' > Colors::Dim, Colors::DarkGray, result, chars, restore_seq, colored),
            '`' => decorate!('`' Colors::Magenta, result, chars, restore_seq, colored),
            '^' => {
                
            },
            _ => result.push(c),
        }
    }
    
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn terminal_look() {
        let input = "Here is ^**some code**^ **inline**";
        let base_seq = "";
        eprintln!("{}", decorator_parser(input, base_seq, true))
    }
    
    #[test]
    fn test_inline_code_no_base_color() {
        let input = "Here is `some code` inline";
        let base_seq = "";
        let expected = "Here is \x1b[35msome code\x1b[0m inline";
        assert_eq!(decorator_parser(input, base_seq, true), expected);
    }

    #[test]
    fn test_bold_no_base_color() {
        let input = "This is **bold text**";
        let base_seq = "";
        let expected = "This is \x1b[34mbold text\x1b[0m";
        assert_eq!(decorator_parser(input, base_seq, true), expected);
    }

    #[test]
    fn test_italic_no_base_color() {
        let input = "This is *italic text*";
        let base_seq = "";
        let expected = "This is \x1b[3mitalic text\x1b[0m";
        assert_eq!(decorator_parser(input, base_seq, true), expected);
    }

    #[test]
    fn test_mixed_no_base_color() {
        let input = "A *mix* of **bold** and `code`";
        let base_seq = "";
        let expected = "A \x1b[3mmix\x1b[0m of \x1b[34mbold\x1b[0m and \x1b[35mcode\x1b[0m";
        assert_eq!(decorator_parser(input, base_seq, true), expected);
    }

    #[test]
    fn test_with_red_base_color() {
        let input = "Hello **world**!";
        let base_seq = Colors::Red.get_escape_sequence(true); // "\x1b[31m"
        
        // It should restore the red color (\x1b[31m) after the bold text finishes
        let expected = "Hello \x1b[34mworld\x1b[0m\x1b[31m!";
        assert_eq!(decorator_parser(input, base_seq, true), expected);
    }

    #[test]
    fn test_unclosed_tags_ignored() {
        // If a tag is unclosed, it simply drops the rest of the text as per the current parser
        // This test ensures we know the current behavior
        let input = "This `code never ends";
        let base_seq = "";
        let expected = "This \x1b[35mcode never ends"; // No restore sequence because it was never closed
        assert_eq!(decorator_parser(input, base_seq, true), expected);
    }
}
