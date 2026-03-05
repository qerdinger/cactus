use crate::protocol::Protocol;
use crate::protocol_enum::ProtocolType;
use crate::protocols::http::HttpProtocImpl;

#[derive(Debug)]
pub struct WSProtocolImpl;

impl TryFrom<&str> for WSProtocolImpl {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("http://") {
            Ok(WSProtocolImpl {})
        } else {
            anyhow::bail!("not http")
        }
    }
}

impl Protocol for WSProtocolImpl {
    fn protocol(&self) -> ProtocolType {
        ProtocolType::WebSocket
    }
}