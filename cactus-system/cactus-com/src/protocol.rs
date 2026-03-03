use std::fmt::Display;
use regex::Regex;

#[derive(Debug)]
pub enum Protocol {
    Http(HTTPProtocol),
    Https(HTTPSProtocol),
    WebSocket(WSProtocol),
    None
}

#[derive(Debug)]
pub struct Version {
    major: u8,
    minor: u8,
}

impl Version {
    fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[derive(Debug)]
pub struct HTTPProtocol {
    version: Option<Version>,
}

impl HTTPProtocol {
    pub fn version(&self) -> Option<&Version> {
        if let Some(version) = &self.version {
            Some(version)
        } else { None }
    }
}

#[derive(Debug)]
pub struct HTTPSProtocol {
    version: Option<Version>,
}

#[derive(Debug)]
pub struct WSProtocol;


impl TryFrom<&str> for HTTPProtocol {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains("HTTP/") {
            // RFC 2145
            let re = Regex::new(r"HTTP/(\d+)\.(\d+)")?;
            let version = re.captures(value).and_then(|caps| {
                let major = caps.get(1)?.as_str().parse::<u8>().ok()?;
                let minor = caps.get(2)?.as_str().parse::<u8>().ok()?;
                Some(Version::new(major, minor))
            });

            Ok(HTTPProtocol { version })
        } else {
            anyhow::bail!("not http")
        }
    }
}

impl TryFrom<&str> for HTTPSProtocol {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains("HTTPS/") {
            // RFC 2145
            let re = Regex::new(r"HTTPS/(\d+)\.(\d+)")?;
            let version = re.captures(value).and_then(|caps| {
                let major = caps.get(1)?.as_str().parse::<u8>().ok()?;
                let minor = caps.get(2)?.as_str().parse::<u8>().ok()?;
                Some(Version::new(major, minor))
            });

            Ok(HTTPSProtocol { version })
        } else {
            anyhow::bail!("not http")
        }
    }
}

impl TryFrom<&str> for WSProtocol {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("http://") {
            Ok(WSProtocol {})
        } else {
            anyhow::bail!("not http")
        }
    }
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