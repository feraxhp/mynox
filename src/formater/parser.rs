use std::fmt::Display;


#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Itallic,
    Default,
    Reset,
    None,
}

impl Colors {
    fn colored(&self, bright: bool) -> &'static str {
        match self {
            // Basic ANSI foreground colors
            Colors::Black             => "\x1b[30m",
            Colors::Red     if bright => "\x1b[91m",
            Colors::Red               => "\x1b[31m",
            Colors::Green   if bright => "\x1b[92m",
            Colors::Green             => "\x1b[32m",
            Colors::Yellow  if bright => "\x1b[93m",
            Colors::Yellow            => "\x1b[33m",
            Colors::Blue    if bright => "\x1b[94m",
            Colors::Blue              => "\x1b[34m",
            Colors::Magenta if bright => "\x1b[95m",
            Colors::Magenta           => "\x1b[35m",
            Colors::Cyan    if bright => "\x1b[96m",
            Colors::Cyan              => "\x1b[36m",
            Colors::White   if bright => "\x1b[97m",
            Colors::White             => "\x1b[37m",
        
            // Aditional secuences
            Colors::Dim           => "\x1b[2m",
            Colors::Itallic       => "\x1b[3m",
            Colors::DarkGray      => "\x1b[90m",
        
            // Formatting reset and empty
            Colors::Default       => "\x1b[39m",
            Colors::Reset         => "\x1b[0m",
            Colors::None          => "",
        }
    }
    
    pub(crate) fn get_escape_sequence(&self, colored: bool, bright: bool) -> &'static str {
        match colored {
            true => self.colored(bright),
            false => "",
        }
    }
}

impl Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() { write!(f, "{}", self.get_escape_sequence(false, false)) }
        else { write!(f, "{}", self.get_escape_sequence(true, false)) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TagKind {
    Bold,      // ** -> Blue
    Code,      // `  -> Magenta
    Cyan,      // *  -> Cyan
    Red,       // -  -> Red
    Green,     // +  -> Green
    Yellow,    // =  -> Yellow
    DarkGray,  // ~~ -> DarkGray
    Bright,    // ^  -> Bright modifier
    Italic,    // _  -> Italic modifier
    Dim,       // ~  -> Dim modifier
}

impl TagKind {
    fn delimiter(&self) -> &'static str {
        match self {
            TagKind::Bold => "**",
            TagKind::Code => "`",
            TagKind::Cyan => "*",
            TagKind::Red => "-",
            TagKind::Green => "+",
            TagKind::Yellow => "=",
            TagKind::DarkGray => "~~",
            TagKind::Bright => "^",
            TagKind::Italic => "_",
            TagKind::Dim => "~",
        }
    }

    fn color(&self) -> Option<Colors> {
        match self {
            TagKind::Bold => Some(Colors::Blue),
            TagKind::Code => Some(Colors::Magenta),
            TagKind::Cyan => Some(Colors::Cyan),
            TagKind::Red => Some(Colors::Red),
            TagKind::Green => Some(Colors::Green),
            TagKind::Yellow => Some(Colors::Yellow),
            TagKind::DarkGray => Some(Colors::DarkGray),
            TagKind::Bright | TagKind::Italic | TagKind::Dim => None,
        }
    }
}

// Delimiters ordered by priority/length to avoid prefix conflicts (e.g. `**` before `*`, `~~` before `~`)
const DELIMITERS: &[(&str, TagKind)] = &[
    ("**", TagKind::Bold),
    ("~~", TagKind::DarkGray),
    ("`", TagKind::Code),
    ("*", TagKind::Cyan),
    ("-", TagKind::Red),
    ("+", TagKind::Green),
    ("=", TagKind::Yellow),
    ("^", TagKind::Bright),
    ("_", TagKind::Italic),
    ("~", TagKind::Dim),
];

const ESCAPES: &[(&str, &str)] = &[
    ("--", "-"),
    ("++", "+"),
    ("==", "="),
    ("^^", "^"),
    ("__", "_"),
    ("``", "`"),
];

fn match_escape(rem: &str) -> Option<(&'static str, usize)> {
    for &(esc, rep) in ESCAPES {
        if rem.starts_with(esc) {
            return Some((rep, esc.len()));
        }
    }
    None
}

fn candidate_delimiters(rem: &str) -> Vec<(&'static str, TagKind)> {
    if rem.starts_with("***") {
        let after_triple = &rem[3..];
        let pos_single = find_isolated_single_star(after_triple);
        let pos_double = after_triple.find("**");

        match (pos_single, pos_double) {
            (Some(s), Some(d)) if s < d => {
                // `***cyan* blue**`: inner is Cyan (*), outer is Bold (**)
                return vec![
                    ("**", TagKind::Bold),
                    ("*", TagKind::Cyan),
                ];
            }
            _ => {
                // `***blue** cyan*`: inner is Bold (**), outer is Cyan (*)
                return vec![
                    ("*", TagKind::Cyan),
                    ("**", TagKind::Bold),
                ];
            }
        }
    }

    for &(delim, kind) in DELIMITERS {
        if rem.starts_with(delim) {
            return vec![(delim, kind)];
        }
    }
    Vec::new()
}

