use log::error;
use yew::prelude::*;
use yew_plotly::plotly::{Plot, Scatter};
use yew_plotly::{Plotly, plotly};
use yew_plotly::plotly::common::Mode;

use algotrader_api::types::*;
use algotrader_api::path::*;

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
    let resolution = "1DAY";
    let ohlc_data = use_state(|| vec![]);
    {
        let ohlc_data = ohlc_data.clone();
        use_effect_with_deps(
            move |_| {
                let ohlc_data = ohlc_data.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_single_api_response::<Vec<Ohlc>>(
                        candles(market, resolution).as_str()
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
    let market = "BTC-USD";
    let resolution = "1DAY";
    let title = format!("OHLC {} {}", market, resolution);
    let state = use_state(|| Plot::new());
    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_single_api_response::<Vec<Ohlc>>(
                        candles(market, resolution).as_str(),
                    )
                        .await
                    {
                        Ok(fetched_data) => {
                            let x = fetched_data.iter()
                                .map(|x| x.timestamp.clone())
                                .collect::<Vec<_>>();
                            let open = fetched_data.iter()
                                .map(|x| x.open.parse::<f64>().unwrap())
                                .collect::<Vec<_>>();
                            let high = fetched_data.iter()
                                .map(|x| x.high.parse::<f64>().unwrap())
                                .collect::<Vec<_>>();
                            let low = fetched_data.iter()
                                .map(|x| x.low.parse::<f64>().unwrap())
                                .collect::<Vec<_>>();
                            let close = fetched_data.iter()
                                .map(|x| x.close.parse::<f64>().unwrap())
                                .collect::<Vec<_>>();
                            let trace = plotly::Ohlc::new(x, open, high, low, close);
                            let mut plot = Plot::new();
                            plot.add_trace(trace);
                            state.set(plot);
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
                <h1 class="title">{title.as_str()}</h1>
                <Plotly plot={(*state).clone()}/>
            </div>
        </div>
    }
}