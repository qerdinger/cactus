use crate::function::Function;
use std::ops::Deref;

#[derive(Debug)]
pub struct Cactuize {
    inner: Function,
}

impl Cactuize {
    pub fn new(inner: Function) -> Self {
        Self { inner }
    }
}

// Usage via Cactuize
impl Deref for Cactuize {
    type Target = Function;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}