use cactus_foundation::fragment::Fragment;
use serde_json::Value as JsonValue;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};
use tracing::{error, info};
use serde_json::json;

struct WorkerProcess {
    stdin: Option<std::process::ChildStdin>,
    stdout: Option<BufReader<std::process::ChildStdout>>,
    child: Child,
}

struct Job {
    args: JsonValue,
    resp: oneshot::Sender<JsonValue>,
}

pub struct ParallelWorker {
    tx: mpsc::Sender<Job>,
}

impl ParallelWorker {
    pub fn new(
        fragments: Vec<Fragment>,
        function: String,
        num_workers: usize,
    ) -> Self {
        let (tx, rx) = mpsc::channel::<Job>(128);
        let rx = Arc::new(Mutex::new(rx));

        let fragments_code = fragments
            .iter()
            .map(|x| x.raw_data())
            .collect::<Vec<_>>()
            .join("");

        for worker_id in 0..num_workers {
            let rx = Arc::clone(&rx);
            let m_fragments_code = fragments_code.clone();
            let m_function = function.clone();

            std::thread::spawn(move || {
                let worker_code = format!(
                    r#"
import sys, json, time, os
from cactuskit import CactusEncoder

{}

func = globals()['{}']

while True:
    line = sys.stdin.readline()
    if not line:
        break
    request = json.loads(line)
    try:
        result = func()
        print(json.dumps(result, cls=CactusEncoder))
        sys.stdout.flush()
    except Exception as e:
        import traceback
        response = {{"status": 500, "payload": traceback.format_exc(), "timestamp": time.time()}}
        print(json.dumps(response))
        sys.stdout.flush()
"#,
                    m_fragments_code, m_function,
                );

                let exe_dir = std::env::current_exe()
                    .expect("exe")
                    .parent()
                    .expect("exe parent")
                    .to_path_buf();

                let cactuskit_dir = exe_dir
                    .join("../../../cactuskit/python3")
                    .canonicalize()
                    .expect("cactuskit path");

                let cactuskit_path = cactuskit_dir.to_str().expect("utf-8 path").to_string();

                let mut child = Command::new("python3")
                    .env("PYTHONPATH", cactuskit_path)
                    .arg("-c")
                    .arg(worker_code)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("failed to spawn worker process");

                let mut stdin = child.stdin.take().expect("piped stdin");
                let mut stdout = BufReader::new(child.stdout.take().expect("piped stdout"));

                info!("ParallelWorker {} initialized (pid: {:?})", worker_id, child.id());

                let mut line_buffer = String::new();

                loop {
                    let job = {
                        let mut guard = rx.blocking_lock();
                        guard.blocking_recv()
                    };

                    let Some(job) = job else { break };

                    let request = json!({ "args": job.args });
                    if writeln!(stdin, "{}", request.to_string()).is_err() {
                        break;
                    }
                    stdin.flush().unwrap();

                    line_buffer.clear();
                    if stdout.read_line(&mut line_buffer).is_err() {
                        break;
                    }

                    let response: JsonValue =
                        serde_json::from_str(&line_buffer).unwrap_or_else(|e| {
                            error!("{:?}", e);
                            JsonValue::Object(serde_json::json!({
                                "status": 500,
                                "payload": "Failed to parse response",
                                "timestamp": 0.0
                            }).as_object().unwrap().clone())
                        });

                    let _ = job.resp.send(response);
                }
            });
        }

        Self { tx }
    }

    pub async fn invoke(&self, args: JsonValue) -> JsonValue {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Job { args, resp: tx }).await.unwrap();
        rx.await.unwrap()
    }
}
