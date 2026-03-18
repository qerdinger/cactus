use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use cactus_com::client::Client;
use cactus_com::magic_request::MagicRequest;
use serde_json::Value as JsonValue;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use cactus_com::protocol::Protocol;
use crate::registry::Registry;

pub async fn handle_conn(mut client: Client<TcpStream, SocketAddr>, registry: &Arc<Registry>) -> Result<(), anyhow::Error> {
    let mut buffer = [0u8; 4096];

    loop {
        let n = client.read(&mut buffer).await?;

        if n == 0 {
            return Ok(());
        }

        let request = MagicRequest::new(&buffer, n);

        if let Ok(req) = &request {
            let protoc_impl = req.protocol();

            if let Some(pool) = registry.get_worker_pool("simple_entrypoint_delayed") {
                let rslt = pool.invoke(JsonValue::Null);
                let rslt_value = rslt.await;

                let protocol = protoc_impl as &dyn Protocol;
                let data = protocol
                    .make_resp(&format!("The request's executed using {:?}", rslt_value.payload))
                    .build();

                if let Err(e) = client.write_all(&data).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    anyhow::bail!("failed to write to socket; err = {:?}", e);
                }
            } else {
                println!("no parallel worker for executing: simple_entrypoint_delayed");
            }
        }
    }
}