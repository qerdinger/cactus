use crate::lang_selector::LangSelector;
use cactus_foundation::fragment::Fragment;

pub struct FragmentExtractor;

impl FragmentExtractor {
    pub fn extract(fragment: &mut Fragment) {
        if fragment.has_functions() {
            return;
        }

        let reader = LangSelector::get_language_reader(fragment);
        let functions = reader.extract(fragment);
        fragment.set_functions(functions);
    }
}
