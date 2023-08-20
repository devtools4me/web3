use log::error;
use yew::prelude::*;
use yew_plotly::plotly::{Plot, Scatter};
use yew_plotly::{Plotly, plotly};
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
    let x = vec![
        "2017-01-04",
        "2017-01-05",
        "2017-01-06",
        "2017-01-09",
        "2017-01-10",
        "2017-01-11",
        "2017-01-12",
        "2017-01-13",
        "2017-01-17",
        "2017-01-18",
        "2017-01-19",
        "2017-01-20",
        "2017-01-23",
        "2017-01-24",
        "2017-01-25",
        "2017-01-26",
        "2017-01-27",
        "2017-01-30",
        "2017-01-31",
        "2017-02-01",
        "2017-02-02",
        "2017-02-03",
        "2017-02-06",
        "2017-02-07",
        "2017-02-08",
        "2017-02-09",
        "2017-02-10",
        "2017-02-13",
        "2017-02-14",
        "2017-02-15",
    ];
    let open = vec![
        115.849998, 115.919998, 116.779999, 117.949997, 118.769997, 118.739998, 118.900002,
        119.110001, 118.339996, 120.0, 119.400002, 120.449997, 120.0, 119.550003, 120.419998,
        121.669998, 122.139999, 120.93, 121.150002, 127.029999, 127.980003, 128.309998, 129.130005,
        130.539993, 131.350006, 131.649994, 132.460007, 133.080002, 133.470001, 135.520004,
    ];
    let high = vec![
        116.510002, 116.860001, 118.160004, 119.43, 119.379997, 119.93, 119.300003, 119.620003,
        120.239998, 120.5, 120.089996, 120.449997, 120.809998, 120.099998, 122.099998, 122.440002,
        122.349998, 121.629997, 121.389999, 130.490005, 129.389999, 129.190002, 130.5, 132.089996,
        132.220001, 132.449997, 132.940002, 133.820007, 135.089996, 136.270004,
    ];
    let low = vec![
        115.75, 115.809998, 116.470001, 117.940002, 118.300003, 118.599998, 118.209999, 118.809998,
        118.220001, 119.709999, 119.370003, 119.730003, 119.769997, 119.5, 120.279999, 121.599998,
        121.599998, 120.660004, 120.620003, 127.010002, 127.779999, 128.160004, 128.899994,
        130.449997, 131.220001, 131.119995, 132.050003, 132.75, 133.25, 134.619995,
    ];
    let close = vec![
        116.019997, 116.610001, 117.910004, 118.989998, 119.110001, 119.75, 119.25, 119.040001,
        120.0, 119.989998, 119.779999, 120.0, 120.080002, 119.970001, 121.879997, 121.940002,
        121.949997, 121.629997, 121.349998, 128.75, 128.529999, 129.080002, 130.289993, 131.529999,
        132.039993, 132.419998, 132.119995, 133.289993, 135.020004, 135.509995,
    ];

    let trace1 = plotly::Ohlc::new(x, open, high, low, close);

    let mut plot = Plot::new();
    plot.add_trace(trace1);

    html! { <Plotly plot={plot}/> }
}