use crate::function::Function;

#[derive(Clone)]
pub struct Fragment {
    name: String,
    raw_data: String,

    functions: Option<Vec<Function>>,
}

impl Fragment {
    pub fn new(name: String, raw_data: String) -> Self {
        Self {
            name,
            raw_data,
            functions: None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn raw_data(&self) -> &str {
        &self.raw_data
    }

    pub fn functions(&self) -> Option<&[Function]> {
        self.functions.as_deref()
    }

    pub fn functions_mut(&mut self) -> Option<&mut Vec<Function>> {
        self.functions.as_mut()
    }

    pub fn set_functions(&mut self, functions: Vec<Function>) {
        self.functions = Some(functions);
    }

    pub fn has_functions(&self) -> bool {
        self.functions.is_some()
    }
}
