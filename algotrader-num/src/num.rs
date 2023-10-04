use std::fs;
use std::path::Path;

use pyo3::prelude::*;
use pyo3::types::PyList;

use crate::types::*;

pub fn get_cointegration(req: CointRequest) -> PyResult<CointResponse> {
    let json = serde_json::to_string(&req).unwrap();
    get_cointegration_json(json.as_str())
        .map(|x| serde_json::from_str::<CointResponse>(x.as_str()).unwrap())
}

pub fn get_cointegration_json(req: &str) -> PyResult<String> {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/py-func"));
    let py_app = fs::read_to_string(path.join("num_func.py"))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &path)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr("get_cointegration_json")?
            .into();
        app.call1(py, (req,))
    });

    from_python.map(|x| x.to_string())
}