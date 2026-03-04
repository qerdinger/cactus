use crate::protocol::Protocol;

#[derive(Debug)]
pub struct MagicRequest {
    protocol: Protocol,
    size: usize,
}

impl MagicRequest {
    pub fn new(buffer: &[u8], size: usize) -> Option<Self> {
        let text = std::str::from_utf8(buffer).ok()?;
        let protocol = Protocol::try_from(text).ok()?;
        Some(Self { protocol, size })
    }

    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }
    pub fn size(&self) -> usize { self.size }
}