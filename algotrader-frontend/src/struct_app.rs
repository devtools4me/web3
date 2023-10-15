use std::rc::Rc;

use log::{error, info};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux::prelude::use_store;

use algotrader_api::endpoints;
use algotrader_api::types::CointegrationData;
use algotrader_common::utils::env_utils;

use crate::utils::api_utils::fetch_single_api_response;
use crate::view::cointegration::CointegrationDataView;
use crate::view::markets::MarketsDatalist;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct CointStore {
    pub markets: Vec<String>,
    pub market1: String,
    pub market2: String,
    pub resolution: String,
    pub data: CointegrationData,
}

impl Default for CointStore {
    fn default() -> Self {
        Self {
            markets: env_utils::get_markets(),
            market1: env_utils::get_market(),
            market2: env_utils::get_market2(),
            resolution: env_utils::get_resolution(),
            data: Default::default(),
        }
    }
}

pub fn set_markets(markets: Vec<String>, dispatch: Dispatch<CointStore>) {
    dispatch.reduce_mut(move |store| {
        store.markets = markets.clone();
    })
}

pub fn set_market1(market1: String, dispatch: Dispatch<CointStore>) {
    dispatch.reduce_mut(move |store| {
        store.market1 = market1.clone();
    })
}

pub fn set_market2(market2: String, dispatch: Dispatch<CointStore>) {
    dispatch.reduce_mut(move |store| {
        store.market2 = market2.clone();
    })
}

pub fn set_data(data: CointegrationData, dispatch: Dispatch<CointStore>) {
    dispatch.reduce_mut(move |store| {
        store.data = data.clone();
    })
}

#[function_component(App)]
pub fn app() -> Html {
    let (store, dispatch) = use_store::<CointStore>();
    //fetch_markets(dispatch.clone());
    fetch_coint_with_dispatch(dispatch.clone());

    let callback1: Callback<String> = {
        let dispatch = dispatch.clone();
        use_callback(
            move |market_value: String, _| {
                info!("market1_value={}", market_value);
                set_market1(market_value.clone(), dispatch.clone());
                fetch_coint_with_dispatch(dispatch.clone());
            },
            (),
        )
    };
    let callback2: Callback<String> = {
        let dispatch = dispatch.clone();
        use_callback(
            move |market_value: String, _| {
                info!("market2_value={}", market_value);
                set_market2(market_value.clone(), dispatch.clone());
                fetch_coint_with_dispatch(dispatch.clone());
            },
            (),
        )
    };

    html! {
        <div class="section">
            <div class="container">
                <h1 class="title">{"Markets"}</h1>
                <MarketsDatalist markets={store.markets.clone()} selected_market={store.market1.clone()} callback={callback1}/>
                <MarketsDatalist markets={store.markets.clone()} selected_market={store.market2.clone()} callback={callback2}/>
            </div>
            <CointegrationDataView data={store.data.clone()}/>
        </div>
    }
}

fn fetch_coint_with_dispatch(dispatch: Dispatch<CointStore>) {
    let store: Rc<CointStore> = dispatch.get();
    fetch_coint(store.market1.clone(), store.market2.clone(), store.resolution.clone(), dispatch);
}

fn fetch_coint(market1: String, market2: String, resolution: String, dispatch: Dispatch<CointStore>) {
    spawn_local(async move {
        let endpoint = endpoints::cointegration(market1.as_str(), market2.as_str(), resolution.as_str());
        match fetch_single_api_response::<CointegrationData>(endpoint.as_str())
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

fn fetch_markets(dispatch: Dispatch<CointStore>) {
    let endpoint = endpoints::markets();
    spawn_local(async move {
        match fetch_single_api_response::<Vec<String>>(endpoint.as_str())
            .await
        {
            Ok(fetched_data) => {
                info!("fetched_data={:?}", fetched_data);
                set_markets(fetched_data, dispatch);
            }
            Err(e) => {
                error!("{e}")
            }
        };
    });
}