use cactus_foundation::fragment::Fragment;
use pyo3::types::PyModule;
use pyo3::{Bound, Python};
use std::ffi::CString;

const PYTHON_MOD_FILENAME: &str = "mymod";
const PYTHON_FILENAME_EXTENSION: &str = ".py";

pub fn build_module<'py>(
    py: Python<'py>,
    fragments: &[Fragment],
) -> Bound<'py, PyModule> {
    let codebase: String = fragments
        .iter()
        .map(|f| f.raw_data())
        .collect();

    let code = CString::new(codebase).expect("invalid python code");
    let filename = CString::new(format!(
        "{}{}",
        PYTHON_MOD_FILENAME,
        PYTHON_FILENAME_EXTENSION
    )).unwrap();
    let module_name = CString::new(PYTHON_MOD_FILENAME).unwrap();

    PyModule::from_code(
        py,
        code.as_c_str(),
        filename.as_c_str(),
        module_name.as_c_str(),
    )
        .expect("failed to create python module")
}
