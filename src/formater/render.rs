use std::iter::once;

use color_print::cformat;

use crate::Error;
use crate::NOTE_DIVIDER;
use crate::formater::transform;
use crate::formater::markdown::Colors;
use crate::termianal::should_use_color_std_err;
use crate::termianal::should_use_color_std_out;
use super::types::Color;
use super::types::Plain;


impl Error {
    /// Renders the error details and notes into lines of plain uncolored text.
    ///
    /// Details are prefixed with `* ` (first line) or `  ` (subsequent lines),
    /// followed by the `## Notes` section divider and the notes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::error;
    ///
    /// let err = error!("auth::login", "Login failed", "Incorrect password" => "Use 'forgot password' to reset");
    /// let lines: Vec<String> = err.render_plain().collect();
    /// assert!(!lines.is_empty());
    /// ```
    pub fn render_plain(&self) -> impl Iterator<Item = String> {
        self.details.iter().enumerate()
            .map(|(idx, dt)| {
                let way = Plain::new(dt.to_owned());
                let dt = transform(way);
                
                if idx == 0 { format!("* {}", dt) }
                else        { format!("  {}", dt) }
            })
            .chain(once(String::new()))
            .chain(once(NOTE_DIVIDER.to_string()))
            .chain(
                self.notes.iter().enumerate()
                    .map(|(idx, dt)| {
                        let way = Plain::new(dt.to_owned());
                        let dt = transform(way);
                        
                        if idx == 0 { format!("* {}", dt) }
                        else        { format!("  {}", dt) }
                    })
            )
    }
    
    /// Renders the error details and notes into lines formatted with ANSI escape colors.
    ///
    /// Includes inline markdown parsing:
    /// - `` `code` `` rendered in magenta.
    /// - `**bold**` rendered in blue.
    /// - `*italic*` rendered in italics.
    /// - Special tip and command markers formatted accordingly.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mynox::error;
    ///
    /// let err = error!("net::tcp", "Connection failed", "Timeout connecting to `127.0.0.1:8080`");
    /// let lines: Vec<String> = err.render_color().collect();
    /// assert!(!lines.is_empty());
    /// ```
    pub fn render_color(&self) -> impl Iterator<Item = String> {
        self.details.iter().enumerate()
            .map(|(idx, dt)| {
                let way = Color::new(dt.to_owned(), Colors::Yellow);
                let dt = transform(way);
                
                if idx == 0 { cformat!("<y>* {}</>", dt) }
                else        { cformat!(   "  {}"   , dt) }
            })
            .chain(once(String::new()))
            .chain(once(cformat!("<g>{}</>", NOTE_DIVIDER)))
            .chain(
                self.notes.iter().enumerate()
                    .map(|(idx, dt)| {
                        let way = Color::new(dt.to_owned(), Colors::None);
                        let dt = transform(way);
                        
                        if idx == 0 { cformat!("{}", dt) }
                        else        { cformat!("{}", dt) }
                    })
            )
    }

    /// Automatically renders using colors if stdout is an interactive terminal and `NO_COLOR` is not set;
    /// otherwise falls back to plain uncolored text.
    pub fn render_std_out_auto(&self) -> Box<dyn Iterator<Item = String> + '_> {
        if should_use_color_std_out() { Box::new(self.render_color()) } 
        else { Box::new(self.render_plain()) }
    }
    
    /// Automatically renders using colors if stderr is an interactive terminal and `NO_COLOR` is not set;
    /// otherwise falls back to plain uncolored text.
    pub fn render_std_err_auto(&self) -> Box<dyn Iterator<Item = String> + '_> {
        if should_use_color_std_err() { Box::new(self.render_color()) } 
        else { Box::new(self.render_plain()) }
    }
}
