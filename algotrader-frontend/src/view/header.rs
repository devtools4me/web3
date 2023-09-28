use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;
use algotrader_api::types::AverageType;
use strum::IntoEnumIterator;

#[function_component(Header)]
pub fn header() -> Html {
    let methods_html = AverageType::iter()
        .map(|x| Route::from_average_type(&x))
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
                    <div class="navbar-item has-dropdown is-hoverable">
                        <a class="navbar-link">
                            { "Methods" }
                        </a>
                        <div class="navbar-dropdown">
                            {for methods_html}
                        </div>
                    </div>
                    <div class="navbar-item has-dropdown is-hoverable">
                        <a class="navbar-link">
                            { "Indicators" }
                        </a>
                        <div class="navbar-dropdown">
                            <Link<Route> to={ Route::MACD } classes="navbar-item">
                                { "MACD" }
                            </Link<Route>>
                            <Link<Route> to={ Route::RSI } classes="navbar-item">
                                { "RSI" }
                            </Link<Route>>
                            <Link<Route> to={ Route::RunTogether } classes="navbar-item">
                                { "Run Together" }
                            </Link<Route>>
                            <Link<Route> to={ Route::SellVolatility } classes="navbar-item">
                                { "Sell Volatility" }
                            </Link<Route>>
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