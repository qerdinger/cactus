use tracing::{info, Level};
use tracing_subscriber::fmt::init;
use tracing_subscriber::FmtSubscriber;

mod discovery;
mod function;
mod fragment;
use crate::discovery::discover::Discover;
use crate::discovery::lang::{Lang, Language};
use crate::function::function::Function;

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
    let fragments = disc.lookup();

    info!("{} fragment(s) discovered", fragments.len());
}
