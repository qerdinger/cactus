use crate::registry::Registry;
use cactus_com::client::Client;
use cactus_com::magic_request::MagicRequest;
use cactus_com::protocol::Protocol;
use serde_json::Value as JsonValue;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn handle_conn(mut client: Client<TcpStream, SocketAddr>, registry: &Arc<Registry>) -> Result<(), anyhow::Error> {
    let mut buffer = [0u8; 4096];

    loop {
        let n = client.read(&mut buffer).await?;

        if n == 0 {
            return Ok(());
        }

        println!("{}", String::from_utf8_lossy(&buffer[..n]));

        let request = MagicRequest::new(&buffer, n);

        if let Ok(req) = &request {
            let protoc_impl = req.protocol();

            let path_requested = (protoc_impl as &dyn Protocol).path();

            if let Some(pool) = registry.get_worker_pool(&path_requested[1..]) {
                let rslt = pool.invoke(JsonValue::Null);
                let rslt_value = rslt.await;
                let protocol = protoc_impl as &dyn Protocol;

                let data = protocol
                    .make_resp(&format!("{:?}\n*** Execution: ***\n{:?}\n*** Time elapsed : {}ms ***", req, rslt_value.payload, req.time_elapsed()))
                    .build();

                if let Err(e) = client.write_all(&data).await {
                    anyhow::bail!("failed to write to socket; err = {:?}", e);
                }
            } else {
                let protocol = protoc_impl as &dyn Protocol;
                let data = protocol
                    .make_resp(&format!("Lambda/function [requested={}], not found in registry!", &path_requested[1..]))
                    .build();
                if let Err(e) = client.write_all(&data).await {
                    anyhow::bail!("failed to write to socket; err = {:?}", e);
                }
            }
        }
    }
}