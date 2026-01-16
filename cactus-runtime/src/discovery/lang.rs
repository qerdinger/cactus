#[derive(Debug)]
pub enum Language {
    C,
    Cpp,
    Python,
    Java,
}

#[derive(Debug)]
pub struct Lang {
    lang: Language,
}

impl Lang {
    pub fn new(lang: Language) -> Lang {
        Self { lang }
    }

    pub fn lang(&self) -> &Language {
        &self.lang
    }
}
