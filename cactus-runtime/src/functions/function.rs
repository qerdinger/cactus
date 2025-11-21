use crate::discovery::lang::Lang;
use crate::functions::argument::Argument;

pub struct Function {
    pub name: String,
    pub lang: Option<Lang>,
    pub args: Vec<Argument>
}