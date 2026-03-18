use std::ops::{Deref, DerefMut};

pub struct Client<S, M> {
    stream: S,
    metadata: M,
}

impl<S, M> Client<S, M> {
    pub fn new(stream: S, metadata: M) -> Self {
        Self { stream, metadata }
    }

    pub fn metadata(&self) -> &M {
        &self.metadata
    }
}

impl<S, M> Deref for Client<S, M> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}

impl<S, M> DerefMut for Client<S, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stream
    }
}
