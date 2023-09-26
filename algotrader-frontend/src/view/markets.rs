use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

use algotrader_api::types::*;

use crate::types::props::{MarketsProps, StrCbProps};
use crate::types::AppState;

#[function_component(MarketsDatalist)]
fn markets_datalist(props: &MarketsProps) -> Html {
    let callback = props.callback.clone();
    let app_context = use_context::<AppState>();
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
    let callback = props.callback.clone();
    let app_context = use_context::<AppState>();
    html! {
        <div class="section">
            <div class="container">
                <h1 class="title">{"Market"}</h1>
                <MarketsDatalist markets={app_context.clone().unwrap_or_default().markets} callback={callback}/>
            </div>
        </div>
    }
}