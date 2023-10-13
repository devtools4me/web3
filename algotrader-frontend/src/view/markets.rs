use yew::prelude::*;

use algotrader_api::types::*;

use crate::types::props::{MarketsProps, StrCbPairProps, StrCbProps};
use crate::types::AppState;
use crate::utils::ui_utils::on_select_element;

#[function_component(MarketsDatalist)]
fn markets_datalist(props: &MarketsProps) -> Html {
    let callback = props.callback.clone();
    let app_context = use_context::<AppState>();
    let on_market_change = on_select_element(callback);
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

#[function_component(MarketsPairSelect)]
pub fn markets_pair_select_component(props: &StrCbPairProps) -> Html {
    let callback1 = props.callback1.clone();
    let callback2 = props.callback2.clone();
    let app_context = use_context::<AppState>();
    html! {
        <div class="section">
            <div class="container">
                <h1 class="title">{"Markets"}</h1>
                <MarketsDatalist markets={app_context.clone().unwrap_or_default().markets} callback={callback1}/>
                <MarketsDatalist markets={app_context.clone().unwrap_or_default().markets} callback={callback2}/>
            </div>
        </div>
    }
}