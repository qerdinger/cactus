use std::marker::PhantomData;

pub struct MagicProtocol<T> {
    _marker: PhantomData<T>,
}

impl<T> MagicProtocol<T> {
    pub fn new(buffer : T) -> Self {
        MagicProtocol { _marker : PhantomData }
    }
}