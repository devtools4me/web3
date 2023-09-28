use std::str::FromStr;
use log::error;
use plotly::{Plot, Scatter};
use yew::prelude::*;
use yew_plotly::{Plotly, plotly};
use yew_plotly::plotly::color::NamedColor;

use algotrader_api::endpoints;
use algotrader_api::types::*;

use crate::utils::api_utils::fetch_single_api_response;

#[derive(Properties, PartialEq)]
pub struct IndicatorChartProps {
    pub indicator_type: IndicatorType,
    pub market: String,
    pub resolution: String
}

#[function_component(IndicatorChartView)]
pub fn indicator_chart_component(IndicatorChartProps { indicator_type, market, resolution }: &IndicatorChartProps) -> Html {
    let indicator_type = IndicatorType::from_str(indicator_type.to_string().as_str()).unwrap();
    let endpoint = endpoints::indicators(&indicator_type, market, resolution);
    let title = format!("{} {} {}", IndicatorType::description(&indicator_type), market, resolution);
    let state = use_state(|| Plot::new());
    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_single_api_response::<Vec<Indicator>>(endpoint.as_str())
                        .await
                    {
                        Ok(fetched_data) => {
                            let traces = scatter(&indicator_type, fetched_data);
                            let mut plot = Plot::new();
                            for t in traces.iter() {
                                plot.add_trace(t.clone());
                            }
                            state.set(plot);
                        }
                        Err(e) => {
                            error!("{e}")
                        }
                    }
                });
            },
            (),
        );
    }

    html! {
        <div class="section">
            <div class="container">
                <h1 class="title">{title.as_str()}</h1>
                <Plotly plot={(*state).clone()}/>
            </div>
        </div>
    }
}

fn scatter(indicator_type: &IndicatorType, fetched_data: Vec<Indicator>) -> Vec<Box<Scatter<String, f64>>> {
    match indicator_type {
        IndicatorType::MACD => scatter_macd(fetched_data),
        IndicatorType::RSI => scatter_rsi(fetched_data),
        IndicatorType::RunTogether => trace_na(fetched_data),
        IndicatorType::SellVolatility => trace_na(fetched_data),
    }
}

fn scatter_macd(fetched_data: Vec<Indicator>) -> Vec<Box<Scatter<String, f64>>> {
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

fn scatter_rsi(fetched_data: Vec<Indicator>) -> Vec<Box<Scatter<String, f64>>> {
    let date = fetched_data.iter()
        .map(|x| x.timestamp.clone())
        .collect();
    let value = fetched_data.iter()
        .map(|x| rsi_indicator(x))
        .map(|x| x.value.parse::<f64>().unwrap() * 100.0)
        .collect();
    vec![Scatter::new(date, value)]
}

fn trace_na(fetched_data: Vec<Indicator>) -> Vec<Box<Scatter<String, f64>>> {
    let date = fetched_data.iter()
        .map(|x| x.timestamp.clone())
        .collect();
    let value = fetched_data.iter()
        .map(|x| 0.0)
        .collect();
    vec![Scatter::new(date, value)]
}