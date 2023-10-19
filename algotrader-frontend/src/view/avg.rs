use std::rc::Rc;

use log::{error, info};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux::prelude::use_store;

use algotrader_api::endpoints;
use algotrader_api::types::*;
use algotrader_common::utils::env_utils;

use crate::utils::api_utils::fetch_single_api_response;
use crate::view::cointegration::CointegrationDataView;
use crate::view::trend::SpreadZScoreDataView;
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

#[function_component(AvgView)]
pub fn avg_component() -> Html {
    info!("avg_component");
    let (store, dispatch) = use_store::<AverageStore>();
    //fetch_cointegration_trends(dispatch.clone());

    let callback1: Callback<String> = {
        let dispatch = dispatch.clone();
        use_callback(
            move |market_value: String, _| {
                info!("market_value={}", market_value);
                set_market(market_value.clone(), dispatch.clone());
            },
            (),
        )
    };
    let callback2: Callback<String> = {
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
                <SelectView values={store.markets.clone()} selected_value={store.market.clone()} callback={callback1}/>
                <SelectView values={store.methods.clone()} selected_value={store.method.clone()} callback={callback2}/>
            </div>
        </div>
    }
}

// fn fetch_cointegration(dispatch: Dispatch<AverageStore>) {
//     let store: Rc<AverageStore> = dispatch.get();
//     let market = store.market.clone();
//     let method = store.method.clone();
//     let resolution = store.resolution.clone();
//     spawn_local(async move {
//         let endpoint = endpoints::cointegration(market1.as_str(), market2.as_str(), resolution.as_str());
//         match fetch_single_api_response::<CointegrationData>(endpoint.as_str())
//             .await
//         {
//             Ok(fetched_data) => {
//                 info!("fetched_data={:?}", fetched_data);
//                 set_cointegration_data(fetched_data, dispatch);
//             }
//             Err(e) => {
//                 error!("{e}")
//             }
//         };
//     });
// }