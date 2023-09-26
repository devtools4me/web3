use log::error;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use algotrader_api::endpoints;
use algotrader_api::types::*;
use crate::types::props::{StrCbProps, MarketsProps};

use crate::utils::api_utils::fetch_single_api_response;

#[function_component(MarketsDatalist)]
fn markets_datalist(props: &MarketsProps) -> Html {
    let callback = props.callback.clone();
    let on_market_change = {
        Callback::from(move |e: Event| {
            let input = e.target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(input) = input {
                //info!("index={}, value={}", input.selected_index(), input.value());
                callback.emit(input.value());
            }
        })
    };
    let market_data_html = props.markets.iter().map(|x| {
        let selected = x.starts_with("BTC");
        html! {
            <option selected={selected} value={x.clone()}>{x.clone()}</option>
        }
    });
    html! {
        <div class="select">
            <select class="is-focused" onchange={on_market_change}>
                {for market_data_html}
            </select>
        </div>
    }
}

#[function_component(MarketsSelect)]
pub fn markets_select_component(props: &StrCbProps) -> Html {
    let endpoint = endpoints::markets();
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

    let callback = props.callback.clone();
    html! {
        <div class="section">
            <div class="container">
                <h1 class="title">{"Market"}</h1>
                <MarketsDatalist markets={(*state).clone()} callback={callback}/>
            </div>
        </div>
    }
}