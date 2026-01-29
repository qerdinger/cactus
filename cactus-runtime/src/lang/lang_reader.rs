use crate::discovery::lang::Lang;
use crate::fragment::fragment::Fragment;
use crate::function::function::Function;

pub trait LangReader {
    fn new() -> Self
    where
        Self: Sized;
    fn lang(&self) -> &Lang;
    fn extract(&self, fragment: &Fragment) -> Vec<Function>;
}
