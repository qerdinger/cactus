use std::env;
use tracing::{error, info, Level};
use tracing_subscriber::fmt::init;
use tracing_subscriber::FmtSubscriber;

mod discovery;
mod fragment;
mod function;
mod interpreter;
mod lang;
mod registry;

use pyo3::prelude::*;
use pyo3::types::PyModule;

use crate::discovery::discover::Discover;
use crate::discovery::lang::{Lang, Language};
use crate::function::function::Function;

use crate::registry::registry::Registry;
use interpreter::lang_interpreter::LangInterpreter;
use interpreter::python_interpreter::PythonInterpreter;

fn tracing_subscriber_handler(max_level: Level) {
    let subscriber = FmtSubscriber::builder().with_max_level(max_level).finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    tracing_subscriber_handler(Level::INFO);
    info!("Cactus Runtime System");

    let disc = Discover();
    let mut fragments = disc.lookup();

    let mut registry = Registry::new();

    info!("{} fragment(s) discovered", fragments.len());
    let mut all_functions = Vec::new();

    for fragment in &mut fragments {
        if let Some(fncs) = fragment.functions_mut() {
            all_functions.extend(fncs.drain(..));
        }
    }

    for fnc in all_functions {
        if PythonInterpreter::is_entrypoint(&fragments, &fnc) {
            registry.add_registered(fnc);
        } else {
            registry.add_unregistered(fnc);
        }
    }

    info!("{} function(s) discovered", fragments.len());

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
