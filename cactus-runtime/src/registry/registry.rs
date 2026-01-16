use crate::function::function::Function;

pub struct Registry {
    registered: Vec<Function>,
    unregistered: Vec<Function>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            registered: Vec::new(),
            unregistered: Vec::new(),
        }
    }

    pub fn add_registered(&mut self, function: Function) {
        self.registered.push(function);
    }

    pub fn add_unregistered(&mut self, function: Function) {
        self.unregistered.push(function);
    }

    pub fn get_registered(&self) -> &[Function] {
        &self.registered
    }

    pub fn get_unregistered(&self) -> &[Function] {
        &self.unregistered
    }
}
