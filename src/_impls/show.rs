
use crate::Error;
use crate::traits::Show;


impl<I> Show for I
where 
    I: Iterator<Item = String>,
{
    fn print(&mut self) {
        for value in self.by_ref() { 
            println!("{value}") 
        }
    }
    fn eprint(&mut self) {
        for value in self.by_ref() { 
            eprintln!("{value}") 
        }
    }
}

impl Error {
    pub fn print(&self) {
        for value in self.render_std_out_auto() { 
            println!("{value}") 
        }
    }
    pub fn eprint(&self) {
        for value in self.render_std_out_auto() { 
            eprintln!("{value}") 
        }
    }
}
