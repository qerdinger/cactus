use crate::protocols::http_status::HttpStatus;

pub trait MagicResponseBuilder {
    fn new() -> Self where Self: Sized;
    fn add_header_str(&mut self, header: &str) -> &mut dyn MagicResponseBuilder;
    fn assign_body_str(&mut self, body: &str) -> &mut dyn MagicResponseBuilder;
    fn set_status(&mut self, status: HttpStatus) -> &mut dyn MagicResponseBuilder;
    fn build(&self) -> Vec<u8>;
}

pub trait MagicResponseBuilderExt: MagicResponseBuilder {
    fn add_header<S: Into<String>>(&mut self, header: S) -> &mut dyn MagicResponseBuilder {
        let header = header.into();
        self.add_header_str(&header)
    }

    fn assign_body<S: Into<String>>(&mut self, body: S) -> &mut dyn MagicResponseBuilder {
        let body = body.into();
        self.assign_body_str(&body)
    }
}

impl<T: MagicResponseBuilder + ?Sized> MagicResponseBuilderExt for T {}