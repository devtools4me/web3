pub mod test_utils;

#[cfg(test)]
mod test {
    use yata::prelude::*;
    use yata::core::Source;
    use yata::helpers::MA;
    use algotrader_api::types::{*, SourceChangeIndicator};
    use algotrader_common::utils::vec_utils::*;
    use algotrader_ta::ops::ToIndicator;
    use algotrader_ta::source_change::*;

    use crate::test_utils;

    #[test]
    fn test_source_change_set() {
        let mut sut = SourceChange::default();
        sut.set("ma", String::from("ema-10")).unwrap();
        sut.set("source", String::from("Volume")).unwrap();
        sut.set("k", String::from("1.5")).unwrap();
        println!("{:?}", sut);

        assert_eq!(sut.ma, MA::EMA(10));
        assert_eq!(sut.source, Source::Volume);
        assert_eq!(sut.k, 1.5);
    }

    #[test]
    fn test_source_change() {
        let v = test_utils::read_ohlc("resources/test/BTC-USD-1DAY.json");
        let mut sut = SourceChange::default();
        let result = sut.to_indicator(v);
        println!("{:?}", result);

        assert_eq!(result.len(), 100);
    }

    #[test]
    fn test_source_change_volume() {
        let v = vec![
            Ohlc::from(("10", "20", "9", "15", "100", "2023-08-28T23:30:40.468Z")),
            Ohlc::from(("10", "20", "9", "15", "200", "2023-08-28T23:31:40.468Z")),
            Ohlc::from(("10", "20", "9", "15", "100", "2023-08-28T23:32:40.468Z")),
        ];
        let mut sut = SourceChange {
            ma: MA::EMA(20),
            source: Source::Volume,
            k: 2.0,
        };
        let result: Vec<Indicator> = sut.to_indicator(v);
        println!("{:?}", result);

        assert_eq!(result.len(), 3);

        let result: Vec<ActionType> = convert(result, |x| {
            let i: SourceChangeIndicator = SourceChangeIndicator::from(&x);
            i.signal
        });
        println!("{:?}", result);

        assert_eq!(result, vec![ActionType::None, ActionType::Buy, ActionType::None])
    }

    #[test]
    fn test_source_change_close() {
        let v = vec![
            Ohlc::from(("10", "20", "9", "10", "100", "2023-08-28T23:30:40.468Z")),
            Ohlc::from(("10", "20", "9", "20", "100", "2023-08-28T23:31:40.468Z")),
            Ohlc::from(("10", "20", "9", "10", "100", "2023-08-28T23:32:40.468Z")),
        ];
        let mut sut = SourceChange {
            ma: MA::EMA(20),
            source: Source::Close,
            k: 2.0,
        };
        let result = sut.to_indicator(v);
        println!("{:?}", result);

        assert_eq!(result.len(), 3);

        let result: Vec<ActionType> = convert(result, |x| {
            let i: SourceChangeIndicator = SourceChangeIndicator::from(&x);
            i.signal
        });
        println!("{:?}", result);

        assert_eq!(result, vec![ActionType::None, ActionType::Buy, ActionType::None])
    }
}