use yew::prelude::*;

use dydx_api::types::Ohlc;

mod data_source;
pub mod mock;
mod gloonet;

struct App;

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

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ohlc_data = data_source::get_ohlc_data().unwrap();
        html! {
            <div class="section">
                <div class="container">
                    <h1 class="title">{"Main page"}</h1>
                    <OhlcList ohlc_data={ohlc_data} />
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}