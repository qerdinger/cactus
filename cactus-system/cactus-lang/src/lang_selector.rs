use crate::lang_reader::LangReader;
use crate::langs::python_reader::PythonReader;
use cactus_foundation::fragment::Fragment;
use cactus_foundation::lang::{Lang, Language};

pub struct LangSelector;

impl LangSelector {
    fn get_lang(_raw_data: &str) -> Lang {
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
