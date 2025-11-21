use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn tracing_subscriber_handler(max_level: Level) {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(max_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

fn main() {
    tracing_subscriber_handler(Level::INFO);
    info!("Hello World");
}
