use std::rc::Rc;
use std::str::FromStr;

use log::{error, info};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_plotly::Plotly;
use yewdux::prelude::*;

use algotrader_api::endpoints;
use algotrader_api::types::*;
use algotrader_common::utils::env_utils;

use crate::utils::api_utils::fetch_single_api_response;
use crate::view::plots::indicator_plot;
use crate::view::ohlc::OhlcChartView2;
use crate::view::select::SelectView;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Store)]
pub struct IndicatorStore {
    pub indicators: Vec<String>,
    pub indicator: String,
    pub markets: Vec<String>,
    pub market: String,
    pub resolution: String,
    pub ohlc_data: Vec<Ohlc>,
    pub data: Vec<Indicator>
}

impl Default for IndicatorStore {
    fn default() -> Self {
        Self {
            indicators: env_utils::get_indicators(),
            indicator: env_utils::get_indicator(),
            markets: env_utils::get_markets(),
            market: env_utils::get_market(),
            resolution: env_utils::get_resolution(),
            ohlc_data: vec![],
            data: vec![],
        }
    }
}

pub fn set_indicators(methods: Vec<String>, dispatch: Dispatch<IndicatorStore>) {
    dispatch.reduce_mut(move |store| {
        store.indicators = methods.clone();
    })
}

pub fn set_indicator(method: String, dispatch: Dispatch<IndicatorStore>) {
    dispatch.reduce_mut(move |store| {
        store.indicator = method.clone();
    })
}

pub fn set_markets(markets: Vec<String>, dispatch: Dispatch<IndicatorStore>) {
    dispatch.reduce_mut(move |store| {
        store.markets = markets.clone();
    })
}

pub fn set_market(market: String, dispatch: Dispatch<IndicatorStore>) {
    dispatch.reduce_mut(move |store| {
        store.market = market.clone();
    })
}

pub fn set_data(data: Vec<Indicator>, dispatch: Dispatch<IndicatorStore>) {
    dispatch.reduce_mut(move |store| {
        store.data = data.clone();
    })
}

pub fn set_ohlc_indicator(ohlc_data: Vec<Ohlc>, indicator_data: Vec<Indicator>, dispatch: Dispatch<IndicatorStore>) {
    dispatch.reduce_mut(move |store| {
        store.data = indicator_data.clone();
        store.ohlc_data = ohlc_data.clone();
    })
}

#[derive(Properties, PartialEq)]
pub struct IndicatorDataProps {
    pub market: String,
    pub indicator: String,
    pub data: Vec<Indicator>,
}

#[function_component(IndicatorChartView2)]
pub fn indicator_chart_component(IndicatorDataProps { market, indicator, data }: &IndicatorDataProps) -> Html {
    let indicator_type = IndicatorType::from_str(indicator.as_str()).unwrap();
    let desc = String::from(IndicatorType::description(&indicator_type.clone()));
    let title = format!("{} {}", desc, market);
    let plot = indicator_plot(indicator_type, data.clone());
    html! {
        <div>
            <h1 class="title">{title.as_str()}</h1>
            <Plotly plot={plot}/>
        </div>
    }
}

#[function_component(IndiView)]
pub fn indicator_component() -> Html {
    info!("indicator_component");
    let (store, dispatch) = use_store::<IndicatorStore>();
    fetch_ohlc_indicator(dispatch.clone());

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
    let indicator_callback: Callback<String> = {
        let dispatch = dispatch.clone();
        use_callback(
            move |indicator_value: String, _| {
                info!("indicator_value={}", indicator_value);
                set_indicator(indicator_value.clone(), dispatch.clone());
            },
            (),
        )
    };

    html! {
        <div class="section">
            <div class="container">
                <h1 class="title">{"Methods"}</h1>
                <SelectView values={store.markets.clone()} selected_value={store.market.clone()} callback={mkt_callback}/>
                <SelectView values={store.indicators.clone()} selected_value={store.indicator.clone()} callback={indicator_callback}/>
                <OhlcChartView2 market={store.market.clone()} data={store.ohlc_data.clone()}/>
                <IndicatorChartView2 market={store.market.clone()} indicator={store.indicator.clone()} data={store.data.clone()}/>
            </div>
        </div>
    }
}

fn fetch_ohlc_indicator(dispatch: Dispatch<IndicatorStore>) {
    let store: Rc<IndicatorStore> = dispatch.get();
    let indicator_type: IndicatorType = IndicatorType::from_str(&store.indicator).unwrap();
    let market = store.market.clone();
    let resolution = store.resolution.clone();
    spawn_local(async move {
        let ohlc_endpoint = endpoints::candles( market.as_str(), resolution.as_str());
        let ohlc_response = fetch_single_api_response::<Vec<Ohlc>>(ohlc_endpoint.as_str());
        let indicator_endpoint = endpoints::indicators(&indicator_type, market.as_str(), resolution.as_str());
        let indicatorg_response = fetch_single_api_response::<Vec<Indicator>>(indicator_endpoint.as_str());
        match futures::join!(ohlc_response, indicatorg_response)
        {
            (Ok(ohlc_data), Ok(avg_data)) => {
                set_ohlc_indicator(ohlc_data, avg_data,dispatch);
            }
            (Err(e), _) => {
                error!("{e}")
            }
            (_, Err(e)) => {
                error!("{e}")
            }
        };
    });
}