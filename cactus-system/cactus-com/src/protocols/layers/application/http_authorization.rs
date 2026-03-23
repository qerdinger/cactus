#[derive(Debug)]
pub enum HttpAuthorization {
    Basic(String),  // RFC 7617
    Bearer(String), // RFC 6750
}

impl HttpAuthorization {
    pub fn new(line: &str) -> Option<Self> {
        match line.trim().split_once(" ") {
            Some(("Basic", val)) => Some(HttpAuthorization::Basic(val.to_string())),
            Some(("Bearer", val)) => Some(HttpAuthorization::Bearer(val.to_string())),
            _ => None,
        }
    }
}