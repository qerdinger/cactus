use std::ffi::CString;

use pyo3::types::PyAnyMethods;
use pyo3::types::{PyAny, PyModule};
use pyo3::Bound;
use pyo3::{Py, Python};

use crate::LangInterpreter;

use crate::discovery::lang::{Lang, Language};
use crate::{
    fragment::fragment::Fragment,
    function::{argument::Argument, function::Function},
};

pub struct PythonInterpreter {
    lang: Lang,
    sys: Py<PyModule>,
    path: Py<PyAny>,
    cactuskit_path: String,
}

const REGISTERED_PROPERTY_ID: &str = "_is_declared";

const PYTHON_MOD_FILENAME: &str = "mymod";
const PYTHON_FILENAME_EXTENSION: &str = ".py";

impl PythonInterpreter {
    pub fn is_entrypoint(&self, fragments: &[Fragment], function: &Function) -> bool {
        <Self as LangInterpreter>::is_entrypoint(self, fragments, function)
    }

    pub fn execute(&self, fragments: &[Fragment], function: &Function, args: &[Argument]) {
        <Self as LangInterpreter>::execute(self, fragments, function, args)
    }

    pub fn new() -> Self {
        <Self as LangInterpreter>::new()
    }

    fn build_module<'py>(&self, py: Python<'py>, fragments: &[Fragment]) -> Bound<'py, PyModule> {
        let codebase: String = fragments.iter().map(|f| f.raw_data()).collect();

        let code = CString::new(codebase).expect("invalid python code");
        let filename = CString::new(format!(
            "{}{}",
            PYTHON_MOD_FILENAME, PYTHON_FILENAME_EXTENSION
        ))
        .unwrap();
        let module_name = CString::new(PYTHON_MOD_FILENAME).unwrap();

        PyModule::from_code(
            py,
            code.as_c_str(),
            filename.as_c_str(),
            module_name.as_c_str(),
        )
        .expect("failed to create python module")
    }
}

impl LangInterpreter for PythonInterpreter {
    fn new() -> Self {
        Python::with_gil(|py| {
            let sys = py.import("sys").expect("sys");
            let path = sys.getattr("path").expect("path");

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

            path.call_method1("insert", (0, &cactuskit_path))
                .expect("sys.path insert");

            Self {
                lang: Lang::new(Language::Python),
                sys: sys.into(),
                path: path.into(),
                cactuskit_path,
            }
        })
    }

    fn lang(&self) -> &Lang {
        &self.lang
    }

    fn execute(&self, fragments: &[Fragment], function: &Function, _args: &[Argument]) {
        Python::with_gil(|py| {
            let module = self.build_module(py, fragments);

            let handler = module.getattr(function.name()).expect("function not found");

            handler.call0().expect("execution failed");
        })
    }

    fn is_entrypoint(&self, fragments: &[Fragment], function: &Function) -> bool {
        Python::with_gil(|py| {
            let module = self.build_module(py, fragments);

            let handler = match module.getattr(function.name()) {
                Ok(h) => h,
                Err(_) => return false,
            };

            handler
                .getattr(REGISTERED_PROPERTY_ID)
                .and_then(|v| v.extract::<bool>())
                .unwrap_or(false)
        })
    }
}
