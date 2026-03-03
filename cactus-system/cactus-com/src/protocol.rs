use crate::protocols::http::HTTPProtocol;
use crate::protocols::https::HTTPSProtocol;
use crate::protocols::ws::WSProtocol;

#[derive(Debug)]
pub enum Protocol {
    Http(HTTPProtocol),
    Https(HTTPSProtocol),
    WebSocket(WSProtocol),
    None,
}

impl TryFrom<&str> for Protocol {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        HTTPProtocol::try_from(value)
            .map(Protocol::Http)
            .or_else(|_| HTTPSProtocol::try_from(value).map(Protocol::Https))
            .or_else(|_| WSProtocol::try_from(value).map(Protocol::WebSocket))
            .or_else(|_| Ok(Protocol::None))
    }
}