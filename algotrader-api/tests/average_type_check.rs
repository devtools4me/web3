use algotrader_api::types::{AverageType, IndicatorType};
use strum::IntoEnumIterator;
use std::str::FromStr;

#[test]
fn average_type_works() {
    assert_eq!(AverageType::EMA.to_string(), "EMA");
    assert_eq!(AverageType::from_str("EMA").unwrap(), AverageType::EMA);
    for i in AverageType::iter() {
        let s = i.to_string();
        println!("{}", s);
        assert_eq!(AverageType::from_str(s.as_str()).unwrap(), i);
    }
}

#[test]
fn indicator_type_works() {
    assert_eq!(IndicatorType::RSI.to_string(), "RSI");
    assert_eq!(IndicatorType::from_str("RSI").unwrap(), IndicatorType::RSI);
    for i in IndicatorType::iter() {
        let s = i.to_string();
        println!("{}", s);
        assert_eq!(IndicatorType::from_str(s.as_str()).unwrap(), i);
    }
}