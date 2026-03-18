use cactus_com::client::Client;
use cactus_com::magic_request::MagicRequest;
use cactus_com::protocol::Protocol;
use cactus_com::protocols::layers::transport::tcp::TcpListener;
use cactus_com::utils::rate_limiter::RateLimiter;
use cactus_foundation::cactuize::Cactuize;
use cactus_ingest::discover::Discover;
use cactus_interpreter::interpreter_engine::InterpreterEngine;
use cactus_interpreter::langs::python_interpreter::PythonInterpreter;
use cactus_lang::fragment_extractor::FragmentExtractor;
use log::error;
use serde_json::Value as JsonValue;
use socket2::{Domain, Protocol as SocketProtocol, Socket, Type};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod handler;
use crate::handler::handle_conn;
mod registry;
use crate::registry::Registry;

fn tracing_subscriber_handler(max_level: Level) {
    let subscriber = FmtSubscriber::builder().with_max_level(max_level).finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
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
            let Some(is_entrypoint) = interpreter_engine
                .with_interpreter_for_lang(f_lang, |interp| {
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
                true => registry.register_to_thread_pool(fragments.clone(), Cactuize::new(fnc)),
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

    let registry = Arc::new(registry);

    /*if let Some(pool) = registry.get_parallel_worker("simple_entrypoint_delayed") {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("error thrown whilst creating tokio runtime");

        let (rd, rd1, rd2, rd3, rd4) = runtime.block_on(async {
            tokio::join!(
            pool.invoke(JsonValue::Null),
            pool.invoke(JsonValue::Null),
            pool.invoke(JsonValue::Null),
            pool.invoke(JsonValue::Null),
            pool.invoke(JsonValue::Null),
        )
        });

        info!("rd: {:?}", rd);
        info!("rd: {:?}", rd1);
        info!("rd: {:?}", rd2);
        info!("rd: {:?}", rd3);
        info!("rd: {:?}", rd4);

        runtime.block_on(async {
            for _ in 0..3 {
                let rslt = pool.invoke(JsonValue::Null);

                info!("rslt: {:?}", rslt.await);
            }
        });
    }*/


    //let fnc = Function::new("test".to_string(), None, vec![]);
    //let cactuized = Cactuize::new(fnc);
    //println!("name={}", cactuized.name());

    const LISTEN_ADDR: &str = "127.0.0.1:8080";
    const BACKLOG: i32 = 2048;
    const MAX_CONCURRENT_CONNECTIONS: usize = 512;

    let listener = TcpListener::new(LISTEN_ADDR, BACKLOG)?;
    let connection_limiter = RateLimiter::new(MAX_CONCURRENT_CONNECTIONS);

    loop {
        let registry = Arc::clone(&registry);
        let permit = match connection_limiter.get_permit().await {
            Ok(p) => p,
            Err(_) => continue,
        };

        let client = match listener.accept().await {
            Ok((conn, sock_addr)) => Client::new(conn, sock_addr),
            Err(e) => {
                eprintln!("failed to accept connection; err = {:?}", e);
                drop(permit);
                continue;
            }
        };

        //let permit = match connection_limiter.clone().acquire_owned().await {
        //    Ok(p) => p,
        //    Err(_) => continue,
        //};

        tokio::spawn(async move {
            handle_conn(client, &registry).await.unwrap();
            permit.forget();
        });
    }
}
