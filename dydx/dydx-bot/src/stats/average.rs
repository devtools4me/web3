use yata::core::PeriodType;
use yata::methods::*;
use yata::prelude::*;

use dydx_api::types::Timeseries;

pub fn ema(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut ema = EMA::new(window_size, &v0).unwrap();
    convert(v, 1, |x| {
        let value = ema.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

pub fn sma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut sma = SMA::new(window_size, &v0).unwrap();
    convert(v, 1, |x| {
        let value = sma.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

pub fn hma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    todo!()
}

pub fn dema(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    todo!()
}

pub fn dma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    todo!()
}

pub fn rma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    todo!()
}

pub fn swma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    todo!()
}

pub fn tema(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    todo!()
}

pub fn tma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    todo!()
}

pub fn trima(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    todo!()
}

pub fn vwma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    todo!()
}

pub fn vidya(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    todo!()
}

pub fn wma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    todo!()
}

pub fn wsma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    todo!()
}

fn convert<A, B>(v: Vec<A>, skip_n: usize, f: impl FnMut(A) -> B) -> Vec<B> {
    v.into_iter().skip(skip_n).map(f).collect()
}