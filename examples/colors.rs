use std::env;
use mynox::formater::parser::decorator_parser;


fn print_formatted(input: &str) {
    let colored = decorator_parser(input, "", true);
    println!("{colored}");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if !args.is_empty() {
        let text = args.join(" ");
        print_formatted(&text);
        return;
    }
    
    println!("{}", decorator_parser("Usage: `cargo run --example colors ---- '-colored sequence-'`", "", true) );
    println!("Examples:");
    println!("  *3 -- 4 = -1-*:\n  -> '{}'", decorator_parser("*3 -- 4 = -1-*", "", true));
    println!("  -red- +green+ =yellow= `code` *cyan* **blue**:\n  -> '{}'", decorator_parser("-red- +green+ =yellow= `code` *cyan* **blue**", "", true));
    println!("  ^**bright blue**^ ~dimmed~ ~~darkgray~~:\n  -> '{}'", decorator_parser("^**bright blue**^ ~dimmed~ ~~darkgray~~", "", true));
    println!("  **blue <raw> -blue- # + blue + </raw>**:\n  -> '{}'", decorator_parser("**This text is blue <raw> -this should be blue- # + this also should be blue + </raw>**", "", true));
}
