use sdk::logger::{Logger, TracingLogger};

pub struct Core {
    pub log: Box<dyn Logger>,
}

impl Core {
    pub fn new() -> Self {
        let logger = TracingLogger;
        logger.init();

        Self { log: Box::new(logger) }
    } 
}