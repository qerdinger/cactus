use crate::magic_response_builder::{MagicResponseBuilder, MagicResponseBuilderExt};
use crate::protocol::Protocol;
use crate::protocol_enum::ProtocolType;
use crate::protocols::http_magic_resp_builder::HTTPMagicResponseBuilder;
use crate::protocols::http_method::HttpMethod;
use cactus_foundation::std::version::Version;
use std::str::FromStr;

#[derive(Debug)]
pub struct HttpProtocImpl {
    method: HttpMethod,
    version: Option<Version>,
    path: String,
}

impl HttpProtocImpl {
    pub fn new<S: Into<String>>(method: HttpMethod, version: Option<Version>, path: S) -> Self {
        Self { method, version, path: path.into() }
    }

    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn version(&self) -> Option<&Version> {
        if let Some(version) = &self.version {
            Some(version)
        } else { None }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

fn parse_request_line(value: &str) -> Option<HttpProtocImpl> {
    let mut parts = value.split_whitespace();

    let method = parts.next()?;
    let path = parts.next()?;
    let version_part = parts.next()?;

    // "HTTP/1.1" or "HTTP/2"
    let (protocol_str, version_str) = version_part.split_once('/')?;

    let (major, minor) = match version_str.split_once('.') {
        Some((maj, min)) => (
            maj.parse::<u8>().ok()?,
            min.parse::<u8>().ok()?,
        ),
        None => (
            version_str.parse::<u8>().ok()?,
            0,
        ),
    };

    let method = match HttpMethod::from_str(method) {
        Ok(method) => method,
        Err(_) => return None,
    };

    //Some((method, path, protocol_str, Version::new(major, minor)))
    Some(HttpProtocImpl::new(method, Some(Version::new(major, minor)), path))
}

impl TryFrom<&str> for HttpProtocImpl {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains("HTTP/") {
            // RFC 2145
            if let Some(http_impl) = parse_request_line(value) {
                return Ok(http_impl);
            }

            anyhow::bail!("HTTP request line fails parsing")
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

    /*fn make_resp(&self, body: &str) -> Vec<u8> {
        format!("HTTP/1.1 200 OK
Content-length: {}
Content-type: text/plain; charset=UTF-8

{}", body.len(), body).into_bytes()
    }*/

    fn make_resp(&self, body: &str) -> Box<dyn MagicResponseBuilder> {
        let mut builder = HTTPMagicResponseBuilder::new(body);

        header_formatting!(builder, "1.1", 200, "OK", body);
        Box::new(builder)
    }
}