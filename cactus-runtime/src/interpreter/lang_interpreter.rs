use crate::discovery::lang::Lang;
use crate::{
    fragment::fragment::Fragment,
    function::{argument::Argument, function::Function},
};

pub trait LangInterpreter {
    fn new() -> Self
    where
        Self: Sized;
    fn lang(&self) -> &Lang;
    fn execute(&self, fragments: &[Fragment], function: &Function, args: &[Argument]);
    fn is_entrypoint(&self, fragments: &[Fragment], function: &Function) -> bool;
}
