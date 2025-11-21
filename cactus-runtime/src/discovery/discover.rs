use crate::discovery::lang::{Lang, Language};
use crate::functions::function::Function;

pub struct Discover ();

impl Discover {
    pub fn lookup(&self) -> Vec<Function> {
        vec![
            Function {
                name: "main".to_owned(),
                lang: Some(Lang { lang: Language::C }),
                args: vec![]
            },
        ]
    }
}