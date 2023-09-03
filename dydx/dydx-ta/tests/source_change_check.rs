pub mod test_utils;

#[cfg(test)]
mod test {
    use dydx_api::types::*;
    use dydx_ta::indicators::source_change;
    use dydx_ta::source_change::SourceChange;
    use crate::test_utils;

    #[test]
    fn test_source_change() {
        let v = test_utils::read_ohlc("resources/test/BTC-USD-1DAY.json");
        let mut sut = SourceChange::default();
        let result = source_change.to_indicator(v);
        println!("{:?}", result);

        assert_eq!(result.len(), 100);
    }
}