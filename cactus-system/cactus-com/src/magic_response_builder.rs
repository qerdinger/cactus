use crate::protocols::layers::application::http_status::HttpStatus;

pub trait MagicResponseBuilder {
    fn new<S: Into<String>>(body: S) -> Self
    where
        Self: Sized;
    fn add_header_str(&mut self, header: &str) -> &mut dyn MagicResponseBuilder;
    fn set_status(&mut self, status: HttpStatus) -> &mut dyn MagicResponseBuilder;
    fn build(&self) -> Vec<u8>;
}

pub trait MagicResponseBuilderExt: MagicResponseBuilder {
    #[inline]
    fn add_header<S: Into<String>>(&mut self, header: S) -> &mut dyn MagicResponseBuilder {
        let header = header.into();
        self.add_header_str(&header)
    }
}

impl<T: MagicResponseBuilder + ?Sized> MagicResponseBuilderExt for T {}