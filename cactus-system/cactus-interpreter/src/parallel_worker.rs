use crate::cactus_resp::CactusResponse;
use cactus_foundation::fragment::Fragment;
use serde_json::json;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use tokio::sync::oneshot;
use tracing::{error, info};

struct WorkerProcess {
    stdin: Option<std::process::ChildStdin>,
    stdout: Option<BufReader<std::process::ChildStdout>>,
    child: Child,
}

struct Job {
    args: HashMap<String, String>,
    resp: oneshot::Sender<CactusResponse>,
}

pub struct ParallelWorker {
    tx: crossbeam_channel::Sender<Job>,
}

impl ParallelWorker {
    pub fn new(
        fragments: Vec<Fragment>,
        function: String,
        num_workers: usize,
    ) -> Self {
        let (tx, rx) = crossbeam_channel::bounded::<Job>(128);

        info!("Creating {} parallel workers for function: {}", num_workers, function);

        let fragments_code = fragments
            .iter()
            .map(|x| x.raw_data())
            .collect::<Vec<_>>()
            .join("");

        for worker_id in 0..num_workers {
            let rx = rx.clone();
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
        call_args = request.get("args")
        if isinstance(call_args, dict):
            result = func(**call_args)
        elif isinstance(call_args, list):
            result = func(*call_args)
        elif call_args is None:
            result = func()
        else:
            result = func(call_args)
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

                let mut child = match Command::new("python3")
                    .env("PYTHONPATH", cactuskit_path)
                    .arg("-c")
                    .arg(worker_code)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    Ok(c) => {
                        info!("ParallelWorker {} spawned successfully", worker_id);
                        c
                    }
                    Err(e) => {
                        error!("ParallelWorker {} failed to spawn Python process: {}", worker_id, e);
                        return;
                    }
                };

                let mut stdin = child.stdin.take().expect("piped stdin");
                let mut stdout = BufReader::new(child.stdout.take().expect("piped stdout"));

                info!("ParallelWorker {} initialized (pid: {:?})", worker_id, child.id());

                for job in rx {
                    info!("ParallelWorker {} START processing job", worker_id);
                    let request = json!({ "args": job.args });
                    if writeln!(stdin, "{}", request.to_string()).is_err() {
                        break;
                    }
                    stdin.flush().unwrap();

                    let mut line_buffer = String::new();
                    if stdout.read_line(&mut line_buffer).is_err() {
                        break;
                    }

                    let response: CactusResponse =
                        serde_json::from_str(&line_buffer).unwrap_or_else(|e| {
                            error!("{:?}", e);
                            JsonValue::Object(serde_json::json!({
                                "status": 500,
                                "payload": "Failed to parse response",
                                "timestamp": 0.0
                            }).as_object().unwrap().clone())
                        }).into();

                    info!("ParallelWorker {} END processing job", worker_id);
                    let _ = job.resp.send(response);
                }

                info!("ParallelWorker {} shut down", worker_id);
            });
        }

        info!("Successfully spawned {} parallel workers for function: {}", num_workers, function);

        Self { tx }
    }

    pub async fn invoke(&self, args: HashMap<String, String>) -> CactusResponse {
        let (tx, rx) = oneshot::channel();
        self.tx.send(Job { args, resp: tx }).ok();
        rx.await.unwrap()
    }
}
