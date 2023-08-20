use log::error;
use yew::prelude::*;
use yew_plotly::plotly::{Plot, Scatter};
use yew_plotly::Plotly;
use yew_plotly::plotly::common::Mode;

use dydx_api::types::Ohlc;

use crate::utils::api_utils::fetch_single_api_response;

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

#[function_component(OhlcView)]
pub fn ohlc_data_component() -> Html {
    let market = "BTC-USD";
    let resolution = "1MIN";
    let ohlc_data = use_state(|| vec![]);
    {
        let ohlc_data = ohlc_data.clone();
        use_effect_with_deps(
            move |_| {
                let ohlc_data = ohlc_data.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_single_api_response::<Vec<Ohlc>>(
                        format!("/candles/{}/{}", market, resolution)
                            .as_str(),
                    )
                        .await
                    {
                        Ok(fetched_data) => {
                            ohlc_data.set(fetched_data);
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
        <div class="section">
            <div class="container">
                <h1 class="title">{"Main page"}</h1>
                <OhlcList ohlc_data={(*ohlc_data).clone()} />
            </div>
        </div>
    }
}

#[function_component(OhlcChartView)]
pub fn ohlc_chart_component() -> Html {
    let mut plot = Plot::new();
    let x_values = vec![1, 2, 3];
    let y_values = vec![1, 3, 2];

    let trace = Scatter::new(x_values, y_values)
        .mode(Mode::LinesMarkersText)
        .name("Scatter");

    plot.add_trace(trace);

    html! { <Plotly plot={plot}/> }
}