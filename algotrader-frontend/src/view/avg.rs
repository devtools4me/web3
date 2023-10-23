use std::rc::Rc;
use std::str::FromStr;

use log::{error, info};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_plotly::Plotly;
use yew_plotly::plotly::color::NamedColor;
use yewdux::prelude::*;

use algotrader_api::endpoints;
use algotrader_api::types::*;
use algotrader_common::utils::env_utils;
use algotrader_common::utils::vec_utils::convert;

use crate::utils::api_utils::fetch_single_api_response;
use crate::utils::ui_utils::plot_with_scatter;
use crate::view::select::SelectView;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct AverageStore {
    pub methods: Vec<String>,
    pub method: String,
    pub markets: Vec<String>,
    pub market: String,
    pub resolution: String,
    pub data: Vec<Timeseries>
}

impl Default for AverageStore {
    fn default() -> Self {
        Self {
            methods: env_utils::get_methods(),
            method: env_utils::get_method(),
            markets: env_utils::get_markets(),
            market: env_utils::get_market(),
            resolution: env_utils::get_resolution(),
            data: vec![],
        }
    }
}

pub fn set_methods(methods: Vec<String>, dispatch: Dispatch<AverageStore>) {
    dispatch.reduce_mut(move |store| {
        store.methods = methods.clone();
    })
}

pub fn set_method(method: String, dispatch: Dispatch<AverageStore>) {
    dispatch.reduce_mut(move |store| {
        store.method = method.clone();
    })
}

pub fn set_markets(markets: Vec<String>, dispatch: Dispatch<AverageStore>) {
    dispatch.reduce_mut(move |store| {
        store.markets = markets.clone();
    })
}

pub fn set_market(market: String, dispatch: Dispatch<AverageStore>) {
    dispatch.reduce_mut(move |store| {
        store.market = market.clone();
    })
}

pub fn set_data(data: Vec<Timeseries>, dispatch: Dispatch<AverageStore>) {
    dispatch.reduce_mut(move |store| {
        store.data = data.clone();
    })
}

#[derive(Properties, PartialEq)]
pub struct AvgDataProps {
    pub market: String,
    pub method: String,
    pub data: Vec<Timeseries>,
}

#[function_component(AvgChartView)]
pub fn avg_chart_component(AvgDataProps { market, method, data }: &AvgDataProps) -> Html {
    let title = format!("{} {}", method, market);
    let avg_plot = plot_with_scatter(
        convert(data.clone(), |x| x.timestamp),
        convert(data.clone(), |x| {
            match x.value.parse::<f64>() {
                Ok(v) => v,
                Err(_) => f64::NAN
            }
        }),
        method.to_owned(),
        NamedColor::Blue);
    html! {
        <div>
            <h1 class="title">{title.as_str()}</h1>
            <Plotly plot={avg_plot}/>
        </div>
    }
}

#[function_component(AvgView)]
pub fn avg_component() -> Html {
    info!("avg_component");
    let (store, dispatch) = use_store::<AverageStore>();
    fetch_avg(dispatch.clone());

    let mkt_callback: Callback<String> = {
        let dispatch = dispatch.clone();
        use_callback(
            move |market_value: String, _| {
                info!("market_value={}", market_value);
                set_market(market_value.clone(), dispatch.clone());
            },
            (),
        )
    };
    let method_callback: Callback<String> = {
        let dispatch = dispatch.clone();
        use_callback(
            move |method_value: String, _| {
                info!("method_value={}", method_value);
                set_method(method_value.clone(), dispatch.clone());
            },
            (),
        )
    };

    html! {
        <div class="section">
            <div class="container">
                <h1 class="title">{"Cointegration"}</h1>
                <SelectView values={store.markets.clone()} selected_value={store.market.clone()} callback={mkt_callback}/>
                <SelectView values={store.methods.clone()} selected_value={store.method.clone()} callback={method_callback}/>
                <AvgChartView market={store.market.clone()} method={store.method.clone()} data={store.data.clone()}/>
            </div>
        </div>
    }
}

fn fetch_avg(dispatch: Dispatch<AverageStore>) {
    let store: Rc<AverageStore> = dispatch.get();
    let method = AverageType::from_str(&store.method).unwrap();
    let market = store.market.clone();
    let resolution = store.resolution.clone();
    spawn_local(async move {
        let endpoint = endpoints::methods(&method, market.as_str(), resolution.as_str());
        match fetch_single_api_response::<Vec<Timeseries>>(endpoint.as_str())
            .await
        {
            Ok(fetched_data) => {
                info!("fetched_data={:?}", fetched_data);
                set_data(fetched_data, dispatch);
            }
            Err(e) => {
                error!("{e}")
            }
        };
    });
}