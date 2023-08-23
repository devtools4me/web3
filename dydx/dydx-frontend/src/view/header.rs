use log::error;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;
use crate::utils::api_utils;

#[function_component(Header)]
pub fn header() -> Html {
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
                            { "Averages" }
                        </a>
                        <div class="navbar-dropdown">
                            <Link<Route> to={ Route::EMA } classes="navbar-item">
                                { "EMA" }
                            </Link<Route>>
                            <Link<Route> to={ Route::HMA } classes="navbar-item">
                                { "HMA" }
                            </Link<Route>>
                            <Link<Route> to={ Route::DEMA } classes="navbar-item">
                                { "DEMA" }
                            </Link<Route>>
                            <Link<Route> to={ Route::DMA } classes="navbar-item">
                                { "DMA" }
                            </Link<Route>>
                            <Link<Route> to={ Route::RMA } classes="navbar-item">
                                { "RMA" }
                            </Link<Route>>
                            <Link<Route> to={ Route::SMA } classes="navbar-item">
                                { "SMA" }
                            </Link<Route>>
                            <Link<Route> to={ Route::SWMA } classes="navbar-item">
                                { "SWMA" }
                            </Link<Route>>
                            <Link<Route> to={ Route::TEMA } classes="navbar-item">
                                { "TEMA" }
                            </Link<Route>>
                            <Link<Route> to={ Route::TMA } classes="navbar-item">
                                { "TMA" }
                            </Link<Route>>
                            <Link<Route> to={ Route::TRIMA } classes="navbar-item">
                                { "TRIMA" }
                            </Link<Route>>
                            <Link<Route> to={ Route::VWMA } classes="navbar-item">
                                { "VWMA" }
                            </Link<Route>>
                            <Link<Route> to={ Route::Vidya } classes="navbar-item">
                                { "Vidya" }
                            </Link<Route>>
                            <Link<Route> to={ Route::WMA } classes="navbar-item">
                                { "WMA" }
                            </Link<Route>>
                            <Link<Route> to={ Route::WSMA } classes="navbar-item">
                                { "WMA" }
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