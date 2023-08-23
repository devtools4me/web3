use yata::core::PeriodType;
use yata::methods::*;
use yata::prelude::*;

use dydx_api::types::Timeseries;
use dydx_common::utils::vec_utils::*;

pub fn rsi(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut avg = EMA::new(window_size, &v0).unwrap();
    convert(v, |x| {
        let value = avg.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}