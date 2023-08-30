pub mod test_utils;

#[cfg(test)]
mod test {
    use dydx_api::types::*;
    use dydx_ta::indicators::run_together;
    use crate::test_utils;

    #[test]
    fn test_run_together() {
        let json = test_utils::read_str("resources/test/BTC-USD-1DAY.json");
        let v: Vec<Ohlc> = serde_json::from_str(json.as_str())
            .expect("Should have been able to read the file");

        let result = run_together(v);
        println!("{:?}", result);

        assert_eq!(result.len(), 100);
    }
}