use yata::helpers::{MA, RandomCandles};
use yata::core::{IndicatorResult, PeriodType};
use yata::indicators::*;
use yata::prelude::*;

use dydx_api::types;
use dydx_api::types::Indicator;
use dydx_common::utils::vec_utils::*;
use log::*;

pub fn macd(v: Vec<types::Ohlc>) -> Vec<types::Indicator> {
    let mut macd = MACD::default();
    macd.signal = MA::TEMA(5);

    let mut macd = macd.init(&v.first().unwrap().convert()).unwrap();
    convert(v, |x| {
        let value = macd.next(&x.convert());
        info!("{:?}", value);
        Indicator {
            value: "".to_string(),
            timestamp: x.timestamp,
        }
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