use log::debug;
use yata::core::{Action, Candle, IndicatorConfigDyn, IndicatorResult};
use dydx_api::types::{ActionType, Indicator, Ohlc};
use dydx_common::utils::vec_utils::convert;
use crate::source_change::SourceChange;

pub trait Convert<T> {
    fn convert(&self) -> T;
}

impl Convert<Candle> for Ohlc {
    fn convert(&self) -> Candle {
        Candle {
            open: self.open.parse::<f64>().unwrap(),
            high: self.high.parse::<f64>().unwrap(),
            low: self.low.parse::<f64>().unwrap(),
            close: self.close.parse::<f64>().unwrap(),
            volume: self.volume.parse::<f64>().unwrap(),
        }
    }
}

pub trait ToIndicator {
    fn to_indicator(&self, v: Vec<Ohlc>) -> Vec<Indicator>;
}

impl ToIndicator for SourceChange {
    fn to_indicator(&self, v: Vec<Ohlc>) -> Vec<Indicator> {
        let mut source_change = self.init(&v.first().unwrap().convert()).unwrap();
        convert(v, |x| {
            let value = source_change.next(&x.convert());
            let res = indicator(&value, x.timestamp.as_str());
            debug!("value={:?}, res={:?}", value, res);
            res
        })
    }
}

pub fn indicator(i: &IndicatorResult, timestamp: &str) -> Indicator {
    let signals = i
        .signals()
        .iter()
        .take(i.signals_length() as usize)
        .map(|&x| action(x))
        .collect();
    let values = i
        .values()
        .iter()
        .take(i.values_length() as usize)
        .map(|&x| x.to_string())
        .collect();
    Indicator {
        signals,
        values,
        timestamp: String::from(timestamp),
    }
}

fn action(a: Action) -> ActionType {
    match a {
        Action::Buy(_) => ActionType::Buy,
        Action::None => ActionType::None,
        Action::Sell(_) => ActionType::Sell
    }
}