use plotly::{Plot, Scatter};
use yew_plotly::plotly;
use yew_plotly::plotly::color::NamedColor;

use algotrader_api::types::*;

pub fn indicator_plot(indicator_type: IndicatorType, v: Vec<Indicator>) -> Plot {
    let traces: Vec<Box<Scatter<String, f64>>> = scatter(&indicator_type, v);
    let mut plot = Plot::new();
    for t in traces.iter() {
        plot.add_trace(t.clone());
    }
    plot
}

pub fn scatter(indicator_type: &IndicatorType, fetched_data: Vec<Indicator>) -> Vec<Box<Scatter<String, f64>>> {
    match indicator_type {
        IndicatorType::MACD => scatter_macd(fetched_data),
        IndicatorType::RSI => scatter_rsi(fetched_data),
        IndicatorType::RunTogether => trace_na(fetched_data),
        IndicatorType::SellVolatility => trace_na(fetched_data),
    }
}

pub fn scatter_macd(fetched_data: Vec<Indicator>) -> Vec<Box<Scatter<String, f64>>> {
    let date: Vec<String> = fetched_data.iter()
        .map(|x| x.timestamp.clone())
        .collect();
    let macd_value = fetched_data.iter()
        .map(|x| macd_indicator(x))
        .map(|x| x.macd_value.parse::<f64>().unwrap())
        .collect();
    let sigline_value = fetched_data.iter()
        .map(|x| macd_indicator(x))
        .map(|x| x.sigline_value.parse::<f64>().unwrap())
        .collect();
    vec![
        Scatter::new(date.clone(), sigline_value).name("Signal Line")
            .line(plotly::common::Line::new().color(NamedColor::LightGreen)),
        Scatter::new(date.clone(), macd_value).name("MACD Line")
            .line(plotly::common::Line::new().color(NamedColor::Orange))
    ]
}

pub fn scatter_rsi(fetched_data: Vec<Indicator>) -> Vec<Box<Scatter<String, f64>>> {
    let date = fetched_data.iter()
        .map(|x| x.timestamp.clone())
        .collect();
    let value = fetched_data.iter()
        .map(|x| rsi_indicator(x))
        .map(|x| x.value.parse::<f64>().unwrap() * 100.0)
        .collect();
    vec![Scatter::new(date, value)]
}

pub fn trace_na(fetched_data: Vec<Indicator>) -> Vec<Box<Scatter<String, f64>>> {
    let date = fetched_data.iter()
        .map(|x| x.timestamp.clone())
        .collect();
    let value = fetched_data.iter()
        .map(|x| 0.0)
        .collect();
    vec![Scatter::new(date, value)]
}