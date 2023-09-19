use yew::prelude::*;
use yew_router::prelude::*;

use crate::view::{ohlc::OhlcView, ohlc::OhlcChartView, average::AverageChartView, indicator::IndicatorChartView};
use algotrader_api::types::*;
use crate::view::composite::{OhlcWithAverageChartView, OhlcWithIndicatorChartView};

#[derive(Clone, Routable, PartialEq)]
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

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <OhlcView /> },
        Route::Ohlc => html! { <OhlcChartView /> },
        //Methods
        Route::EMA => html! { <OhlcWithAverageChartView average_type={AverageType::EMA} /> },
        Route::HMA => html! { <OhlcWithAverageChartView average_type={AverageType::HMA} /> },
        Route::DEMA => html! { <OhlcWithAverageChartView average_type={AverageType::DEMA} /> },
        Route::DMA => html! { <OhlcWithAverageChartView average_type={AverageType::DMA} /> },
        Route::Momentum => html! { <OhlcWithAverageChartView average_type={AverageType::Momentum} /> },
        Route::RMA => html! { <OhlcWithAverageChartView average_type={AverageType::RMA} /> },
        Route::SMA => html! { <OhlcWithAverageChartView average_type={AverageType::SMA} /> },
        Route::SWMA => html! { <OhlcWithAverageChartView average_type={AverageType::SWMA} /> },
        Route::TEMA => html! { <OhlcWithAverageChartView average_type={AverageType::TEMA} /> },
        Route::TMA => html! { <OhlcWithAverageChartView average_type={AverageType::TMA} /> },
        Route::TRIMA => html! { <OhlcWithAverageChartView average_type={AverageType::TRIMA} /> },
        Route::VWMA => html! { <OhlcWithAverageChartView average_type={AverageType::VWMA} /> },
        Route::Vidya => html! { <OhlcWithAverageChartView average_type={AverageType::Vidya} /> },
        Route::WMA => html! { <OhlcWithAverageChartView average_type={AverageType::WMA} /> },
        Route::WSMA => html! { <OhlcWithAverageChartView average_type={AverageType::WSMA} /> },
        //Indicators
        Route::MACD => html! { <OhlcWithIndicatorChartView indicator_type={IndicatorType::MACD} /> },
        Route::RSI => html! { <OhlcWithIndicatorChartView indicator_type={IndicatorType::RSI} /> },
        Route::RunTogether => html! { <OhlcWithIndicatorChartView indicator_type={IndicatorType::RunTogether} /> },
        Route::SellVolatility => html! { <OhlcWithIndicatorChartView indicator_type={IndicatorType::SellVolatility} /> },
        // Other
        Route::About => html! { <p class="text-white">{ "Not found" }</p> },
        Route::NotFound => html! { <p class="text-white">{ "Not found" }</p> },
    }
}