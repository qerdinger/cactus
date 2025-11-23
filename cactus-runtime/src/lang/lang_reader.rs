use crate::function::function::Function;

pub trait LangReader {
    fn extract(&self) -> Vec<Function>;
}
