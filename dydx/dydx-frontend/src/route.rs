use yew::prelude::*;
use yew_router::prelude::*;

use crate::view::{ohlc::OhlcView, ohlc::OhlcChartView, average::AverageChartView};
use dydx_api::types::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/about")]
    About,
    #[at("/ohlc")]
    Ohlc,
    #[at("/sma")]
    Sma,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <OhlcView />},
        Route::Ohlc => html! { <OhlcChartView />},
        Route::Sma => html! { <AverageChartView average_type={AverageType::SMA} />},
        Route::About => html! { <p class="text-white">{ "Not found" }</p> },
        Route::NotFound => html! { <p class="text-white">{ "Not found" }</p> },
    }
}