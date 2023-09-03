use yata::helpers::MA;
use yata::indicators::{MACD, RSI};
use dydx_api::types::{Indicator, Ohlc};

use crate::ops::ToIndicator;
use crate::run_together::RunTogether;
use crate::sell_volatility::SellVolatility;
use crate::source_change::SourceChange;

pub fn macd(v: Vec<Ohlc>) -> Vec<Indicator> {
    let mut alg = MACD::default();
    alg.signal = MA::TEMA(5);
    alg.to_indicator(v)
}

pub fn rsi(v: Vec<Ohlc>) -> Vec<Indicator> {
    let mut alg = RSI::default();
    alg.ma = MA::EMA(14);
    alg.to_indicator(v)
}

pub fn source_change(v: Vec<Ohlc>) -> Vec<Indicator> {
    let mut alg = SourceChange::default();
    alg.to_indicator(v)
}

pub fn run_together(v: Vec<Ohlc>) -> Vec<Indicator> {
    let mut alg = RunTogether::default();
    alg.to_indicator(v)
}

pub fn sell_volatility(v: Vec<Ohlc>) -> Vec<Indicator> {
    let mut alg = SellVolatility::default();
    alg.to_indicator(v)
}