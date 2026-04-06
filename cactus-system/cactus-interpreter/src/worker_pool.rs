use crate::cactus_resp::CactusResponse;
use crate::langs::python_worker::PythonWorker;
use cactus_foundation::cactuize::Cactuize;
use cactus_foundation::fragment::Fragment;
use pyo3::Python;
use std::sync::Arc;
use tokio::sync::oneshot;
use tracing::{error, info};

struct Job {
    args: serde_json::Value,
    resp: oneshot::Sender<CactusResponse>,
}

pub struct WorkerPool {
    tx: crossbeam_channel::Sender<Job>,
}

impl WorkerPool {
    pub fn new(
        fragments: Vec<Fragment>,
        function_name: &str,
        size: usize,
    ) -> Self {
        let (tx, rx) = crossbeam_channel::bounded::<Job>(128);

        for _ in 0..size {
            let rx = rx.clone();
            let fragments = fragments.clone();
            let function_name = function_name.to_string();

            std::thread::spawn(move || {
                let thread_id = std::thread::current().id();
                let worker = Python::with_gil(|py| PythonWorker::new(py, &fragments, &function_name));
                info!("Worker initialized for {} on {:?}", function_name, thread_id);

                for job in rx {
                    info!("Worker start {} on thread {:?}", function_name, thread_id);
                    let res = Python::with_gil(|py| worker.invoke(py, job.args));
                    info!("Worker end {} on thread {:?}", function_name, thread_id);
                    let _ = job.resp.send(res);
                }

                info!("Worker shut down for {} on thread {:?}", function_name, thread_id);
            });
        }

        Self { tx }
    }

    pub async fn invoke(&self, args: serde_json::Value) -> CactusResponse {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Job { args, resp: tx }).ok();

        rx.await.unwrap_or_else(|e| {
            error!("Worker failed to receive result: {}", e);
            CactusResponse::error(e.to_string())
        })
    }
}
