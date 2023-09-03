pub mod test_utils;

#[cfg(test)]
mod test {
    use dydx_api::types::*;
    use dydx_ta::indicators::run_together;
    use crate::test_utils;

    #[test]
    fn test_run_together() {
        let v: Vec<Ohlc> = test_utils::read_ohlc("resources/test/BTC-USD-1DAY.json");
        let result = run_together(v);
        println!("{:?}", result);

        assert_eq!(result.len(), 100);
    }
}