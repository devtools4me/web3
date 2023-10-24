use std::ops::Deref;
use log::error;
use yew::prelude::*;
use yew::prelude::*;
use yew_plotly::{Plotly, plotly};
use yew_plotly::plotly::Plot;

use algotrader_api::endpoints;
use algotrader_api::types::*;
use algotrader_api::types::*;
use crate::types::props::{OhlcListProps, OhlcProps};

use crate::utils::api_utils::fetch_single_api_response;

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
pub fn ohlc_data_component(props: &OhlcProps) -> Html {
    let endpoint = endpoints::candles(props.market.as_str(), props.resolution.as_str());
    let title = format!("OHLC {} {}", props.market, props.resolution);
    let ohlc_data = use_state(|| vec![]);
    {
        let ohlc_data = ohlc_data.clone();
        use_effect_with_deps(
            move |_| {
                let ohlc_data = ohlc_data.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_single_api_response::<Vec<Ohlc>>(endpoint.as_str())
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
                <h1 class="title">{title.as_str()}</h1>
                <OhlcList ohlc_data={(*ohlc_data).clone()} />
            </div>
        </div>
    }
}

#[function_component(OhlcChartView)]
pub fn ohlc_chart_component(props: &OhlcProps) -> Html {
    let endpoint = endpoints::candles(props.market.as_str(), props.resolution.as_str());
    let title = format!("OHLC {} {}", props.market, props.resolution);
    let state = use_state(|| Plot::new());
    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_single_api_response::<Vec<Ohlc>>(endpoint.as_str())
                        .await
                    {
                        Ok(fetched_data) => {
                            let plot = ohlc_plot(fetched_data);
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
                <Plotly plot={state.deref().clone()}/>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct OhlcDataProps {
    pub market: String,
    pub data: Vec<Ohlc>,
}

#[function_component(OhlcChartView2)]
pub fn ohlc_chart_component_2(OhlcDataProps { market, data }: &OhlcDataProps) -> Html {
    let title = format!("OHLC {}", market);
    let ohlc_plot = ohlc_plot(data.clone());
    html! {
        <div>
            <h1 class="title">{title.as_str()}</h1>
            <Plotly plot={ohlc_plot}/>
        </div>
    }
}

fn ohlc_plot(fetched_data: Vec<Ohlc>) -> Plot {
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
    plot
}