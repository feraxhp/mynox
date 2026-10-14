use std::env;
use std::io::{self, IsTerminal};
use std::sync::OnceLock;

pub fn should_use_color_std_out() -> bool {
    static USE_COLOR_STD_OUT: OnceLock<bool> = OnceLock::new();

    *USE_COLOR_STD_OUT.get_or_init(|| {
        if env::var_os("NO_COLOR").is_some_and(|val| !val.is_empty()) {
            return false;
        }
        if env::var_os("CLICOLOR_FORCE").is_some_and(|val| !val.is_empty() && val != "0") {
            return true;
        }
        if env::var_os("TERM").is_some_and(|term| term == "dumb") {
            return false;
        }
        io::stdout().is_terminal()
    })
}

pub fn should_use_color_std_err() -> bool {
    static USE_COLOR_STD_ERR: OnceLock<bool> = OnceLock::new();

    *USE_COLOR_STD_ERR.get_or_init(|| {
        if env::var_os("NO_COLOR").is_some_and(|val| !val.is_empty()) {
            return false;
        }
        if env::var_os("CLICOLOR_FORCE").is_some_and(|val| !val.is_empty() && val != "0") {
            return true;
        }
        if env::var_os("TERM").is_some_and(|term| term == "dumb") {
            return false;
        }
        io::stdout().is_terminal()
    })
}
