use std::env;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

mod discovery;
mod fragment;
mod function;
mod interpreter;
mod lang;
mod registry;

use crate::discovery::discover::Discover;

use crate::discovery::lang::{Lang, Language};
use crate::interpreter::interpreter_engine::InterpreterEngine;
use crate::registry::registry::Registry;
use interpreter::lang_interpreter::LangInterpreter;
use interpreter::langs::python_interpreter::PythonInterpreter;

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

    fragments.iter_mut().for_each(|frgmt| {
        if let Some(fncs) = frgmt.functions_mut() {
            all_functions.extend(fncs.drain(..));
        }
    });

    let mut interpreter_engine = InterpreterEngine::new();

    for _ in 0..=4 {
        interpreter_engine.register(PythonInterpreter::new());
    }

    for fnc in all_functions {
        if let (f_name, Some(f_lang)) = (fnc.name(), fnc.lang()) {
            let Some(is_entrypoint) = interpreter_engine
                .with_interpreter_for_lang(f_lang, |interp| interp.is_entrypoint(&fragments, &fnc))
            else {
                info!(
                    "{}: No interpreter available for lang (defined as [{:?}])",
                    f_name, f_lang
                );
                continue;
            };

            match is_entrypoint {
                true => registry.add_registered(fnc),
                false => registry.add_unregistered(fnc),
            }
        } else {
            info!(
                "{}: Language not defined (defined as [{:?}])",
                fnc.name(),
                fnc.lang()
            )
        }
    }

    info!(
        "{} function(s) registered / {} function(s) unregistered",
        registry.get_registered().len(),
        registry.get_unregistered().len()
    );

    // inter = interpreter_engine.for(Python).get_interpreter()
    // inter.release() / interpreter_engine.release(inter) / interpreter_engine.release(inter.get_id())
}
