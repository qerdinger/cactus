use crate::magic_response_builder::MagicResponseBuilder;
use crate::protocols::http_status::HttpStatus;

#[derive(Debug)]
pub struct HTTPMagicResponseBuilder {
    headers: Vec<String>,
    status: HttpStatus,
    body: String,
}

impl MagicResponseBuilder for HTTPMagicResponseBuilder {
    fn new<S: Into<String>>(body: S) -> Self {
        Self {
            status: HttpStatus::Ok,
            headers: Vec::new(),
            body: body.into(),
        }
    }

    #[inline]
    fn add_header_str(&mut self, header: &str) -> &mut dyn MagicResponseBuilder {
        self.headers.push(header.to_string());
        self
    }

    #[inline]
    fn set_status(&mut self, status: HttpStatus) -> &mut dyn MagicResponseBuilder {
        self.status = status;
        self
    }

    fn build(&self) -> Vec<u8> {
        format!("{}\r\n\r\n{}", self.headers.join("\r\n"), self.body)
            .as_bytes()
            .to_vec()
    }
}