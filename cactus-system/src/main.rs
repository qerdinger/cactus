use cactus_com::magic_request::MagicRequest;
use cactus_com::protocol::Protocol;
use cactus_foundation::cactuize::Cactuize;
use cactus_ingest::discover::Discover;
use cactus_interpreter::interpreter_engine::InterpreterEngine;
use cactus_interpreter::langs::python_interpreter::PythonInterpreter;
use cactus_lang::fragment_extractor::FragmentExtractor;
use socket2::{Domain, Protocol as SocketProtocol, Socket, Type};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Semaphore;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod registry;
use crate::registry::Registry;

fn tracing_subscriber_handler(max_level: Level) {
    let subscriber = FmtSubscriber::builder().with_max_level(max_level).finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

fn build_listener(addr: &str, backlog: i32) -> Result<TcpListener, Box<dyn std::error::Error>> {
    let socket_addr: SocketAddr = addr.parse()?;
    let domain = if socket_addr.is_ipv4() {
        Domain::IPV4
    } else {
        Domain::IPV6
    };

    let socket = Socket::new(domain, Type::STREAM, Some(SocketProtocol::TCP))?;
    socket.set_reuse_address(true)?;
    socket.bind(&socket_addr.into())?;
    socket.listen(backlog)?;

    let std_listener: std::net::TcpListener = socket.into();
    std_listener.set_nonblocking(true)?;

    Ok(TcpListener::from_std(std_listener)?)
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

    let listener = build_listener(LISTEN_ADDR, BACKLOG)?;
    let connection_limiter = Arc::new(Semaphore::new(MAX_CONCURRENT_CONNECTIONS));

    loop {
        let permit = connection_limiter.clone().acquire_owned().await?;

        let (mut socket, _) = match listener.accept().await {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("failed to accept connection; err = {:?}", e);
                drop(permit);
                continue;
            }
        };

        tokio::spawn(async move {
            let _permit = permit;
            let mut buf = [0; 4096];

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

                //println!("{}b received", n);
                //println!("{}", &buf[0..n].iter().map(|&b| b as char).collect::<String>());

                let request = MagicRequest::new(&buf, n);

                if let Some(req) = &request {
                    let protoc_impl = req.protocol();
                    let protocol = protoc_impl as &dyn Protocol;
                    let data = protocol
                        .make_resp(&format!("The request's been executed using {:?}", req))
                        //.add_header("Content-Language: fr-FR")
                        .build();

                    //println!("billable {}ms", req.time_elapsed());
                    if let Err(e) = socket.write_all(&data).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}
