use enum_dispatch::enum_dispatch;

#[allow(unused_imports)]
use crate::protocol_enum::{ProtocolImpl, ProtocolType};

/*
Protocol implementation
Need to here for enum_dispatch macro

List all protocol implementations below, for being integrated into enum_dispatch
Also see : protocol_enum.rs
 */
#[allow(unused_imports)]
use crate::protocols::http::HttpProtocImpl;
#[allow(unused_imports)]
use crate::protocols::https::HttpsProtocImpl;
#[allow(unused_imports)]
use crate::protocols::ws::WSProtocolImpl;

#[enum_dispatch]
pub trait Protocol {
    fn protocol(&self) -> ProtocolType;
    fn make_resp(&self, body: &str) -> Vec<u8>;
}