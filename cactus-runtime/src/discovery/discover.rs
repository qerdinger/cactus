use crate::discovery::lang::{Lang, Language};
use crate::function::function::Function;
use crate::fragment::fragment::Fragment;

pub struct Discover ();

impl Discover {
    pub fn lookup(&self) -> Vec<Fragment> {
        vec![
            Fragment::new("serverless.py".to_owned(),
                "def entrypoint():\
                    return 'Hello'".to_owned())
        ]
    }
}