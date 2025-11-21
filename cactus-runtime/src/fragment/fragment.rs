use crate::function::function::Function;

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
    
    pub fn extract(&self) {
        if self.functions.is_some() {
            return;
        }
    }
}