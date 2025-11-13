use tracing::{Level, event};

pub trait Logger {
    fn init(&self);
    fn log(&self, s: &str);
}


pub struct TracingReceiver;
impl TracingReceiver {
    fn init() {
        tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
}
}

pub struct TracingLogger;
impl Logger for TracingLogger {
    fn init(&self) {
        TracingReceiver::init();
    }

    #[inline(always)]
    fn log(&self, s: &str) {
        event!(Level::TRACE, "{}", s);
    }
}

#[macro_export]
macro_rules! log {
    ($logger:expr, $($arg:expr),+ $(,)?) => {
        {
            let msg = [$($arg),+].join(" ");
            $logger.log(&msg);
        }
    };
}