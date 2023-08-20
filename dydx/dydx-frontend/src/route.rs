use yew::prelude::*;
use yew_router::prelude::*;

use crate::view::ohlc::OhlcView;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/about")]
    About,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <OhlcView />},
        Route::About => html! { <p class="text-white">{ "Not found" }</p> },
        Route::NotFound => html! { <p class="text-white">{ "Not found" }</p> },
    }
}