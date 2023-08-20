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
                    <div class="navbar-item has-dropdown is-hoverable">
                        <Link<Route> to={ Route::Home } classes="navbar-link">
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> to={ Route::Sma } classes="navbar-link">
                            { "SMA" }
                        </Link<Route>>
                        <Link<Route> to={ Route::About } classes="navbar-link">
                            { "About" }
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </nav>
    }
}