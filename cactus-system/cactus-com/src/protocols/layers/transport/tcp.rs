use anyhow::anyhow;
use socket2::{Domain, Protocol as SocketProtocol, Socket, Type};
use std::net::SocketAddr;
use std::ops::Deref;

pub struct TcpListener {
    listener: tokio::net::TcpListener,
}

impl TcpListener {
    pub fn new(listen_addr: &str, backlog_size: i32) -> Result<Self, anyhow::Error> {
        let listener = build_listener(listen_addr, backlog_size)
            .map_err(|e| anyhow!("Could not create TCP listener: {:?}", e))?;

        Ok(Self { listener })
    }
}

impl Deref for TcpListener {
    type Target = tokio::net::TcpListener;

    fn deref(&self) -> &Self::Target {
        &self.listener
    }
}

fn build_listener(addr: &str, backlog: i32) -> Result<tokio::net::TcpListener, Box<dyn std::error::Error>> {
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

    Ok(tokio::net::TcpListener::from_std(std_listener)?)
}