use std::path::Path;

use pyo3::prelude::*;

use crate::py::{py_call1, PyConf};
use crate::types::*;

pub fn get_cointegration(req: CointRequest) -> PyResult<CointResponse> {
    let json = serde_json::to_string(&req).unwrap();
    py_call1(PyConf {
        dir: Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/py-func")),
        script: "num_func.py",
        func: "get_cointegration_json",
    }, json.as_str())
        .map(|x| serde_json::from_str::<CointResponse>(x.as_str()).unwrap())
}

pub fn get_spread_z_score(req: SpreadRequest) -> PyResult<SpreadResponse> {
    let json = serde_json::to_string(&req).unwrap();
    py_call1(PyConf {
        dir: Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/resources/py-func")),
        script: "num_func.py",
        func: "get_spread_z_score_json",
    }, json.as_str())
        .map(|x| serde_json::from_str::<SpreadResponse>(x.as_str()).unwrap())
}