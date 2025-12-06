use std::fs::write;
use crate::discovery::lang::{Lang, Language};
use crate::fragment::fragment::Fragment;
use crate::function::argument::Argument;
use crate::function::function::Function;
use crate::lang::lang_reader::LangReader;

const FUNCDECL_LENGTH: usize = 3;
const SPACE_LENGTH: usize = 1;
const ARGUMENT_SEPARATOR: char = ',';

pub struct PythonReader;

impl PythonReader {
    fn convert_idx_to_function_name(s :&str, s_len: usize, s_at: usize) -> Option<String> {
        if s_at + FUNCDECL_LENGTH + SPACE_LENGTH >= s_len {
            return None;
        }

        for (i, c) in s[s_at + FUNCDECL_LENGTH + SPACE_LENGTH..].chars().enumerate() {
            if (c == '(') {
                return Some(
                    String::from(
                        &s[s_at + FUNCDECL_LENGTH + SPACE_LENGTH..s_at + FUNCDECL_LENGTH + SPACE_LENGTH + i]
                    )
                );
            }
        }
        None
    }

    fn convert_idx_to_arguments(s : &str, s_len: usize, s_at: usize) -> Option<Argument> {
        let mut l_parenthesis: Option<usize> = None;
        let mut r_parenthesis: Option<usize> = None;

        for (i, c) in s[s_at..].chars().enumerate() {
            match c {
                '(' => l_parenthesis = Some(s_at + i),
                ')' => {r_parenthesis = Some(s_at + i); break;},
                _ => {}
            }
        }
        if l_parenthesis.is_none() && r_parenthesis.is_none() {
            return None;
        }

        for (i, c) in s[l_parenthesis.unwrap() + SPACE_LENGTH .. r_parenthesis.unwrap()]
            .split(ARGUMENT_SEPARATOR)
            .map(|x| x.replace(" ", ""))
            .enumerate() {
            println!("{}", c);
        }
        None
    }
}

impl LangReader for PythonReader {
    fn extract(&self, fragment: &Fragment) -> Vec<Function> {
        let fnc_indexes: Vec<usize> = fragment
            .raw_data()
            .match_indices("def")
            .map(|(i, _)|i)
            .collect();

        println!("{:?}", fnc_indexes);
        let content_size: usize = fragment.raw_data().chars().count();

        let functions: Vec<_> = fnc_indexes.iter().map(|x| {
            (
                Self::convert_idx_to_function_name(fragment.raw_data(), content_size, *x),
                Self::convert_idx_to_arguments(fragment.raw_data(), content_size, *x)
            )
        }).collect();

        eprintln!("functions: {:?}", functions);

        vec![
            Function::new("entrypoint".to_owned(), Some(Lang::new(Language::Python)), vec![
                Argument::new("input1".to_owned(), None),
            ])
        ]
    }
}