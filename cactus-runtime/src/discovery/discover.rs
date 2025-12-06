use crate::discovery::lang::{Lang, Language};
use crate::function::function::Function;
use crate::fragment::fragment::Fragment;

pub struct Discover ();

impl Discover {
    pub fn lookup(&self) -> Vec<Fragment> {
        vec![
            Fragment::new("serverless.py".to_owned(),
                "#Basic cactus entrypoint\
                def entrypoint(arg1, arg2):\
                    return 'Hello'".to_owned())
        ]
    }
}