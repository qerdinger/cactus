use std::fs;

use crate::fragment::fragment::Fragment;

pub struct Discover();

impl Discover {
    pub fn lookup(&self) -> Vec<Fragment> {
        let file_content = fs::read_to_string("../serverless.py");

        match file_content {
            Ok(content) => {
                vec![
                    Fragment::new("serverless.py".to_owned(), content)
                ]
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }
}