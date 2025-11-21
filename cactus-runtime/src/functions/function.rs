use crate::discovery::lang::Lang;
use crate::functions::argument::Argument;

pub struct Function {
    name: String,
    lang: Option<Lang>,
    args: Vec<Argument>,
}

impl Function {
    pub fn new(name: String, lang: Option<Lang>, args: Vec<Argument>) -> Function {
        Self { name, lang, args }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn lang(&self) -> &Option<Lang> {
        &self.lang
    }

    pub fn args(&self) -> &Vec<Argument> {
        &self.args
    }
}