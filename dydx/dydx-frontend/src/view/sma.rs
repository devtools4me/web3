use log::error;
use yew::prelude::*;

use crate::js::bindings;

#[function_component(SmaChartView)]
pub fn ohlc_component() -> Html {
    html! {
        <div class="section">
            <div class="container">
                <h1 class="title">{"SMA Chart"}</h1>
                <h2 class="subtitle">{bindings::get_now_date()}</h2>
            </div>
        </div>
    }
}