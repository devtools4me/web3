use std::ops::Deref;

use log::error;
use yew::ContextProvider;
use yew::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use algotrader_api::endpoints;
use algotrader_api::types::*;
use route::{Route, switch};
use view::header::Header;

use crate::types::AppState;
use crate::utils::api_utils::fetch_single_api_response;

mod utils;
mod view;
mod route;
mod js;
mod types;

#[function_component(App)]
fn app() -> Html {
    let endpoint = endpoints::markets();
    let state = use_state(|| AppState::default());
    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_single_api_response::<Vec<String>>(endpoint.as_str())
                        .await
                    {
                        Ok(markets) => {
                            state.set(AppState {
                                markets,
                                ..state.deref().clone()
                            });
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

    html! {
         <BrowserRouter>
            <ContextProvider<AppState> context={state.deref().clone()}>
                <Header />
                <Switch<Route> render={switch} />
            </ContextProvider<AppState>>
         </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}