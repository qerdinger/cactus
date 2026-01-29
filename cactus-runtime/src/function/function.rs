use crate::discovery::lang::Lang;
use crate::function::argument::Argument;
use std::sync::Arc;

#[derive(Debug)]
pub struct Function {
    name: String,
    lang: Option<Arc<Lang>>,
    args: Vec<Argument>,
}

impl Function {
    pub fn new(name: String, lang: Option<Arc<Lang>>, args: Vec<Argument>) -> Function {
        Self { name, lang, args }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lang(&self) -> Option<&Lang> {
        self.lang.as_ref().map(|lang| lang as &_)
    }

    pub fn args(&self) -> &Vec<Argument> {
        &self.args
    }
}
