use crate::magic_response_builder::MagicResponseBuilder;
use crate::protocol::Protocol;
use crate::protocol_enum::ProtocolType;
use cactus_foundation::std::version::Version;
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct HttpsProtocImpl {
    version: Option<Version>,
}

impl TryFrom<&str> for HttpsProtocImpl {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains("HTTPS/") {
            // RFC 2145
            let re = Regex::new(r"HTTPS/(\d+)\.(\d+)")?;
            let version = re.captures(value).and_then(|caps| {
                let major = caps.get(1)?.as_str().parse::<u8>().ok()?;
                let minor = caps.get(2)?.as_str().parse::<u8>().ok()?;
                Some(Version::new(major, minor, None))
            });

            Ok(HttpsProtocImpl { version })
        } else {
            anyhow::bail!("not http")
        }
    }
}

impl Protocol for HttpsProtocImpl {
    fn protocol(&self) -> ProtocolType {
        ProtocolType::Https
    }

    fn make_resp(&self, body: &str, status_code: u16) -> Box<dyn MagicResponseBuilder> {
        todo!()
    }

    fn path(&self) -> &PathBuf {
        todo!()
    }

    fn query_strings(&self) -> &HashMap<String, String> {
        todo!()
    }
}