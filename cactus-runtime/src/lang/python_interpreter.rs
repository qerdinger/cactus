use pyo3::Python;
use pyo3::types::{PyAnyMethods, PyModule};
use tracing::info;
use tracing_subscriber::fmt::format;

use crate::fragment::fragment::Fragment;
use crate::function::argument::Argument;
use crate::function::function::Function;
use crate::lang::lang_interpreter::LangInterpreter;

use std::path::PathBuf;
use std::ffi::CString;

pub struct PythonInterpreter;

impl LangInterpreter for PythonInterpreter {
    fn execute(fragments: &[Fragment], function: &Function, args: &[Argument]) {
    }

    fn is_entrypoint(fragments: &[Fragment], function: &Function) -> bool {
        Python::with_gil(|py| {

            // dependencies
            let sys = py.import("sys").expect("msg");
            let path = sys.getattr("path").expect("msg");

            info!("Integrating cactuskit...");
            let exe_dir = std::env::current_exe().expect("msg")
                .parent()
                .unwrap()
                .to_path_buf();

            let cactuskit_dir = exe_dir.join("../../../cactuskit/python3").canonicalize().expect("msg");
            info!("Seeking cactuskit python3 directory : [{}]", cactuskit_dir.to_str().unwrap());
            path.call_method1(
                "insert",
                (0, cactuskit_dir.to_str().unwrap(),),
            ).expect("msg");
            info!("Integrated cactuskit: OK!");

            let codebase: String = fragments.iter().map(|f| f.raw_data()).collect::<String>();

            info!("Executing codebase {}", codebase);

            let code = CString::new(codebase)
                .expect("msg");
            let filename = CString::new("mymod.py")
                .expect("msg");
            let module_name = CString::new("mymod")
                .expect("msg");

            let custom_module = PyModule::from_code(
                py,
                code.as_c_str(),
                filename.as_c_str(),
                module_name.as_c_str(),
            )
                .expect("msg");

            info!("fetching [{}] handler...", function.name());
            let handler = custom_module.getattr(function.name()).expect("msg");

            info!("gathering [is_initialised] method...");
            let is_init_method = match handler.getattr("is_initialised") {
                Ok(m) => m,
                Err(_) => return false,
            };

            if !is_init_method.is_callable() {
                info!("method is_initialised not callable");
                return false;
            }

            info!("executing is_initialised method...");
            let result = is_init_method.call0().expect("msg");
            result.extract::<bool>().expect("msg")
        })
    }
}