fn find_isolated_single_star(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'*' {
            let is_prev_star = i > 0 && bytes[i - 1] == b'*';
            let is_next_star = i + 1 < bytes.len() && bytes[i + 1] == b'*';
            if !is_prev_star && !is_next_star {
                return Some(i);
            }
            if is_next_star {
                i += 1;
                while i < bytes.len() && bytes[i] == b'*' {
                    i += 1;
                }
                continue;
            }
        }
        i += 1;
    }
    None
}

/// AST Node for formatted text
#[derive(Debug)]
enum Node<'a> {
    Text(&'a str),
    Tagged {
        kind: TagKind,
        children: Vec<Node<'a>>,
    },
}

/// Recursively parses text into AST nodes respecting strict hierarchical nesting.
/// If an opening tag does not properly close hierarchically, it is treated as literal text.
fn parse_tokens<'a>(
    input: &'a str,
    mut idx: usize,
    end_idx: usize,
    stop_kind: Option<TagKind>,
) -> (Vec<Node<'a>>, usize, bool) {
    let mut nodes = Vec::new();
    let mut text_start = idx;

    while idx < end_idx {
        let rem = &input[idx..end_idx];

        // 0. Check for escape sequences (e.g. `--` -> `-`, `++` -> `+`, `==` -> `=`)
        if let Some((rep, esc_len)) = match_escape(rem) {
            if text_start < idx {
                nodes.push(Node::Text(&input[text_start..idx]));
            }
            nodes.push(Node::Text(rep));
            idx += esc_len;
            text_start = idx;
            continue;
        }

        // 1. Check if current position matches the closing delimiter of stop_kind
        if let Some(stop) = stop_kind {
            let delim = stop.delimiter();
            if rem.starts_with(delim) {
                // If stop is `*` (Cyan) and rem starts with `**`, do NOT close `*`
                // because `**` is Bold.
                let should_close = if stop == TagKind::Cyan && rem.starts_with("**") {
                    false
                } else {
                    true
                };

                if should_close {
                    if text_start < idx {
                        nodes.push(Node::Text(&input[text_start..idx]));
                    }
                    return (nodes, idx + delim.len(), true);
                }
            }
        }

        // 2. Try opening candidate tags
        let candidates = candidate_delimiters(rem);
        let mut matched_candidate = None;

        for (delim, kind) in candidates {
            let next_idx = idx + delim.len();
            let (child_nodes, closed_idx, matched) = parse_tokens(input, next_idx, end_idx, Some(kind));

            if matched {
                matched_candidate = Some((kind, child_nodes, closed_idx));
                break;
            }
        }

        if let Some((kind, child_nodes, closed_idx)) = matched_candidate {
            if text_start < idx {
                nodes.push(Node::Text(&input[text_start..idx]));
            }
            nodes.push(Node::Tagged {
                kind,
                children: child_nodes,
            });
            idx = closed_idx;
            text_start = closed_idx;
            continue;
        }

        // 3. Fallback: if stop is Cyan (*) and rem starts with `**`,
        // and it could not open or close Bold, then close Cyan on the first `*`
        if let Some(stop) = stop_kind {
            if stop == TagKind::Cyan && rem.starts_with("*") {
                if text_start < idx {
                    nodes.push(Node::Text(&input[text_start..idx]));
                }
                return (nodes, idx + 1, true);
            }
        }

        // Normal character, advance one utf-8 character
        let ch = rem.chars().next().unwrap();
        idx += ch.len_utf8();
    }

    // End of input reached while expecting stop_kind -> failed to close
    if stop_kind.is_some() {
        return (Vec::new(), idx, false);
    }

    if text_start < end_idx {
        nodes.push(Node::Text(&input[text_start..end_idx]));
    }

    (nodes, idx, true)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StyleState<'a> {
    color: Option<Colors>,
    bright: bool,
    italic: bool,
    dim: bool,
    base_seq: &'a str,
}

impl<'a> StyleState<'a> {
    fn new(base_seq: &'a str) -> Self {
        Self {
            color: None,
            bright: false,
            italic: false,
            dim: false,
            base_seq,
        }
    }

