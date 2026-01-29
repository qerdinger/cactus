use cactus_foundation::argument::Argument;
use cactus_foundation::fragment::Fragment;
use cactus_foundation::function::Function;
use cactus_foundation::lang::Lang;

pub trait LangInterpreter {
    fn new() -> Self
    where
        Self: Sized;
    fn lang(&self) -> &Lang;
    fn execute(&self, fragments: &[Fragment], function: &Function, args: &[Argument]);
    fn is_entrypoint(&self, fragments: &[Fragment], function: &Function) -> bool;
}
