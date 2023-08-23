use log::error;
use yew::prelude::*;
use yew_plotly::plotly::{Plot, Scatter};
use yew_plotly::{Plotly, plotly};
use yew_plotly::plotly::common::Mode;

use dydx_api::types::*;
use dydx_api::path::*;

use crate::utils::api_utils::fetch_single_api_response;

#[function_component(AverageChartView)]
pub fn average_chart_component() -> Html {
    let market = "BTC-USD";
    let resolution = "1DAY";
    let title = format!("SMA {} {}", market, resolution);
    let state = use_state(|| Plot::new());
    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_single_api_response::<Vec<Timeseries>>(
                        averages(AverageType::SMA, market, resolution).as_str(),
                    )
                        .await
                    {
                        Ok(fetched_data) => {
                            let date = fetched_data.iter()
                                .map(|x| x.timestamp.clone())
                                .collect::<Vec<_>>();
                            let value = fetched_data.iter()
                                .map(|x| x.value.parse::<f64>().unwrap())
                                .collect::<Vec<_>>();
                            let trace = Scatter::new(date, value);
                            let mut plot = Plot::new();
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