use yew::prelude::*;
use yew_router::prelude::*;
use strum_macros::{Display, EnumIter, EnumString};
use std::str::FromStr;

use crate::view::{ohlc::OhlcView, ohlc::OhlcChartView};
use algotrader_api::types::*;
use algotrader_common::utils::env_utils;
use crate::view::composite::{OhlcWithAverageChartView, OhlcWithIndicatorChartView};

#[derive(Clone, Routable, PartialEq, Display, EnumString, EnumIter)]
pub enum Route {
    #[at("/about")]
    About,
    #[at("/ohlc")]
    Ohlc,

    // Methods
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
    let resolution = env_utils::get_resolution();
    match route {
        Route::Home => html! { <OhlcView market={"BTC-USD"} resolution={"1DAY"} /> },
        Route::Ohlc => html! { <OhlcChartView market={"BTC-USD"} resolution={"1DAY"}  /> },
        //Methods
        Route::EMA => html! { <OhlcWithAverageChartView average_type={AverageType::EMA} market={market} resolution={resolution} /> },
        Route::HMA => html! { <OhlcWithAverageChartView average_type={AverageType::HMA} market={market} resolution={resolution} /> },
        Route::DEMA => html! { <OhlcWithAverageChartView average_type={AverageType::DEMA} market={market} resolution={resolution} /> },
        Route::DMA => html! { <OhlcWithAverageChartView average_type={AverageType::DMA} market={market} resolution={resolution} /> },
        Route::Momentum => html! { <OhlcWithAverageChartView average_type={AverageType::Momentum} market={market} resolution={resolution} /> },
        Route::RMA => html! { <OhlcWithAverageChartView average_type={AverageType::RMA} market={market} resolution={resolution} /> },
        Route::SMA => html! { <OhlcWithAverageChartView average_type={AverageType::SMA} market={market} resolution={resolution} /> },
        Route::SWMA => html! { <OhlcWithAverageChartView average_type={AverageType::SWMA} market={market} resolution={resolution} /> },
        Route::TEMA => html! { <OhlcWithAverageChartView average_type={AverageType::TEMA} market={market} resolution={resolution} /> },
        Route::TMA => html! { <OhlcWithAverageChartView average_type={AverageType::TMA} market={market} resolution={resolution} /> },
        Route::TRIMA => html! { <OhlcWithAverageChartView average_type={AverageType::TRIMA} market={market} resolution={resolution} /> },
        Route::VWMA => html! { <OhlcWithAverageChartView average_type={AverageType::VWMA} market={market} resolution={resolution} /> },
        Route::Vidya => html! { <OhlcWithAverageChartView average_type={AverageType::Vidya} market={market} resolution={resolution} /> },
        Route::WMA => html! { <OhlcWithAverageChartView average_type={AverageType::WMA} market={market} resolution={resolution} /> },
        Route::WSMA => html! { <OhlcWithAverageChartView average_type={AverageType::WSMA} market={market} resolution={resolution} /> },
        //Indicators
        Route::MACD => html! { <OhlcWithIndicatorChartView indicator_type={IndicatorType::MACD} market={market} resolution={resolution} /> },
        Route::RSI => html! { <OhlcWithIndicatorChartView indicator_type={IndicatorType::RSI} market={market} resolution={resolution} /> },
        Route::RunTogether => html! { <OhlcWithIndicatorChartView indicator_type={IndicatorType::RunTogether} market={market} resolution={resolution} /> },
        Route::SellVolatility => html! { <OhlcWithIndicatorChartView indicator_type={IndicatorType::SellVolatility} market={market} resolution={resolution} /> },
        // Other
        Route::About => html! { <p class="text-white">{ "Not found" }</p> },
        Route::NotFound => html! { <p class="text-white">{ "Not found" }</p> },
    }
}