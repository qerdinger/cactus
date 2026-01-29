use crate::argument::Argument;
use crate::lang::Lang;
use std::sync::Arc;

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
