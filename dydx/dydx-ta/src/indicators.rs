use yata::helpers::{MA, RandomCandles};
use yata::core::{IndicatorResult, PeriodType, Action};
use yata::indicators::*;
use yata::prelude::*;

use dydx_api::types;
use dydx_api::types::{ActionType, Indicator, Ohlc};
use dydx_common::utils::vec_utils::*;
use log::*;
use crate::run_together::RunTogether;
use crate::sell_volatility::SellVolatility;
use crate::source_change::SourceChange;
use crate::ops::{Convert, ToIndicator, indicator};

pub fn macd(v: Vec<Ohlc>) -> Vec<Indicator> {
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

pub fn rsi(v: Vec<Ohlc>) -> Vec<Indicator> {
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

pub fn source_change(v: Vec<Ohlc>) -> Vec<Indicator> {
    let mut source_change = SourceChange::default();
    source_change.to_indicator(v)
}

pub fn run_together(v: Vec<Ohlc>) -> Vec<Indicator> {
    let mut run_together = RunTogether::default();
    let mut run_together = run_together.init(&v.first().unwrap().convert()).unwrap();
    convert(v, |x| {
        let value = run_together.next(&x.convert());
        let res = indicator(&value, x.timestamp.as_str());
        debug!("value={:?}, res={:?}", value, res);
        res
    })
}

pub fn sell_volatility(v: Vec<Ohlc>) -> Vec<Indicator> {
    let mut sell_volatility = SellVolatility::default();
    let mut sell_volatility = sell_volatility.init(&v.first().unwrap().convert()).unwrap();
    convert(v, |x| {
        let value = sell_volatility.next(&x.convert());
        let res = indicator(&value, x.timestamp.as_str());
        debug!("value={:?}, res={:?}", value, res);
        res
    })
}