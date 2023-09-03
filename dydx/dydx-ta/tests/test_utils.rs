use std::{fs, io};
use std::path::PathBuf;
use dydx_api::types::Ohlc;

pub fn read_str(path: &str) -> io::Result<String> {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push(path);
    fs::read_to_string(file_path.as_path())
}

pub fn read_ohlc(path: &str) -> Vec<Ohlc> {
    let json = read_str(path).unwrap();
    serde_json::from_str(json.as_str()).unwrap()
}