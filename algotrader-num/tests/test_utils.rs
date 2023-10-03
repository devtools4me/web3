use std::{fs, io};
use std::path::PathBuf;

pub fn read_str(path: &str) -> io::Result<String> {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push(path);
    fs::read_to_string(file_path.as_path())
}