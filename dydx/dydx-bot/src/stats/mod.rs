use yata::core::PeriodType;
use yata::methods::SMA;
use yata::prelude::*;

use dydx_api::types::Timeseries;

pub fn sma(v: Vec<Timeseries>, len: PeriodType) -> Vec<Timeseries> {
    let mut sma = SMA::new(len, &0.0).unwrap();
    convert(v, |x| {
        let value = sma.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

fn convert<A, B>(v: Vec<A>, f: impl FnMut(A) -> B) -> Vec<B> {
    v.into_iter().map(f).collect()
}