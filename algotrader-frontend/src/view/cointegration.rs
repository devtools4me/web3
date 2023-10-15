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
pub struct CointegrationDataProps {
    pub data: CointegrationData
}

#[function_component(CointegrationDataView)]
pub fn cointegration_data_component(CointegrationDataProps { data }: &CointegrationDataProps) -> Html {
    let CointegrationData {p_value, coint_t, c_value, hedge_ratio, zero_crossings} = data;
    let data = vec![
        ("p_value", p_value.to_string()),
        ("coint_t", coint_t.to_string()),
        ("c_value", c_value.to_string()),
        ("hedge_ratio", hedge_ratio.to_string()),
        ("zero_crossings", zero_crossings.to_string()),
    ];
    let coint_data_html = data.iter().map(|x| {
        html! {
            <tr>
                <td>{x.0.clone()}</td>
                <td>{x.1.clone()}</td>
            </tr>
        }
    });
    html! {
        <div>
            <table class="table is-hoverable is-striped">
                <thead>
                    <tr>
                        <th>{"Property"}</th>
                        <th>{"Value"}</th>
                    </tr>
                </thead>
                <tbody>
                {for coint_data_html}
                </tbody>
            </table>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CointegrationProps {
    pub market1: String,
    pub market2: String,
    pub resolution: String
}

#[function_component(CointegrationView)]
pub fn cointegration_component(props: &CointegrationProps) -> Html {
    let CointegrationProps { market1, market2, resolution } = props;
    let endpoint = endpoints::cointegration(market1.as_str(), market2.as_str(), resolution.as_str());
    let title = format!("Cointegration {} {} {}", market1, market2, resolution);
    let state = use_state(|| CointegrationData::default());
    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_single_api_response::<CointegrationData>(endpoint.as_str())
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
                <h1 class="title">{title.as_str()}</h1>
                <CointegrationDataView data={(*state).clone()}/>
            </div>
        </div>
    }
}