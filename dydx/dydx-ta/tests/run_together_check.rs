pub mod test_utils;

#[cfg(test)]
mod test {
    use algotrader_api::types::*;
    use dydx_ta::ops::ToIndicator;
    use dydx_ta::run_together::RunTogether;

    use crate::test_utils;

    #[test]
    fn test_run_together() {
        let v: Vec<Ohlc> = test_utils::read_ohlc("resources/test/BTC-USD-1DAY.json");
        let mut sut = RunTogether::default();
        let result = sut.to_indicator(v);
        println!("{:?}", result);

        assert_eq!(result.len(), 100);
    }
}