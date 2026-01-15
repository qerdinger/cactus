#[derive(Debug)]
pub struct Argument {
    name: String,
    primitive: Option<String>,
}

impl Argument {
    pub fn new(name: String, primitive: Option<String>) -> Self {
        Self { name, primitive }
    }
}