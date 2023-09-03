use yata::helpers::{MA, RandomCandles};
use yata::core::{IndicatorResult, PeriodType, Action};
use yata::indicators::*;
use yata::prelude::*;

use dydx_api::types;
use dydx_api::types::{ActionType, Indicator};
use dydx_common::utils::vec_utils::*;
use log::*;
use crate::run_together::RunTogether;
use crate::sell_volatility::SellVolatility;
use crate::source_change::SourceChange;

pub fn macd(v: Vec<types::Ohlc>) -> Vec<Indicator> {
    let mut macd = MACD::default();
    macd.signal = MA::TEMA(5);

    let mut macd = macd.init(&v.first().unwrap().convert()).unwrap();
    convert(v, |x| {
        let value = macd.next(&x.convert());
        let res = indicator(&value, x.timestamp.as_str());
        debug!("value={:?}, res={:?}", value, res);
        res
    })
}

pub fn rsi(v: Vec<types::Ohlc>) -> Vec<Indicator> {
    let mut rsi = RSI::default();
    rsi.ma = MA::EMA(14);

    let mut rsi = rsi.init(&v.first().unwrap().convert()).unwrap();
    convert(v, |x| {
        let value = rsi.next(&x.convert());
        let res = indicator(&value, x.timestamp.as_str());
        debug!("value={:?}, res={:?}", value, res);
        res
    })
}

pub fn source_change(v: Vec<types::Ohlc>) -> Vec<Indicator> {
    let mut source_change = SourceChange::default();
    let mut source_change = source_change.init(&v.first().unwrap().convert()).unwrap();
    convert(v, |x| {
        let value = source_change.next(&x.convert());
        let res = indicator(&value, x.timestamp.as_str());
        debug!("value={:?}, res={:?}", value, res);
        res
    })
}

pub fn run_together(v: Vec<types::Ohlc>) -> Vec<Indicator> {
    let mut run_together = RunTogether::default();
    let mut run_together = run_together.init(&v.first().unwrap().convert()).unwrap();
    convert(v, |x| {
        let value = run_together.next(&x.convert());
        let res = indicator(&value, x.timestamp.as_str());
        debug!("value={:?}, res={:?}", value, res);
        res
    })
}

pub fn sell_volatility(v: Vec<types::Ohlc>) -> Vec<Indicator> {
    let mut sell_volatility = SellVolatility::default();
    let mut sell_volatility = sell_volatility.init(&v.first().unwrap().convert()).unwrap();
    convert(v, |x| {
        let value = sell_volatility.next(&x.convert());
        let res = indicator(&value, x.timestamp.as_str());
        debug!("value={:?}, res={:?}", value, res);
        res
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