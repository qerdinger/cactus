use cactus_foundation::fragment::Fragment;
use cactus_foundation::function::Function;
use cactus_foundation::lang::Lang;

pub trait LangReader {
    fn new() -> Self
    where
        Self: Sized;
    fn lang(&self) -> &Lang;
    fn extract(&self, fragment: &Fragment) -> Vec<Function>;
}
