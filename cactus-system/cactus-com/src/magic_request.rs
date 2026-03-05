use crate::protocol_enum::ProtocolImpl;

#[derive(Debug)]
pub struct MagicRequest {
    protocol: ProtocolImpl,
    size: usize,
}

impl MagicRequest {
    pub fn new(buffer: &[u8], size: usize) -> Option<Self> {
        let text = std::str::from_utf8(buffer).ok()?;
        let protocol = ProtocolImpl::parse(text).ok()?;
        Some(Self { protocol, size })
    }

    pub fn protocol(&self) -> &ProtocolImpl {
        &self.protocol
    }
    pub fn size(&self) -> usize { self.size }
}