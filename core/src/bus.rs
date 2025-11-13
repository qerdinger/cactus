use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use plugin_api::{AppBusTx, AppBusRx};

#[derive(Clone, Default)]
pub struct BusTx {
    inner: Arc<Mutex<VecDeque<(String, Vec<u8>)>>>,
}
#[derive(Default)]
pub struct BusRx {
    inner: Arc<Mutex<VecDeque<(String, Vec<u8>)>>>,
}
pub fn channel() -> (BusTx, BusRx) {
    let q = Arc::new(Mutex::new(VecDeque::new()));
    (BusTx { inner: q.clone() }, BusRx { inner: q })
}
impl AppBusTx for BusTx {
    fn publish(&self, topic: &str, payload: &[u8]) {
        if let Ok(mut q) = self.inner.lock() {
            q.push_back((topic.to_string(), payload.to_vec()));
        }
    }
}
impl AppBusRx for BusRx {
    fn try_recv(&mut self) -> Option<(String, Vec<u8>)> {
        self.inner.lock().ok()?.pop_front()
    }
}
