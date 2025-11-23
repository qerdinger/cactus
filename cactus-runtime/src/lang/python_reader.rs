use crate::function::function::Function;
use crate::lang::lang_reader::LangReader;

pub struct PythonReader;

impl LangReader for PythonReader {
    fn extract(&self) -> Vec<Function> {
        vec![]
    }
}