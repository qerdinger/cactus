use crate::langs::python_worker::PythonWorker;
use cactus_foundation::fragment::Fragment;
use pyo3::Python;
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, oneshot};
use tracing::{error, info};
use crate::cactus_resp::CactusResponse;

struct Job {
    args: serde_json::Value,
    resp: oneshot::Sender<CactusResponse>,
}

pub struct WorkerPool {
    tx: mpsc::Sender<Job>,
}

impl WorkerPool {
    pub fn new(
        fragments: Vec<Fragment>,
        function: String,
        size: usize,
    ) -> Self {
        let (tx, rx) = mpsc::channel::<Job>(128);
        let rx = Arc::new(Mutex::new(rx));

        for _ in 0..size {
            let rx = Arc::clone(&rx);
            let fragments = fragments.clone();
            let function = function.clone();

            std::thread::spawn(move || {
                let thread_id = std::thread::current().id();
                let worker = Python::with_gil(|py| PythonWorker::new(py, &fragments, &function));
                info!("Worker initialized for {} on {:?}", function, thread_id);

                loop {
                    let job = {
                        let mut guard = rx.lock().unwrap();
                        guard.blocking_recv()
                    };

                    let Some(job) = job else { break };
                    info!("Worker start {} on thread {:?}", function, thread_id);
                    let res = Python::with_gil(|py| worker.invoke(py, job.args));
                    info!("Worker end {} on thread {:?}", function, thread_id);
                    let _ = job.resp.send(res);
                }
            });
        }

        Self { tx }
    }

    pub async fn invoke(&self, args: serde_json::Value) -> CactusResponse {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Job { args, resp: tx }).await.unwrap();

        rx.await.unwrap_or_else(|e| {
            error!("Worker failed to receive result: {}", e);
            CactusResponse::error(e.to_string())
        })
    }
}
