use cactus_ingest::discover::Discover;
use cactus_interpreter::interpreter_engine::InterpreterEngine;
use cactus_interpreter::langs::python_interpreter::PythonInterpreter;
use cactus_lang::fragment_extractor::FragmentExtractor;
use serde_json::Value as JsonValue;
use std::env;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod registry;
use crate::registry::Registry;

fn tracing_subscriber_handler(max_level: Level) {
    let subscriber = FmtSubscriber::builder().with_max_level(max_level).finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

fn main() {
    let _args: Vec<String> = env::args().collect();
    tracing_subscriber_handler(Level::INFO);
    info!("Cactus Runtime System");

    let disc = Discover();
    let mut fragments = disc.lookup();

    let mut registry = Registry::new();

    info!("{} fragment(s) discovered", fragments.len());
    let mut all_functions = Vec::new();

    fragments.iter_mut().for_each(|frgmt| {
        FragmentExtractor::extract(frgmt);
        if let Some(fncs) = frgmt.functions_mut() {
            all_functions.extend(fncs.drain(..));
        }
    });


    let mut interpreter_engine = InterpreterEngine::new();

    interpreter_engine.register(PythonInterpreter::new());


    for fnc in all_functions {
        if let (f_name, Some(f_lang)) = (fnc.name(), fnc.lang()) {
            info!("Registering function: {}", f_name);
            let Some(is_entrypoint) = interpreter_engine
                .with_interpreter_for_lang(f_lang, |interp| {
                    info!("Executing function: {} {}", fnc.name(), fragments.len());
                    let is_entrypoint = interp.is_entrypoint(&fragments, &fnc);
                    info!("{} is_entrypoint={}", fnc.name(), is_entrypoint);

                    return is_entrypoint;
                })
            else {
                info!(
                    "{}: No interpreter available for lang (defined as [{:?}])",
                    f_name, f_lang
                );
                continue;
            };

            match is_entrypoint {
                true => registry.register_registered(fragments.clone(), fnc),
                _ => registry.register_unregistered(fnc),
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

    if let Some(pool) = registry.get_worker_pool("simple_entrypoint") {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");
        let resp = runtime.block_on(pool.invoke(JsonValue::Null));
        info!("Invocation response for simple_entrypoint: {:?}", resp);
    } else {}
}
