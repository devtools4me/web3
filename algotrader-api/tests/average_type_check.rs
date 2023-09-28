use algotrader_api::types::AverageType;
use strum::IntoEnumIterator;
use std::str::FromStr;

#[test]
fn format_price_works() {
    assert_eq!(AverageType::EMA.to_string(), "EMA");
    assert_eq!(AverageType::parse("EMA"), AverageType::EMA);
    for i in AverageType::iter() {
        let s = i.to_string();
        assert_eq!(AverageType::from_str(s.as_str()).unwrap(), i);
    }
}