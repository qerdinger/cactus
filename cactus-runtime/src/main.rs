use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod discovery;
mod functions;

use crate::discovery::discover::Discover;
use crate::discovery::lang::{Lang, Language};
use crate::functions::function::Function;

fn tracing_subscriber_handler(max_level: Level) {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(max_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

fn main() {
    tracing_subscriber_handler(Level::INFO);
    info!("Cactus Runtime System");

    let disc = Discover();
    let functions = disc.lookup();

    for fnc in functions {
        info!("function name=[{}] discovered !", fnc.name)
    }
}
