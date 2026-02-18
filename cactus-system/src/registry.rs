use cactus_foundation::fragment::Fragment;
use cactus_foundation::function::Function;
use cactus_interpreter::worker_pool::WorkerPool;
use std::collections::HashMap;
use tracing::info;

pub struct Registry {
    ledger: HashMap<String, WorkerPool>,
    registered: Vec<Function>,
    unregistered: Vec<Function>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            ledger: HashMap::new(),
            registered: Vec::new(),
            unregistered: Vec::new(),
        }
    }

    pub fn register_registered(&mut self, fragments: Vec<Fragment>, function: Function) {
        self.ledger.insert(
            function.name().to_string(),
            WorkerPool::new(fragments, function.name().to_string(), 4),
        );
        info!("{} registered new registration", function.name());

        self.registered.push(function);
    }

    pub fn register_unregistered(&mut self, function: Function) {
        self.unregistered.push(function);
    }

    pub fn get_registered(&self) -> &[Function] {
        &self.registered
    }

    pub fn get_unregistered(&self) -> &[Function] {
        &self.unregistered
    }

    pub fn get_worker_pool(&self, name: &str) -> Option<&WorkerPool> {
        self.ledger.get(name)
    }
}
