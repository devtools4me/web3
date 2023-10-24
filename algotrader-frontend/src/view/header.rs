use strum::IntoEnumIterator;
use yew::prelude::*;
use yew_router::prelude::*;

use algotrader_api::types::IndicatorType;

use crate::route::Route;

#[function_component(Header)]
pub fn header() -> Html {
    let indicators_html = IndicatorType::iter()
        .map(|x| Route::from_indicator_type(&x))
        .map(|x| {
            let route = x.clone();
            let s = route.to_string();
            html! {
                <Link<Route> to={ route } classes="navbar-item">
                    { s }
                </Link<Route>>
            }
        });

    html! {
        <nav class="navbar is-transparent">
            <div class="navbar-brand">
                <div class="navbar-burger" data-target="navbarExampleTransparentExample">
                    <span></span>
                    <span></span>
                    <span></span>
                </div>
            </div>

            <div id="navbarExampleTransparentExample" class="navbar-menu">
                <div class="navbar-start">
                    <Link<Route> to={ Route::Home } classes="navbar-link">
                        { "Home" }
                    </Link<Route>>
                    <Link<Route> to={ Route::Ohlc } classes="navbar-link">
                        { "OHLC" }
                    </Link<Route>>
                    <Link<Route> to={ Route::Methods } classes="navbar-link">
                        { "Methods" }
                    </Link<Route>>
                    <div class="navbar-item has-dropdown is-hoverable">
                        <a class="navbar-link">
                            { "Cointegration" }
                        </a>
                        <div class="navbar-dropdown">
                            <Link<Route> to={ Route::Cointegration } classes="navbar-link">
                                { "Cointegration" }
                            </Link<Route>>
                            <Link<Route> to={ Route::Spread } classes="navbar-link">
                                { "Spread & Z-Score" }
                            </Link<Route>>
                            <Link<Route> to={ Route::Trends } classes="navbar-link">
                                { "Trends" }
                            </Link<Route>>
                        </div>
                    </div>
                    <div class="navbar-item has-dropdown is-hoverable">
                        <a class="navbar-link">
                            { "Indicators" }
                        </a>
                        <div class="navbar-dropdown">
                            {for indicators_html}
                        </div>
                    </div>
                    <Link<Route> to={ Route::About } classes="navbar-link">
                        { "About" }
                    </Link<Route>>
                </div>
            </div>
        </nav>
    }
}