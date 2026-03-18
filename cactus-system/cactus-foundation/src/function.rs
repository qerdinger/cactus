use crate::argument::Argument;
use crate::lang::Lang;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Function {
    name: String,
    lang: Option<Arc<Lang>>,
    args: Vec<Argument>,
}

impl Function {
    pub fn new(name: String, lang: Option<Arc<Lang>>, args: Vec<Argument>) -> Self {
        Self { name, lang, args }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn lang(&self) -> Option<&Lang> {
        self.lang.as_deref()
    }

    pub fn args(&self) -> &[Argument] {
        &self.args
    }
}