    fn to_ansi(&self) -> String {
        let mut s = String::new();
        s.push_str(Colors::Reset.colored(false));
        s.push_str(self.base_seq);

        if let Some(c) = self.color {
            s.push_str(c.colored(self.bright));
        } else if self.bright {
            // Bright white/default foreground if bright is active without explicit color
            s.push_str(Colors::White.colored(true));
        }

        if self.italic {
            s.push_str(Colors::Itallic.colored(false));
        }

        if self.dim {
            s.push_str(Colors::Dim.colored(false));
        }

        s
    }
}

fn render_node<'a>(
    node: &Node,
    current_style: StyleState<'a>,
    colored: bool,
    result: &mut String,
    last_applied_style: &mut StyleState<'a>,
) {
    match node {
        Node::Text(txt) => {
            if colored && *last_applied_style != current_style {
                result.push_str(&current_style.to_ansi());
                *last_applied_style = current_style;
            }
            result.push_str(txt);
        }
        Node::Tagged { kind, children } => {
            if colored {
                let mut next_style = current_style;
                if let Some(col) = kind.color() {
                    next_style.color = Some(col);
                } else {
                    match kind {
                        TagKind::Bright => next_style.bright = true,
                        TagKind::Italic => next_style.italic = true,
                        TagKind::Dim => next_style.dim = true,
                        _ => {}
                    }
                }

                for child in children {
                    render_node(child, next_style, colored, result, last_applied_style);
                }
            } else {
                // In plain mode: keep formatting delimiters (Markdown), strip purely terminal modifiers (^ and ~)
                let print_delim = match kind {
                    TagKind::Bright | TagKind::Dim => false,
                    _ => true,
                };

                if print_delim {
                    result.push_str(kind.delimiter());
                }

                for child in children {
                    render_node(child, current_style, colored, result, last_applied_style);
                }

                if print_delim {
                    result.push_str(kind.delimiter());
                }
            }
        }
    }
}

pub fn decorator_parser(input: &str, base_seq: &str, colored: bool) -> String {
    let (ast, _, _) = parse_tokens(input, 0, input.len(), None);

    let mut result = String::with_capacity(input.len());
    let initial_style = StyleState::new(base_seq);
    let mut last_applied_style = initial_style;

    for node in &ast {
        render_node(node, initial_style, colored, &mut result, &mut last_applied_style);
    }

    if colored && last_applied_style != initial_style {
        result.push_str(&initial_style.to_ansi());
    }

    result
}

