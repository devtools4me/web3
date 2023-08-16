use gloo_net::http::Request;
use log::error;
use yew::prelude::*;

use dydx_api::types::Ohlc;

mod data_source;
pub mod mock;
mod client;

#[derive(Properties, PartialEq)]
struct OhlcListProps {
    ohlc_data: Vec<Ohlc>,
}

#[function_component(OhlcList)]
fn ohlc_list(OhlcListProps { ohlc_data }: &OhlcListProps) -> Html {
    let ohlc_data_html = ohlc_data.iter().map(|x| {
        html! {
            <tr>
                <td>{x.open.clone()}</td>
                <td>{x.high.clone()}</td>
                <td>{x.low.clone()}</td>
                <td>{x.close.clone()}</td>
                <td>{x.timestamp.clone()}</td>
            </tr>
        }
    });
    html! {
        <div>
            <table class="table is-hoverable is-striped">
                <thead>
                    <tr>
                        <th>{"Open"}</th>
                        <th>{"High"}</th>
                        <th>{"Low"}</th>
                        <th>{"Close"}</th>
                        <th>{"Timestamp"}</th>
                    </tr>
                </thead>
                <tbody>
                {for ohlc_data_html}
                </tbody>
            </table>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let ohlc_data = use_state(|| vec![]);
    {
        let ohlc_data = ohlc_data.clone();
        use_effect_with_deps(
            move |_| {
                let ohlc_data = ohlc_data.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = data_source::get_ohlc_data().await;
                    match result {
                        Ok(fetched_data) => {
                            ohlc_data.set(fetched_data);
                        }
                        Err(err) => {
                            error!("{err}")
                        }
                    }
                });
            },
            (),
        );
    }

    html! {
        <div class="section">
            <div class="container">
                <h1 class="title">{"Main page"}</h1>
                <OhlcList ohlc_data={(*ohlc_data).clone()} />
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}