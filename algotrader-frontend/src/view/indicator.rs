use std::str::FromStr;

use log::error;
use plotly::Plot;
use yew::prelude::*;
use yew_plotly::{Plotly, plotly};

use algotrader_api::endpoints;
use algotrader_api::types::*;

use crate::utils::api_utils::fetch_single_api_response;
use crate::view::plots::indicator_plot;

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
                            let plot = indicator_plot(indicator_type, fetched_data);
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