use std::fs;
use std::path::Path;

use log::info;
use pyo3::prelude::*;
use pyo3::types::PyList;

#[derive(Clone, PartialEq, Debug)]
pub struct PyConf<'a> {
    pub dir: &'a Path,
    pub script: &'a str,
    pub func: &'a str,
}

pub fn py_call1(conf: PyConf, args: &str) -> PyResult<String> {
    info!("py_call1, conf={:?}", conf);
    let py_app = fs::read_to_string(conf.dir.join(conf.script))?;
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let syspath: &PyList = py.import("sys")?.getattr("path")?.downcast::<PyList>()?;
        syspath.insert(0, &conf.dir)?;
        let app: Py<PyAny> = PyModule::from_code(py, &py_app, "", "")?
            .getattr(conf.func)?
            .into();
        app.call1(py, (args, ))
    });

    from_python.map(|x| x.to_string())
}