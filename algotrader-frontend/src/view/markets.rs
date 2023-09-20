use log::error;
use plotly::{Plot, Scatter};
use yew::prelude::*;
use yew_plotly::{Plotly, plotly};
use yew_plotly::plotly::color::NamedColor;

use algotrader_api::path::*;
use algotrader_api::types::*;

use crate::utils::api_utils::fetch_single_api_response;

#[derive(Properties, PartialEq)]
struct MarketsProps {
    markets: Vec<String>,
}

#[function_component(MarletsDatalist)]
fn markets_datalist(MarketsProps { markets }: &MarketsProps) -> Html {
    let market_data_html = markets.iter().map(|x| {
        let selected = x.starts_with("BTC");
        html! {
            <option selected={selected} value={x.clone()}>{x.clone()}</option>
        }
    });
    html! {
        <div class="select">
            <select class="is-focused">
                {for market_data_html}
            </select>
        </div>
    }
}

#[function_component(MarketsSelect)]
pub fn markets_select_component() -> Html {
    let endpoint = markets();
    let state = use_state(|| vec![]);
    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_single_api_response::<Vec<String>>(endpoint.as_str())
                        .await
                    {
                        Ok(fetched_data) => {
                            state.set(fetched_data);
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
                <h1 class="title">{"Market"}</h1>
                <MarletsDatalist markets={(*state).clone()} />
            </div>
        </div>
    }
}