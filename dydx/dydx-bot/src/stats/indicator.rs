use yata::helpers::{MA, RandomCandles};
use yata::core::{IndicatorResult, PeriodType, Action};
use yata::indicators::*;
use yata::prelude::*;

use dydx_api::types;
use dydx_api::types::{ActionType, Indicator};
use dydx_common::utils::vec_utils::*;
use log::*;

pub fn macd(v: Vec<types::Ohlc>) -> Vec<types::Indicator> {
    let mut macd = MACD::default();
    macd.signal = MA::TEMA(5);

    let mut macd = macd.init(&v.first().unwrap().convert()).unwrap();
    convert(v, |x| {
        let value = macd.next(&x.convert());
        info!("{:?}", value);
        indicator(&value, x.timestamp.as_str())
    })
}

pub fn rsi(v: Vec<types::Ohlc>) -> Vec<types::Indicator> {
    let mut rsi = RSI::default();
    rsi.ma = MA::EMA(14);

    let mut rsi = rsi.init(&v.first().unwrap().convert()).unwrap();
    convert(v, |x| {
        let value = rsi.next(&x.convert());
        info!("{:?}", value);
        indicator(&value, x.timestamp.as_str())
    })
}

pub trait Convert<T> {
    fn convert(&self) -> T;
}

impl Convert<Candle> for types::Ohlc {
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

fn action(a: Action) -> ActionType {
    match a {
        Action::Buy(_) => ActionType::Buy,
        Action::None => ActionType::None,
        Action::Sell(_) => ActionType::Sell
    }
}

fn indicator(i: &IndicatorResult, timestamp: &str) -> Indicator {
    let actions = i
        .signals()
        .iter()
        .take(i.signals_length() as usize)
        .map(|&x| action(x))
        .collect();
    let values = i
        .values()
        .iter()
        .take(i.values_length() as usize)
        .map(|&x| format!("{:>7.4}", x))
        .collect();
    Indicator {
        signals: actions,
        values: values,
        timestamp: String::from(timestamp),
    }
}