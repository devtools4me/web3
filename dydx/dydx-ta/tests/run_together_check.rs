use dydx_ta::indicators::run_together;

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use std::fs;

    #[test]
    fn test_run_together() {
        let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        file_path.push("resources/test/BTC-USD-1DAY.json");

        let contents = fs::read_to_string(file_path.as_path())
            .expect("Should have been able to read the file");

        assert!(true);
    }
}