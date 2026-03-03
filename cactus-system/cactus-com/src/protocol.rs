#[derive(Debug)]
pub enum Protocol {
    Http(Http),
    Https(Https),
    WebSocket(WebSocket),
    None
}

#[derive(Debug)]
struct Http;

#[derive(Debug)]
struct Https;

#[derive(Debug)]
struct WebSocket;


impl TryFrom<&str> for Http {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("http://") {
            Ok(Http {})
        } else {
            anyhow::bail!("not http")
        }
    }
}

impl TryFrom<&str> for Https {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("http://") {
            Ok(Https {})
        } else {
            anyhow::bail!("not http")
        }
    }
}

impl TryFrom<&str> for WebSocket {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("http://") {
            Ok(WebSocket {})
        } else {
            anyhow::bail!("not http")
        }
    }
}

impl TryFrom<&str> for Protocol {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Http::try_from(value)
            .map(Protocol::Http)
            .or_else(|_| Https::try_from(value).map(Protocol::Https))
            .or_else(|_| WebSocket::try_from(value).map(Protocol::WebSocket))
            .or_else(|_| Ok(Protocol::None))
    }
}