use std::str::FromStr;

use strum_macros::{Display, EnumIter, EnumString};
use yew::prelude::*;
use yew_router::prelude::*;

use algotrader_api::types::*;
use algotrader_common::utils::env_utils;
use crate::view::avg::AvgView;
use crate::view::indi::IndiView;

use crate::view::composite::*;
use crate::view::coint::CointegrationView;
use crate::view::ohlc::OhlcChartView;
use crate::view::struct_markets::StructMarkets;

#[derive(Clone, Routable, PartialEq, Display, EnumString, EnumIter)]
pub enum Route {
    #[at("/about")]
    About,
    #[at("/ohlc")]
    Ohlc,
    #[at("/cointegration")]
    Cointegration,
    #[at("/spread")]
    Spread,
    #[at("/trends")]
    Trends,
    // Methods
    #[at("/methods")]
    Methods,
    #[at("/ema")]
    EMA,
    #[at("/hma")]
    HMA,
    #[at("/dema")]
    DEMA,
    #[at("/dma")]
    DMA,
    #[at("/momentum")]
    Momentum,
    #[at("/rma")]
    RMA,
    #[at("/sma")]
    SMA,
    #[at("/swma")]
    SWMA,
    #[at("/tema")]
    TEMA,
    #[at("/tma")]
    TMA,
    #[at("/trima")]
    TRIMA,
    #[at("/vwma")]
    VWMA,
    #[at("/vidya")]
    Vidya,
    #[at("/wma")]
    WMA,
    #[at("/wsma")]
    WSMA,

    // Indicators
    #[at("/indicators")]
    Indicators,
    #[at("/macd")]
    MACD,
    #[at("/rsi")]
    RSI,
    #[at("/run_together")]
    RunTogether,
    #[at("/sell_volatility")]
    SellVolatility,

    //
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub fn from_average_type(t: &AverageType) -> Route {
        Route::from_str(t.to_string().as_str()).unwrap()
    }

    pub fn from_indicator_type(t: &IndicatorType) -> Route {
        Route::from_str(t.to_string().as_str()).unwrap()
    }
}

pub fn switch(route: Route) -> Html {
    let market = env_utils::get_market();
    let market2 = env_utils::get_market2();
    let resolution = env_utils::get_resolution();
    let markets = env_utils::get_markets();
    match route {
        Route::Home => html! { <OhlcWithMarketView market={market} resolution={resolution} /> },
        Route::Ohlc => html! { <OhlcChartView market={market} resolution={resolution}  /> },
        Route::Cointegration => html! { <CointegrationView /> },
        Route::Spread => html! { <CointegrationWithMarketView market1={market} market2={market2} resolution={resolution}  /> },
        //Route::Trends => html! { <CointegrationWithMarketView market1={market} market2={market2} resolution={resolution}  /> },
        Route::Trends => html! { <StructMarkets markets={markets} selected_market={market2}/> },
        //Methods
        Route::Methods => html! { <AvgView /> },
        //Indicators
        Route::Indicators => html! { <IndiView /> },
        // Other
        Route::About => html! { <p class="text-white">{ "Not found" }</p> },
        Route::NotFound => html! { <p class="text-white">{ "Not found" }</p> },
        _ => html! { <p class="text-white">{ "Not found" }</p> },
    }
}