use crate::{fragment::fragment::Fragment, function::{argument::Argument, function::Function}};

pub trait LangInterpreter {
    fn execute(fragments: &[Fragment], function: &Function, args: &[Argument]);
    fn is_entrypoint(fragments: &[Fragment], function: &Function) -> bool;
}
