use yata::core::PeriodType;
use yata::methods::*;
use yata::prelude::*;

use dydx_api::types::Timeseries;
use algotrader_common::utils::vec_utils::*;

pub fn ema(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
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

pub fn sma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut avg = SMA::new(window_size, &v0).unwrap();
    convert(v, |x| {
        let value = avg.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

pub fn hma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut avg = HMA::new(window_size, &v0).unwrap();
    convert(v, |x| {
        let value = avg.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

pub fn dema(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut avg = DEMA::new(window_size, &v0).unwrap();
    convert(v, |x| {
        let value = avg.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

pub fn dma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut avg = DMA::new(window_size, &v0).unwrap();
    convert(v, |x| {
        let value = avg.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

pub fn rma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut avg = RMA::new(window_size, &v0).unwrap();
    convert(v, |x| {
        let value = avg.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

pub fn swma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut avg = SWMA::new(window_size, &v0).unwrap();
    convert(v, |x| {
        let value = avg.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

pub fn tema(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut avg = TEMA::new(window_size, &v0).unwrap();
    convert(v, |x| {
        let value = avg.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

pub fn tma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut avg = TMA::new(window_size, &v0).unwrap();
    convert(v, |x| {
        let value = avg.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

pub fn trima(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut avg = TRIMA::new(window_size, &v0).unwrap();
    convert(v, |x| {
        let value = avg.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

pub fn vwma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    todo!()
}

pub fn vidya(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut avg = Vidya::new(window_size, &v0).unwrap();
    convert(v, |x| {
        let value = avg.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

pub fn wma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut avg = WMA::new(window_size, &v0).unwrap();
    convert(v, |x| {
        let value = avg.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

pub fn wsma(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut avg = WSMA::new(window_size, &v0).unwrap();
    convert(v, |x| {
        let value = avg.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}

pub fn momentum(v: Vec<Timeseries>, window_size: PeriodType) -> Vec<Timeseries> {
    let v0 = v[0].value.parse::<f64>().unwrap();
    let mut method = Momentum::new(window_size, &v0).unwrap();
    convert(v, |x| {
        let value = method.next(&x.value.parse::<f64>().unwrap());
        Timeseries {
            value: value.to_string(),
            timestamp: x.timestamp,
        }
    })
}