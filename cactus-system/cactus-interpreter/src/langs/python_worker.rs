use crate::python_helper::build_module;
use cactus_foundation::fragment::Fragment;
use pyo3::types::PyAnyMethods;
use pyo3::types::{
    PyAny,
    PyDict,
    PyList,
    PyListMethods,
    PyModule,
    PyTuple,
    PyTupleMethods,
};
use pyo3::{Bound, Py, Python};
use crate::cactus_resp::CactusResponse;

pub struct PythonWorker {
    handler: Py<PyAny>,
}

impl PythonWorker {
    pub fn new(
        py: Python,
        fragments: &[Fragment],
        function_name: &str,
    ) -> Self {
        let module = build_module(py, fragments);
        let func = module.getattr(function_name).expect("fn missing");

        Self {
            handler: func.into(),
        }
    }

    pub fn invoke(
        &self,
        py: Python,
        args: serde_json::Value,
    ) -> CactusResponse {
        let json = PyModule::import(py, "json").expect("failed to import json");
        let args_json = serde_json::to_string(&args).expect("failed to serialize args");
        let py_args = json
            .getattr("loads")
            .unwrap()
            .call1((args_json,))
            .unwrap();
        let handler = self.handler.bind(py);
        let res = call_handler(py, handler, py_args);

        let (status, payload_any, timestamp) = normalize_response(py, res)
            .expect("failed to normalize response");
        let payload_json = json
            .getattr("dumps")
            .unwrap()
            .call1((payload_any,))
            .unwrap()
            .extract::<String>()
            .unwrap();
        let payload = serde_json::from_str::<serde_json::Value>(&payload_json)
            .expect("failed to parse payload");

        CactusResponse::ok(status, timestamp, payload)
    }
}

fn call_handler<'py>(
    py: Python<'py>,
    handler: &Bound<'py, PyAny>,
    args: Bound<'py, PyAny>,
) -> Bound<'py, PyAny> {
    if args.is_none() {
        return handler.call0().expect("handler call failed");
    }

    if let Ok(dict) = args.cast::<PyDict>() {
        let pos_any = dict.get_item("args").ok();
        let kwargs_any = dict.get_item("kwargs").ok();
        let has_args = pos_any.is_some();
        let has_kwargs = kwargs_any.is_some();

        if has_args || has_kwargs {
            let pos_tuple = to_tuple(py, pos_any.as_ref());
            let kwargs_dict = if let Some(kwargs_any) = kwargs_any {
                kwargs_any.extract::<Py<PyDict>>().ok()
            } else {
                None
            };

            let call_result = if let Some(kwargs_dict) = kwargs_dict {
                let kwargs_bound = kwargs_dict.bind(py);
                handler.call(pos_tuple, Some(&kwargs_bound))
            } else {
                handler.call(pos_tuple, None)
            };

            return call_result.expect("handler call failed");
        }

        return handler
            .call((), Some(dict))
            .expect("handler call failed");
    }

    if let Ok(tuple) = args.cast::<PyTuple>() {
        return handler.call(tuple, None).expect("handler call failed");
    }

    if let Ok(list) = args.cast::<PyList>() {
        let tuple = PyTuple::new(py, list.iter()).expect("failed to build args tuple");
        return handler.call(tuple, None).expect("handler call failed");
    }

    handler.call1((args,)).expect("handler call failed")
}

fn to_tuple<'py>(
    py: Python<'py>,
    value: Option<&Bound<'py, PyAny>>,
) -> Bound<'py, PyTuple> {
    let Some(value) = value else { return PyTuple::empty(py) };

    if value.is_none() {
        return PyTuple::empty(py);
    }

    if let Ok(tuple) = value.cast::<PyTuple>() {
        return tuple.clone();
    }

    if let Ok(list) = value.cast::<PyList>() {
        return PyTuple::new(py, list.iter()).expect("failed to build args tuple");
    }

    PyTuple::new(py, [value.clone()]).expect("failed to build args tuple")
}

fn normalize_response<'py>(
    py: Python<'py>,
    res: Bound<'py, PyAny>,
) -> Result<(u16, Bound<'py, PyAny>, f64), pyo3::PyErr> {
    if let Ok(dict) = res.cast::<PyDict>() {
        if let (Some(status_any), Some(payload_any), Some(timestamp_any)) = (
            dict.get_item("status").ok(),
            dict.get_item("payload").ok(),
            dict.get_item("timestamp").ok(),
        ) {
            let status = status_any.extract::<u16>()?;
            let timestamp = timestamp_any.extract::<f64>()?;
            return Ok((status, payload_any, timestamp));
        }
    }

    if res.hasattr("get_status_code")?
        && res.hasattr("get_payload")?
        && res.hasattr("get_timestamp")?
    {
        let status_any = res.getattr("get_status_code")?.call0()?;
        let status = status_any.extract::<u16>()?;
        let payload_any = res.getattr("get_payload")?.call0()?;
        let timestamp = res.getattr("get_timestamp")?.call0()?.extract::<f64>()?;
        return Ok((status, payload_any, timestamp));
    }

    if let Ok(tuple) = res.cast::<PyTuple>() {
        if tuple.len() == 2 {
            let status = tuple.get_item(0)?.extract::<u16>()?;
            let payload_any = tuple.get_item(1)?.clone();
            let timestamp = PyModule::import(py, "time")?
                .getattr("time")?
                .call0()?
                .extract::<f64>()?;
            return Ok((status, payload_any, timestamp));
        }
    }

    let timestamp = PyModule::import(py, "time")?
        .getattr("time")?
        .call0()?
        .extract::<f64>()?;
    Ok((200, res, timestamp))
}
