use crate::discovery::lang::{Lang, Language};
use crate::function::argument::Argument;
use crate::function::function::Function;
use crate::lang::lang_reader::LangReader;

pub struct PythonReader;

impl LangReader for PythonReader {
    fn extract(&self) -> Vec<Function> {
        vec![
            Function::new("entrypoint".to_owned(), Some(Lang::new(Language::Python)), vec![
                Argument::new("input1".to_owned(), None),
            ])
        ]
    }
}