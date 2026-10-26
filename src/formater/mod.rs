pub mod parser;
pub mod printer;
pub mod render;
pub mod types;

use std::fmt::Display;

use crate::Formater;
use crate::traits::Printer;


impl<D> Formater for D
where 
    D: Display
{
    fn as_command(self) ->          String { format!("aaa:{}", self) }
    fn as_tip(self) ->              String { format!("aab:{}", self) }
    fn as_tip_cotinuation(self) ->  String { format!("aac:{}", self) }
}

pub(crate) fn transform<D: Printer>(input: D) -> String {
    let input_ = input.as_ref();
    let mut parts = input_.split(":");
    
    match parts.next(){
        Some(k) => {
            match k {
                "aaa" => input.as_command(),
                "aab" => input.as_tip(),
                "aac" => input.as_tip_cotinuation(),
                _ => input_.to_owned()
            }
        },
        None => input_.to_owned(),
    }
}
