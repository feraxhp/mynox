use std::iter::once;

use crate::Error;
use crate::NOTE_DIVIDER;
use crate::formater::transform;
use crate::formater::parser::Colors;
use crate::termianal::should_use_color_std_err;
use crate::termianal::should_use_color_std_out;
use super::types::Color;
use super::types::Plain;


impl Error {
    /// Renders the error details and notes into lines of plain uncolored text.
    ///
    /// Details are prefixed with `* ` (first line) or `  ` (subsequent lines),
    /// followed by the `# Notes` section divider and the notes.
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
    pub fn render_plain(&self) -> Box<dyn Iterator<Item = String> + '_> {
        let details_iter = self.details.iter().enumerate().map(|(idx, dt)| {
            let way = Plain::new(dt.to_owned());
            let dt = transform(way);
            
            if idx == 0 { format!("* {}", dt) }
            else        { format!("  {}", dt) }
        });
        
        let suberrors = self.is_collection().then(|| {
            self.errors.iter().enumerate().flat_map(|(idx, e)| {
                    let header = once(format!("\n  {}. {}", idx+1, e.message()));
                    let body = e.render_plain().map(|line| format!("     {line}"));
            
                    header.chain(body)
                })
        })
        .into_iter()
        .flatten();
        
        let notes_iter = (!self.notes.is_empty()).then(|| {
            once(String::new())
                .chain(once(NOTE_DIVIDER.to_string()))
                .chain(self.notes.iter().enumerate().map(|(idx, dt)| {
                    let way = Plain::new(dt.to_owned());
                    let dt = transform(way);
    
                    if idx == 0 { format!("  {}", dt) }
                    else        { format!("  {}", dt) }
                }))
        })
        .into_iter()
        .flatten(); // Si fue None, produce 0 elementos; si fue Some, produce las notas
        
        Box::new(details_iter.chain(suberrors).chain(notes_iter))
    }
    
    
    /// Renders the error details and notes into lines formatted with ANSI escape colors.
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
    pub fn render_color(&self) -> Box<dyn Iterator<Item = String> + '_> {
        let details_iter = self.details.iter().enumerate()
            .map(|(idx, dt)| {
                let color = if idx == 0 { Colors::Yellow } else { Colors::Yellow }; 
                let way = Color::new(dt.to_owned(), color);
                let dt = transform(way);
                
                if idx == 0 { format!("{}* {}{}", Colors::Yellow, dt, Colors::Reset) }
                else        { format!("  {}", dt) }
            });
        
        let suberrors = self.is_collection().then(|| {
            self.errors.iter().enumerate().flat_map(|(idx, e)| {
                    let header = once(format!("\n  {}{}. {}{}", Colors::Red, idx+1, e.message(), Colors::Reset));
                    let body = e.render_color().map(|line| format!("     {line}"));
            
                    header.chain(body)
                })
        })
        .into_iter()
        .flatten();
        
        let notes_iter = (!self.notes.is_empty()).then(|| {
            once(String::new())
                .chain(once(format!("{}{}{}", Colors::Green, NOTE_DIVIDER, Colors::Reset)))
                .chain(
                    self.notes.iter().enumerate().map(|(_idx, dt)| {
                        let way = Color::new(dt.to_owned(), Colors::None);
                        let dt = transform(way);
                        format!("  {}", dt)
                    })
                )
        })
        .into_iter()
        .flatten();
        
        Box::new(details_iter.chain(suberrors).chain(notes_iter))
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
