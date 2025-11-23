use crate::discovery::lang::Language;
use crate::lang::lang_reader::LangReader;
use crate::lang::python_reader::PythonReader;

pub struct LangSelector;

impl LangSelector {
    fn get_language(raw_data : &str) -> Language {
        Language::Python
    }

    fn get_language_reader(raw_data : &str) -> Box<dyn LangReader> {
        Box::new(PythonReader)
    }
}