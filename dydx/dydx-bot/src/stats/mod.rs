use yata::core::PeriodType;
use yata::methods::SMA;
use yata::prelude::*;

use dydx_api::types::Timeseries;

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

fn convert<A, B>(v: Vec<A>, skip_n: usize, f: impl FnMut(A) -> B) -> Vec<B> {
    v.into_iter().skip(skip_n).map(f).collect()
}