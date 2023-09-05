pub mod test_utils;

#[cfg(test)]
mod test {
    use yata::core::Source;
    use yata::helpers::MA;
    use dydx_api::types::*;
    use dydx_api::types::New;
    use dydx_ta::ops::ToIndicator;
    use dydx_ta::source_change::SourceChange;

    use crate::test_utils;

    #[test]
    fn test_source_change() {
        let v = test_utils::read_ohlc("resources/test/BTC-USD-1DAY.json");
        let mut sut = SourceChange::default();
        let result = sut.to_indicator(v);
        println!("{:?}", result);

        assert_eq!(result.len(), 100);
    }

    #[test]
    fn test_source_change_2() {
        let v = vec![
            Ohlc::new(("26059", "26196", "25841", "26137", "100", "2023-08-28T23:30:40.468Z")),
            Ohlc::new(("26059", "26196", "25841", "26137", "200", "2023-08-28T23:30:40.468Z"))
        ];
        let mut sut = SourceChange {
            ma: MA::EMA(20),
            source: Source::Volume,
            k: 2.0
        };
        let result = sut.to_indicator(v);
        println!("{:?}", result);

        assert_eq!(result.len(), 2);
    }
}