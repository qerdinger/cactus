use cactus_foundation::std::version::Version;
use regex::Regex;

#[derive(Debug)]
pub struct HTTPSProtocol {
    version: Option<Version>,
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