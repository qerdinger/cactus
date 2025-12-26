use crate::function::{argument::Argument, function::Function};

pub trait LangInterpreter {
    fn new(&self) -> Self;
    fn execute(&self, function: &Function, args: &[Argument]);
    fn is_entrypoint(&self, function: &Function) -> bool;
}
