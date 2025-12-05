use crate::function::function::Function;
use crate::lang::lang_selector::LangSelector;

pub struct Fragment {
    name: String,
    raw_data: String,
    
    functions: Option<Vec<Function>>,
}

impl Fragment {
    pub fn new(name: String, raw_data: String) -> Fragment {
        Self { name, raw_data, functions: None }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn raw_data(&self) -> &str {
        &self.raw_data
    }

    pub fn functions(&self) -> Option<&[Function]> {
        if let Some(functions) = &self.functions {
            Some(functions)
        } else { None }
    }
    
    pub fn extract(&mut self) {
        if self.functions.is_some() {
            return;
        }

        let reader = LangSelector::get_language_reader(&self);
        self.functions = Some(reader.extract(&self));

    }
}