#[macro_export]
macro_rules! show { ($string:literal $(, $base:expr )?) => { println!("{}", decorator_parser($string, None $(.or(Some($base)) )? .unwrap_or(Colors::None).colored(false), true)) }; }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visual() {
        // Run with `cargo test visual -- --show-output`
        
        // Base color
        show!("\x1b[34mSomething", Colors::Blue);
        println!("-> Unset {}", Colors::Reset.colored(false));
        show!("\x1b[34mSome in blue -then red- then blue again", Colors::Blue);
        println!("-> Unset {}", Colors::Reset.colored(false));
        
        show!("-red content ^bright red^-");
        show!("-red content ~dimmed red~-");
        show!("*cyan content ~dimmed cyan~*");
        show!("~All dimmed **blue** =yellow=~");
        show!("not dimmed **blue** =yellow=");
        show!("colors: **blue** *cyan* -red- +green+ =yellow= `magenta`");
        show!("brigth: ^**blue** *cyan* -red- +green+ =yellow= `magenta`^");
        show!("dimmed: ~**blue** *cyan* -red- +green+ =yellow= `magenta`~");
        show!("half  : ~**blue** *cyan* -red-~^ +green+ =yellow= `magenta`^");
        show!("-+=`***blue** cyan* magenta` yellow= green+ red-");
        show!("-+=`***cyan* blue** magenta` yellow= green+ red-");
        show!("**you = test**");
        show!("*3 -- 4 = -1-*");
    }
    
    #[test]
    fn plain_mode_preserves_markdown_removes_bright_dim() {
        assert_eq!(decorator_parser("**text**", "", false), "**text**");
        assert_eq!(decorator_parser("**^text^**", "", false), "**text**");
        assert_eq!(decorator_parser("`code`", "", false), "`code`");
        assert_eq!(decorator_parser("*cyan*", "", false), "*cyan*");
        assert_eq!(decorator_parser("_italic_", "", false), "_italic_");
        assert_eq!(decorator_parser("-red-", "", false), "-red-");
        assert_eq!(decorator_parser("+green+", "", false), "+green+");
        assert_eq!(decorator_parser("=yellow=", "", false), "=yellow=");
        assert_eq!(decorator_parser("~~darkgray~~", "", false), "~~darkgray~~");
        assert_eq!(decorator_parser("~dim~", "", false), "dim");
    }

    #[test]
    fn unclosed_tags_remain_literal() {
        assert_eq!(decorator_parser("**text", "", false), "**text");
        assert_eq!(decorator_parser("= algo", "", false), "= algo");
        assert_eq!(decorator_parser("Invalid character '^' at position 15", "", false), "Invalid character '^' at position 15");
        assert_eq!(decorator_parser("**text", "", true), "**text");
        assert_eq!(decorator_parser("= algo", "", true), "= algo");
    }

    #[test]
    fn interleaved_tags_hierarchical() {
        // ^text **some^ting** -> should not match crossed tags
        let plain = decorator_parser("^text **some^ting**", "", false);
        assert_eq!(plain, "^text **some^ting**");
    }

    #[test]
    fn colored_rendering() {
        let colored = decorator_parser("**text**", "", true);
        assert_eq!(colored, format!("{}{}{}", "\x1b[0m\x1b[34m", "text", "\x1b[0m"));

        let bright_blue = decorator_parser("**^text^**", "", true);
        assert_eq!(bright_blue, format!("{}{}{}", "\x1b[0m\x1b[94m", "text", "\x1b[0m"));
    }

    #[test]
    fn nested_colors() {
        let res = decorator_parser("**blue -red- blue**", "", true);
        let blue_ansi = "\x1b[0m\x1b[34m";
        let red_ansi = "\x1b[0m\x1b[31m";
        let restore_blue = "\x1b[0m\x1b[34m";
        let final_restore = "\x1b[0m";
        assert_eq!(
            res,
            format!("{}blue {}red{} blue{}", blue_ansi, red_ansi, restore_blue, final_restore)
        );
    }

    #[test]
    fn escape_sequences_plain() {
        assert_eq!(decorator_parser("--", "", false), "-");
        assert_eq!(decorator_parser("++", "", false), "+");
        assert_eq!(decorator_parser("==", "", false), "=");
        assert_eq!(decorator_parser("^^", "", false), "^");
        assert_eq!(decorator_parser("__", "", false), "_");
        assert_eq!(decorator_parser("``", "", false), "`");
        assert_eq!(decorator_parser("*3 -- 4 = -1-*", "", false), "*3 - 4 = -1-*");
        assert_eq!(decorator_parser("-foo--bar-", "", false), "-foo-bar-");
        assert_eq!(decorator_parser("----", "", false), "--");
    }

    #[test]
    fn escape_sequences_colored() {
        let res = decorator_parser("*3 -- 4 = -1-*", "", true);
        let cyan_ansi = "\x1b[0m\x1b[36m";
        let red_ansi = "\x1b[0m\x1b[31m";
        let restore = "\x1b[0m";
        assert_eq!(
            res,
            format!("{}3 - 4 = {}1{}", cyan_ansi, red_ansi, restore)
        );

        let res_red = decorator_parser("-foo--bar-", "", true);
        assert_eq!(
            res_red,
            format!("{}foo-bar{}", red_ansi, restore)
        );
    }

    #[test]
    fn user_examples() {
        // **^text^** -> bright_blue(text)
        assert_eq!(
            decorator_parser("**^text^**", "", true),
            format!("{}text{}", "\x1b[0m\x1b[94m", "\x1b[0m")
        );

        // **^text^ someting** -> bright_blue(text) blue( someting)
        assert_eq!(
            decorator_parser("**^text^ someting**", "", true),
            format!("{}text{} someting{}", "\x1b[0m\x1b[94m", "\x1b[0m\x1b[34m", "\x1b[0m")
        );

        // ^text **someting**^ -> normal(text ) bright_blue(someting)
        assert_eq!(
            decorator_parser("^text **someting**^", "", true),
            format!("{}text {}someting{}", "\x1b[0m\x1b[97m", "\x1b[0m\x1b[94m", "\x1b[0m")
        );
    }

    #[test]
    fn deeply_nested_all_delimiters() {
        let deep = "-+=`***blue** cyan* magenta` yellow= green+ red-";
        let deep_res = decorator_parser(deep, "", true);
        assert!(deep_res.contains("\x1b[34mblue"));
        assert!(deep_res.contains("\x1b[36m cyan"));
        assert!(deep_res.contains("\x1b[35m magenta"));
        assert!(deep_res.contains("\x1b[33m yellow"));
        assert!(deep_res.contains("\x1b[32m green"));
        assert!(deep_res.contains("\x1b[31m red"));

        let deep2 = "-+=`***cyan* blue** magenta` yellow= green+ red-";
        let deep2_res = decorator_parser(deep2, "", true);
        assert!(deep2_res.contains("\x1b[36mcyan"));
        assert!(deep2_res.contains("\x1b[34m blue"));
        assert!(deep2_res.contains("\x1b[35m magenta"));
        assert!(deep2_res.contains("\x1b[33m yellow"));
        assert!(deep2_res.contains("\x1b[32m green"));
        assert!(deep2_res.contains("\x1b[31m red"));
    }
}
