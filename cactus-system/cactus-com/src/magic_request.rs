use crate::protocol_enum::ProtocolImpl;
use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct MagicRequest {
    protocol: ProtocolImpl,

    size: usize,
    _timestamp: i64,
    _request_id: String,
}

impl MagicRequest {
    pub fn new(buffer: &[u8], size: usize) -> Option<Self> {
        let text = std::str::from_utf8(buffer).ok()?;
        let protocol = ProtocolImpl::parse(text).ok()?;
        Some(Self {
            protocol,

            size,
            _timestamp: Utc::now().timestamp_millis(),
            _request_id: Uuid::new_v4().to_string(),
        })
    }

    pub fn protocol(&self) -> &ProtocolImpl {
        &self.protocol
    }

    pub fn timestamp(&self) -> i64 { self._timestamp }

    pub fn time_elapsed(&self) -> i64 {
        Utc::now().timestamp_millis() - self._timestamp
    }

    pub fn time_elapsed_compared_to(&self, time_compared_with: i64) -> i64 {
        time_compared_with - self._timestamp
    }

    pub fn request_id(&self) -> &str { &self._request_id }

    pub fn size(&self) -> usize { self.size }
}