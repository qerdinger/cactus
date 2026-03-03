use crate::protocol::Protocol;

#[derive(Debug)]
pub struct MagicRequest {
    protocol: Protocol,
}

impl MagicRequest {
    pub fn new(buffer: &[u8]) -> Option<Self> {
        let text = std::str::from_utf8(buffer).ok()?;
        let protocol = Protocol::try_from(text).ok()?;
        Some(Self { protocol })
    }

    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }
}