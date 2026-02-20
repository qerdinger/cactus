use serde_json::{json, Value};

const STATUS_ID: &str = "_status_code";
const PAYLOAD_ID: &str = "_payload";
const TIMESTAMP_ID: &str = "_timestamp";


#[derive(Debug)]
pub enum CactusValue {
    None,
    Json(Value),
}

#[derive(Debug)]
pub struct CactusResponse {
    pub status: u16,
    pub payload: CactusValue,
    pub timestamp: Option<f64>,
}

impl CactusResponse {
    pub fn ok(status: u16, timestamp: f64, payload: Value) -> Self {
        Self { status, payload: CactusValue::Json(payload), timestamp: Some(timestamp) }
    }

    pub fn error(error: String) -> Self {
        Self { status: 500, payload: CactusValue::Json(json!({ "error": error })), timestamp: None }
    }
}

impl Into<CactusResponse> for Value {
    fn into(self) -> CactusResponse {
        match (self.get(STATUS_ID), self.get(TIMESTAMP_ID), self.get(PAYLOAD_ID)) {
            (Some(st), Some(ts), Some(pl)) => {
                match (st.as_u64(), ts.as_f64()) {
                    (Some(st_u64), Some(ts_f64)) => {
                        return CactusResponse::ok(st_u64 as u16, ts_f64, pl.to_owned())
                    },
                    _ => {}
                }
            },
            _ => {},
        };
        CactusResponse::error("error whilst casting into CactusResponse".to_string())
    }
}