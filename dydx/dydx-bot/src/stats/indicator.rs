use yata::helpers::{MA, RandomCandles};
use yata::core::{IndicatorResult, PeriodType};
use yata::indicators::*;
use yata::prelude::*;

use dydx_api::types;
use dydx_common::utils::vec_utils::*;

pub fn macd(v: Vec<types::Ohlc>) -> Vec<IndicatorResult> {
    let mut macd = MACD::default();
    macd.signal = MA::TEMA(5);

    let candles = convert(v, |x| x.convert());
    let mut macd = macd.init(candles.first().unwrap()).unwrap();
    convert(candles, |x| macd.next(&x))
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