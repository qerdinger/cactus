use std::fmt::Display;

#[derive(Debug)]
pub enum Language {
    C,
    Cpp,
    Python,
    Java
}

#[derive(Debug)]
pub struct Lang {
    pub lang: Language,
}