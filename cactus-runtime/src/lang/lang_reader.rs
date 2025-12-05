use crate::fragment::fragment::Fragment;
use crate::function::function::Function;

pub trait LangReader {
    fn extract(&self, fragment: &Fragment) -> Vec<Function>;
}
