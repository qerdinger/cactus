use std::collections::HashMap;
use crate::magic_response_builder::MagicResponseBuilder;
use crate::protocol::Protocol;
use crate::protocol_enum::ProtocolType;

#[derive(Debug)]
pub struct WSProtocolImpl;

impl TryFrom<&str> for WSProtocolImpl {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("http://") {
            Ok(WSProtocolImpl {})
        } else {
            anyhow::bail!("not http")
        }
    }
}

impl Protocol for WSProtocolImpl {
    fn protocol(&self) -> ProtocolType {
        ProtocolType::WebSocket
    }

    fn make_resp(&self, body: &str) -> Box<dyn MagicResponseBuilder> {
        todo!()
    }

    fn path(&self) -> &str {
        todo!()
    }

    fn query_strings(&self) -> &HashMap<String, String> {
        todo!()
    }
}