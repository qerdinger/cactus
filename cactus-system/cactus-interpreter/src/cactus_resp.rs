use serde_json::Value;

pub struct CactusResponse {
    pub status: usize,
    pub payload: Value,
    pub timestamp: f64,
}