use std::env;
use mynox::formater::parser::decorator_parser;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let plain = args.iter().any(|arg| arg == "--plain" || arg == "-p");
    let filtered_args: Vec<String> = args
        .into_iter()
        .filter(|arg| arg != "--plain" && arg != "-p")
        .collect();

    let colored = !plain;

    if !filtered_args.is_empty() {
        let text = filtered_args.join(" ");
        println!("{}", decorator_parser(&text, "", colored));
        return;
    }
    
    println!("{}", decorator_parser("Usage: `cargo run --example colors [--plain|-p] -- '<sequence>'`", "", colored));
    println!("Examples (mode: {}):", if colored { "colored" } else { "plain" });
    println!("  *3 -- 4 = -1-*:\n  -> '{}'", decorator_parser("*3 -- 4 = -1-*", "", colored));
    println!("  -red- +green+ =yellow= `code` *cyan* **blue**:\n  -> '{}'", decorator_parser("-red- +green+ =yellow= `code` *cyan* **blue**", "", colored));
    println!("  ^**bright blue**^ ~dimmed~ ~~darkgray~~:\n  -> '{}'", decorator_parser("^**bright blue**^ ~dimmed~ ~~darkgray~~", "", colored));
    println!("  **blue <raw> -blue- # + blue + </raw>**:\n  -> '{}'", decorator_parser("**This text is blue <raw> -this should be blue- # + this also should be blue + </raw>**", "", colored));
}
