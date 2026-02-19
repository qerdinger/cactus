use cactus_foundation::fragment::Fragment;
use cactus_foundation::function::Function;
use cactus_interpreter::parallel_worker::ParallelWorker;
use cactus_interpreter::worker_pool::WorkerPool;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

pub struct Registry {
    thread_ledger: HashMap<String, WorkerPool>,
    parallel_ledger: HashMap<String, Arc<ParallelWorker>>,
    registered: Vec<Function>,
    unregistered: Vec<Function>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            thread_ledger: HashMap::new(),
            parallel_ledger: HashMap::new(),
            registered: Vec::new(),
            unregistered: Vec::new(),
        }
    }

    pub fn register_registered(&mut self, fragments: Vec<Fragment>, function: Function) {
        self.thread_ledger.insert(
            function.name().to_string(),
            WorkerPool::new(fragments, function.name().to_string(), 4),
        );
        info!("{} registered new registration (thread pool)", function.name());

        self.registered.push(function);
    }

    pub fn register_parallel(&mut self, fragments: Vec<Fragment>, function: Function) {
        self.parallel_ledger.insert(
            function.name().to_string(),
            Arc::new(ParallelWorker::new(fragments, function.name().to_string(), 4)),
        );
        info!("{} registered new registration (parallel pool)", function.name());

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
        self.thread_ledger.get(name)
    }

    pub fn get_parallel_worker(&self, name: &str) -> Option<Arc<ParallelWorker>> {
        self.parallel_ledger.get(name).cloned()
    }
}
