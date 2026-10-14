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

    pub fn render_std_out_auto(&self) -> Box<dyn Iterator<Item = String> + '_> {
        if should_use_color_std_out() { Box::new(self.render_color()) } 
        else { Box::new(self.render_plain()) }
    }
    
    pub fn render_std_err_auto(&self) -> Box<dyn Iterator<Item = String> + '_> {
        if should_use_color_std_err() { Box::new(self.render_color()) } 
        else { Box::new(self.render_plain()) }
    }
}
