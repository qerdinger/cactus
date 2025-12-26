use std::env;
use tracing::{error, info, Level};
use tracing_subscriber::fmt::init;
use tracing_subscriber::FmtSubscriber;

mod discovery;
mod function;
mod fragment;
mod lang;

use pyo3::prelude::*;
use pyo3::types::PyModule;

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
    let args: Vec<String> = env::args().collect();
    tracing_subscriber_handler(Level::INFO);
    info!("Cactus Runtime System");

    let disc = Discover();
    let fragments = disc.lookup();

    info!("{} fragment(s) discovered", fragments.len());
    for mut fragment in fragments {
        info!("{}", fragment.name());
        info!("Extracting function(s)...");
        fragment.extract();
        if let Some(x) = fragment.functions() {
            for function in x {
                info!("{:?}", function);
            }
        }
    }

    /*
    Python::with_gil(|py| {
        let module = PyModule::import(py, "../examples/serverless");
        if module.is_err() {
            error!("Could not load the module");
            return;
        }
        let module = module.unwrap();

        let functions = vec![
            "simple_entrypoint",
            "entrypoint",
            "en_lang",
            "fr_lang"
        ];

        for func_name in functions {
            let func = module.getattr(func_name);

            if func.is_err() {
                continue;
            }
            let func = func.unwrap();

            // call function (no args)
            let result = func.call0();

            if result.is_err() {
                continue;
            }

            let result = result.unwrap();

            println!("Called {} - {:?}", func_name, result);
        }
    });
     */
}
