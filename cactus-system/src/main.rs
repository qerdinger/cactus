use cactus_com::magic_request::MagicRequest;
use cactus_com::magic_response_builder::MagicResponseBuilderExt;
use cactus_com::protocol::Protocol;
use cactus_com::protocol_enum::ProtocolImpl;
use cactus_ingest::discover::Discover;
use cactus_interpreter::interpreter_engine::InterpreterEngine;
use cactus_interpreter::langs::python_interpreter::PythonInterpreter;
use cactus_lang::fragment_extractor::FragmentExtractor;
use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod registry;
use crate::registry::Registry;

fn tracing_subscriber_handler(max_level: Level) {
    let subscriber = FmtSubscriber::builder().with_max_level(max_level).finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                true => registry.register_parallel(fragments.clone(), fnc),
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

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    //

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                println!("{}b received", n);
                println!("{}", &buf[0..n].iter().map(|&b| b as char).collect::<String>());

                let request = MagicRequest::new(&buf, n);

                fn make_resp(_obj: &ProtocolImpl, data: &str) -> Vec<u8> {
                    //_obj.make_resp(|resp| {
                    //    resp.add_header()
                    //    resp.add_header()
                    //    resp.add_header()
                    //})

                    //_obj.make_resp(data)
                    //    .add_header()
                    //    .add_header()
                    //    .add_header()
                    //    .add_header()
                    //    .add_header()
                    //    .build()
                    info!("Here I am!");
                    _obj.make_resp(data)
                        .add_header("")
                        .build()

                }

                if let Some(req) = &request {
                    let protocol = req.protocol();
                    let data2 = make_resp(protocol, &format!("The request's been executed using {:?}", req));
                    println!("billable {}ms", req.time_elapsed());
                    if let Err(e) = socket.write_all(&data2).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}
