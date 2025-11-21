use crate::discovery::lang::{Lang, Language};
use crate::functions::function::Function;

pub struct Discover ();

impl Discover {
    pub fn lookup(&self) -> Vec<Function> {
        vec![
            Function::new(
                "main".to_owned(), Some(Lang::new(Language::C)), vec![]
            ),
            Function::new("name".to_owned(), None, vec![])
        ]
    }
}