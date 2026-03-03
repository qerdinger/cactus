use cactus_foundation::std::version::Version;
use regex::Regex;

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