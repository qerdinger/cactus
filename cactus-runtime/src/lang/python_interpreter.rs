use crate::function::argument::Argument;
use crate::function::function::Function;
use crate::lang::lang_interpreter::LangInterpreter;
pub struct PythonInterpreter;

impl LangInterpreter for PythonInterpreter {
    fn new(&self) -> PythonInterpreter {
        Self
    }

    fn execute(&self, function: &Function, args: &[Argument]) {
    }

    fn is_entrypoint(&self, function: &Function) -> bool {
        false
    }
}