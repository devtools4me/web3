use std::str::FromStr;

use log::error;
use yew::prelude::*;
use yew_plotly::Plotly;
use yew_plotly::plotly::{Plot, Scatter};

use algotrader_api::endpoints;
use algotrader_api::types::*;

use crate::utils::api_utils::fetch_single_api_response;

#[derive(Properties, PartialEq)]
pub struct AverageChartProps {
    pub average_type: AverageType,
    pub market: String,
    pub resolution: String
}

#[function_component(AverageChartView)]
pub fn average_chart_component(AverageChartProps { average_type, market, resolution }: &AverageChartProps) -> Html {
    let average_type = AverageType::from_str(average_type.to_string().as_str()).unwrap();
    let endpoint = endpoints::methods(&average_type, market, resolution);
    let title = format!("{} {} {}", AverageType::description(&average_type), market, resolution);
    let state = use_state(|| Plot::new());
    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_single_api_response::<Vec<Timeseries>>(endpoint.as_str())
                        .await
                    {
                        Ok(fetched_data) => {
                            let mut plot = Plot::new();
                            let trace = scatter(fetched_data);
                            plot.add_trace(trace);
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

fn scatter(v: Vec<Timeseries>) -> Box<Scatter<String, f64>> {
    let date = v.iter()
        .map(|x| x.timestamp.clone())
        .collect::<Vec<_>>();
    let value = v.iter()
        .map(|x| x.value.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    Scatter::new(date, value)
}