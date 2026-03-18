use crate::protocols::layers::application::http::HttpProtocImpl;
use crate::protocols::layers::application::https::HttpsProtocImpl;
use crate::protocols::layers::application::ws::WSProtocolImpl;
use enum_dispatch::enum_dispatch;

#[derive(Debug)]
pub enum ProtocolType {
    Http,
    Https,
    WebSocket,
}

#[derive(Debug)]
#[enum_dispatch(Protocol)]
pub enum ProtocolImpl {
    Http(HttpProtocImpl),
    Https(HttpsProtocImpl),
    WebSocket(WSProtocolImpl),
}

impl ProtocolImpl {
    pub fn parse(value: &str) -> anyhow::Result<Self> {
        HttpProtocImpl::try_from(value)
            .map(ProtocolImpl::Http)
            .or_else(|_| HttpsProtocImpl::try_from(value).map(ProtocolImpl::Https))
            .or_else(|_| WSProtocolImpl::try_from(value).map(ProtocolImpl::WebSocket))
            .or_else(|_| anyhow::bail!("ProtocolImpl::try_from({}) not supported", value))
    }
}