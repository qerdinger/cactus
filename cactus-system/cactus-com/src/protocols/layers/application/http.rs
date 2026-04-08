use crate::magic_response_builder::{MagicResponseBuilder, MagicResponseBuilderExt};
use crate::protocol::Protocol;
use crate::protocol_enum::ProtocolType;
use crate::protocols::layers::application::http_authorization::HttpAuthorization;
use crate::protocols::layers::application::http_encoding::HttpEncoding;
use crate::protocols::layers::application::http_magic_resp_builder::HTTPMagicResponseBuilder;
use crate::protocols::layers::application::http_method::HttpMethod;
use crate::protocols::layers::application::http_status::HttpStatus;
use crate::protocols::layers::application::user_agent::UserAgent;
use anyhow::anyhow;
use cactus_foundation::std::version::Version;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::{error, info};
use url::Url;

const INTERNAL_URL: &str = "http://cactus-sys.runtime.internal";

#[derive(Debug)]
pub struct HttpProtocImpl {
    method: HttpMethod,
    version: Option<Version>,
    path: PathBuf,
    query_strings: HashMap<String, String>,

    auth: Option<HttpAuthorization>,
    user_agent: Option<UserAgent>,
    encoding: Option<HttpEncoding>,
}

impl HttpProtocImpl {
    pub fn new<S>(method: HttpMethod, version: Option<Version>, path_with_queries: S) -> Result<Self, anyhow::Error>
    where
        S: Into<String>,
    {
        let data_url = Url::parse(&format!("{INTERNAL_URL}{}", path_with_queries.into())).map_err(|e| anyhow!(e))?;
        Ok(
            Self {
                method,
                version,
                path: PathBuf::from_str(data_url.path()).map_err(|e| anyhow!(e))?,
                query_strings: data_url
                    .query_pairs()
                    .map(|(k, v)|
                        (k.to_string(), v.to_string())
                    )
                    .collect(),

                auth: None,
                user_agent: None,
                encoding: None,
            }
        )
    }

    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn version(&self) -> Option<&Version> {
        if let Some(version) = &self.version {
            Some(version)
        } else { None }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn query_strings(&self) -> &HashMap<String, String> {
        &self.query_strings
    }

    pub fn auth(&self) -> Option<&HttpAuthorization> {
        self.auth.as_ref()
    }

    pub fn user_agent(&self) -> Option<&UserAgent> {
        self.user_agent.as_ref()
    }

    pub fn encoding(&self) -> Option<&HttpEncoding> {
        self.encoding.as_ref()
    }
}

fn parse_authentification_attribute(value: &str, http_impl: &mut HttpProtocImpl) {
    http_impl.auth = HttpAuthorization::new(value);
}

fn parse_user_agent_attribute(value: &str, http_impl: &mut HttpProtocImpl) {
    if let Ok(agent) = UserAgent::from_request(value) {
        http_impl.user_agent = Some(agent);
    } else {
        error!("Could not parse user agent from request: {}", value);
    }
}

fn parse_encoding_attribute(value: &str, http_impl: &mut HttpProtocImpl) {
    if let Ok(encoding) = HttpEncoding::try_from(value) {
        http_impl.encoding = Some(encoding);
    } else {
        error!("Could not parse encoding from request: {}", value);
    }
}

fn parse_request_line(value: &str) -> Result<HttpProtocImpl, anyhow::Error> {
    let mut parts = value.split_whitespace();

    let method = parts.next().ok_or_else(|| anyhow!("Missing method"))?;

    let path = parts.next().ok_or_else(|| anyhow!("Missing path"))?;

    let version_part = parts.next().ok_or_else(|| anyhow!("Missing version"))?;

    let (_, version_str) = version_part.split_once('/').ok_or_else(|| anyhow!("Missing version"))?;

    let (major, minor) = match version_str.split_once('.') {
        Some((maj, min)) => (
            maj.parse::<u8>().map_err(|_| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid major version",
            ))?,
            min.parse::<u8>().map_err(|_| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid minor version",
            ))?,
        ),
        None => (
            version_str.parse::<u8>().map_err(|_| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid version",
            ))?,
            0,
        ),
    };

    let method = HttpMethod::from_str(method).map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid method")
    })?;

    Ok(HttpProtocImpl::new(
        method,
        Some(Version::new(major, minor, None)),
        path,
    )?)
}

impl TryFrom<&str> for HttpProtocImpl {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains("HTTP/") {
            let mut http_impl = parse_request_line(value)?; // RFC 2145
            for arg in value.lines() {
                match arg.split_once(":") {
                    Some(("Authorization", val)) => parse_authentification_attribute(val, &mut http_impl),
                    Some(("User-Agent", val)) => parse_user_agent_attribute(val, &mut http_impl),
                    Some(("Accept-Encoding", val)) => parse_encoding_attribute(val, &mut http_impl),
                    _ => {}
                }
            }
            Ok(http_impl)
        } else {
            anyhow::bail!("not http")
        }
    }
}

macro_rules! header_formatting {
    ($builder:expr, $protoc_version:expr, $status:expr, $status_def:expr, $body:expr) => {{
        $builder
            .add_header(format!("HTTP/{} {} {}", $protoc_version, $status, $status_def))
            .add_header(format!("Content-length: {}", $body.len()))
            .add_header("Content-type: text/plain; charset=UTF-8");
    }};
}

impl Protocol for HttpProtocImpl {
    fn protocol(&self) -> ProtocolType {
        ProtocolType::Http
    }

    fn path(&self) -> &PathBuf {
        self.path()
    }

    fn query_strings(&self) -> &HashMap<String, String> {
        &self.query_strings
    }

    /*fn make_resp(&self, body: &str) -> Vec<u8> {
        format!("HTTP/1.1 200 OK
Content-length: {}
Content-type: text/plain; charset=UTF-8

{}", body.len(), body).into_bytes()
    }*/

    fn make_resp(&self, body: &str, status_code: u16) -> Box<dyn MagicResponseBuilder> {
        let mut builder = HTTPMagicResponseBuilder::new(body);

        let status_code = HttpStatus::convert_status_to_u16_and_string(&status_code.into());
        header_formatting!(builder, "1.1", status_code.0, status_code.1, body);
        Box::new(builder)
    }
}