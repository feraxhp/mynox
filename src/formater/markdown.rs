use std::fmt::Display;


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
    Default,
    Reset,
    None,
}

impl Colors {
    pub(crate) fn get_escape_sequence(&self) -> &'static str {
        match self {
            // Basic ANSI foreground colors
            Colors::Black   => "\x1b[30m",
            Colors::Red     => "\x1b[31m",
            Colors::Green   => "\x1b[32m",
            Colors::Yellow  => "\x1b[33m",
            Colors::Blue    => "\x1b[34m",
            Colors::Magenta => "\x1b[35m",
            Colors::Cyan    => "\x1b[36m",
            Colors::White   => "\x1b[37m",
            
            // Default foreground color
            Colors::Default => "\x1b[39m",
        
            // Formatting reset and empty
            Colors::Reset   => "\x1b[0m",
            Colors::None    => "",
        }
    }
}

impl Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_escape_sequence())
    }
}

pub fn parse_markdown_inline(input: &str, base_seq: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.char_indices().peekable();

    let restore_seq = format!("{}{}", Colors::Reset, base_seq);

    while let Some((_, c)) = chars.next() {
        match c {
            '`' => { // Inline code: magenta
                result.push_str(Colors::Magenta.get_escape_sequence());
                let mut closed = false;
                while let Some((_, inner_c)) = chars.next() {
                    if inner_c == '`' {
                        closed = true;
                        break;
                    }
                    result.push(inner_c);
                }
                if closed {
                    result.push_str(&restore_seq);
                }
            }
            '*' => { // Check if it's bold (**text**) or italic (*text*)
                if let Some(&(_, '*')) = chars.peek() {
                    // Bold: blue
                    chars.next(); // Consume second '*'
                    result.push_str(Colors::Blue.get_escape_sequence());
                    let mut closed = false;
                    while let Some((_, inner_c)) = chars.next() {
                        if inner_c == '*' {
                            if let Some(&(_, '*')) = chars.peek() {
                                chars.next(); // Consume second '*'
                                closed = true;
                                break;
                            }
                        }
                        result.push(inner_c);
                    }
                    if closed {
                        result.push_str(&restore_seq);
                    }
                } else { // Italic: italic
                    result.push_str("\x1b[3m");
                    let mut closed = false;
                    while let Some((_, inner_c)) = chars.next() {
                        if inner_c == '*' {
                            closed = true;
                            break;
                        }
                        result.push(inner_c);
                    }
                    if closed {
                        result.push_str(&restore_seq);
                    }
                }
            }
            _ => {
                result.push(c);
            }
        }
    }

    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_code_no_base_color() {
        let input = "Here is `some code` inline";
        let base_seq = "";
        let expected = "Here is \x1b[35msome code\x1b[0m inline";
        assert_eq!(parse_markdown_inline(input, base_seq), expected);
    }

    #[test]
    fn test_bold_no_base_color() {
        let input = "This is **bold text**";
        let base_seq = "";
        let expected = "This is \x1b[34mbold text\x1b[0m";
        assert_eq!(parse_markdown_inline(input, base_seq), expected);
    }

    #[test]
    fn test_italic_no_base_color() {
        let input = "This is *italic text*";
        let base_seq = "";
        let expected = "This is \x1b[3mitalic text\x1b[0m";
        assert_eq!(parse_markdown_inline(input, base_seq), expected);
    }

    #[test]
    fn test_mixed_no_base_color() {
        let input = "A *mix* of **bold** and `code`";
        let base_seq = "";
        let expected = "A \x1b[3mmix\x1b[0m of \x1b[34mbold\x1b[0m and \x1b[35mcode\x1b[0m";
        assert_eq!(parse_markdown_inline(input, base_seq), expected);
    }

    #[test]
    fn test_with_red_base_color() {
        let input = "Hello **world**!";
        let base_seq = Colors::Red.get_escape_sequence(); // "\x1b[31m"
        
        // It should restore the red color (\x1b[31m) after the bold text finishes
        let expected = "Hello \x1b[34mworld\x1b[0m\x1b[31m!";
        assert_eq!(parse_markdown_inline(input, base_seq), expected);
    }

    #[test]
    fn test_unclosed_tags_ignored() {
        // If a tag is unclosed, it simply drops the rest of the text as per the current parser
        // This test ensures we know the current behavior
        let input = "This `code never ends";
        let base_seq = "";
        let expected = "This \x1b[35mcode never ends"; // No restore sequence because it was never closed
        assert_eq!(parse_markdown_inline(input, base_seq), expected);
    }
}
