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
use crate::view::markets::MarketsDatalist;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct CointStore {
    pub markets: Vec<String>,
    pub market1: String,
    pub market2: String,
    pub resolution: String,
    pub cointegration_data: CointegrationData,
    pub spread_zscore_data: SpreadZScoreData
}

impl Default for CointStore {
    fn default() -> Self {
        Self {
            markets: env_utils::get_markets(),
            market1: env_utils::get_market(),
            market2: env_utils::get_market2(),
            resolution: env_utils::get_resolution(),
            cointegration_data: CointegrationData::default(),
            spread_zscore_data: SpreadZScoreData::default(),
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

pub fn set_cointegration_data(data: CointegrationData, dispatch: Dispatch<CointStore>) {
    dispatch.reduce_mut(move |store| {
        store.cointegration_data = data.clone();
    })
}

pub fn set_spread_zscore_data(data: SpreadZScoreData, dispatch: Dispatch<CointStore>) {
    dispatch.reduce_mut(move |store| {
        store.spread_zscore_data = data.clone();
    })
}

pub fn set_trends_data(data: TrendsData, dispatch: Dispatch<CointStore>) {
    dispatch.reduce_mut(move |store| {
        store.cointegration_data = data.cointegration_data.clone();
        store.spread_zscore_data = data.spread_zscore_data.clone();
    })
}

#[function_component(CointegrationView)]
pub fn cointegration_component() -> Html {
    info!("cointegration_component");
    let (store, dispatch) = use_store::<CointStore>();
    fetch_cointegration_trends(dispatch.clone());

    let callback1: Callback<String> = {
        let dispatch = dispatch.clone();
        use_callback(
            move |market_value: String, _| {
                info!("market1_value={}", market_value);
                set_market1(market_value.clone(), dispatch.clone());
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
            },
            (),
        )
    };

    html! {
        <div class="section">
            <div class="container">
                <h1 class="title">{"Cointegration"}</h1>
                <MarketsDatalist markets={store.markets.clone()} selected_market={store.market1.clone()} callback={callback1}/>
                <MarketsDatalist markets={store.markets.clone()} selected_market={store.market2.clone()} callback={callback2}/>
                <CointegrationDataView data={store.cointegration_data.clone()}/>
                <SpreadZScoreDataView market1={store.market1.clone()} market2={store.market2.clone()} data={store.spread_zscore_data.clone()}/>
            </div>
        </div>
    }
}

fn fetch_cointegration(dispatch: Dispatch<CointStore>) {
    let store: Rc<CointStore> = dispatch.get();
    let market1 = store.market1.clone();
    let market2 = store.market2.clone();
    let resolution = store.resolution.clone();
    spawn_local(async move {
        let endpoint = endpoints::cointegration(market1.as_str(), market2.as_str(), resolution.as_str());
        match fetch_single_api_response::<CointegrationData>(endpoint.as_str())
            .await
        {
            Ok(fetched_data) => {
                info!("fetched_data={:?}", fetched_data);
                set_cointegration_data(fetched_data, dispatch);
            }
            Err(e) => {
                error!("{e}")
            }
        };
    });
}

fn fetch_cointegration_trends(dispatch: Dispatch<CointStore>) {
    let store: Rc<CointStore> = dispatch.get();
    let market1 = store.market1.clone();
    let market2 = store.market2.clone();
    let resolution = store.resolution.clone();
    spawn_local(async move {
        let endpoint = endpoints::cointegration_trends(market1.as_str(), market2.as_str(), resolution.as_str());
        match fetch_single_api_response::<TrendsData>(endpoint.as_str())
            .await
        {
            Ok(fetched_data) => {
                info!("fetched_data={:?}", fetched_data);
                set_trends_data(fetched_data, dispatch);
            }
            Err(e) => {
                error!("{e}")
            }
        };
    });
}