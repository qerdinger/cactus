use crate::discovery::lang::{Lang, Language};
use crate::fragment::fragment::Fragment;
use crate::lang::lang_reader::LangReader;
use crate::lang::python_reader::PythonReader;

pub struct LangSelector;

impl LangSelector {
    fn get_lang(raw_data: &str) -> Lang {
        Lang::new(Language::Python)
    }

    pub fn get_language_reader(fragment: &Fragment) -> Box<dyn LangReader> {
        let reader = match Self::get_lang(fragment.name()).language() {
            Language::Python => PythonReader::new(),
            _ => panic!("Language not supported!"),
        };

        Box::new(reader)
    }
}
