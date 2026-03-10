use crate::magic_response_builder::MagicResponseBuilder;
use crate::protocols::http_status::HttpStatus;

#[derive(Debug)]
pub struct HTTPMagicResponseBuilder {
    headers: Vec<String>,
    status: HttpStatus,
    body: Option<String>,
}

impl MagicResponseBuilder for HTTPMagicResponseBuilder {
    fn new() -> Self {
        Self {
            status: HttpStatus::Ok,
            headers: Vec::new(),
            body: None,
        }
    }

    fn add_header_str(&mut self, header: &str) -> &mut dyn MagicResponseBuilder {
        self.headers.push(header.to_string());
        self
    }

    fn assign_body_str(&mut self, body: &str) -> &mut dyn MagicResponseBuilder {
        self.body = Some(body.to_string());
        self
    }

    fn set_status(&mut self, status: HttpStatus) -> &mut dyn MagicResponseBuilder {
        self.status = status;
        self
    }

    fn build(&self) -> Vec<u8> {
        println!("MagicResponseBuilder::build()");
        println!("{:#?}", self);
        let _headers_str = self.headers.join("\r\n");
        let _body = self.body.clone().unwrap_or_else(|| "".to_owned());
        Vec::new()
    }
}