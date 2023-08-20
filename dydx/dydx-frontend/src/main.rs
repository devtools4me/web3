use log::error;
use yew::prelude::*;
use yew_router::prelude::*;

use route::{Route, switch};
use view::{header::Header, ohlc::OhlcView};

mod utils;
mod view;
mod route;
mod js;

#[function_component(App)]
fn app() -> Html {
    html! {
         <BrowserRouter>
            <Header />
            <Switch<Route> render={switch} />
         </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}