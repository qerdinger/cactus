use std::fmt;
use strum_macros::Display;

#[derive(Debug, Display, PartialEq)]
pub enum Language {
    #[strum(serialize = "C")]
    C,
    #[strum(serialize = "C++")]
    Cpp,
    #[strum(serialize = "Python")]
    Python,
    #[strum(serialize = "Java")]
    Java,
}

#[derive(Debug, PartialEq)]
pub struct Lang {
    lang: Language,
    version: Option<String>,
}

impl Lang {
    pub fn new(lang: Language) -> Lang {
        Self {
            lang,
            version: None,
        }
    }

    pub fn language(&self) -> &Language {
        &self.lang
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name={}, version={:?}", self.lang, self.version())
    }
}
