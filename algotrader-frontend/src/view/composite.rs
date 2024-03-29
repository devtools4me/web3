use yew::prelude::*;
use log::info;
use crate::types::props::OhlcProps;
use crate::view::ohlc::*;
use crate::view::markets::{MarketsPairSelect, MarketsSelect};
use crate::view::average::*;
use crate::view::cointegration::*;
use crate::view::indicator::*;

#[function_component(CointegrationWithMarketView)]
pub fn cointegration_with_market_component(props: &CointegrationProps) -> Html {
    let current_market1 = use_state(|| props.market1.clone());
    let callback1: Callback<String> = {
        let current_market1 = current_market1.clone();
        use_callback(
            move |market_value: String, _| {
                info!("market1_value={}", market_value);
                current_market1.set(market_value.clone());
            },
            (),
        )
    };
    let current_market2 = use_state(|| props.market2.clone());
    let callback2: Callback<String> = {
        let current_market2 = current_market2.clone();
        use_callback(
            move |market_value: String, _| {
                info!("market2_value={}", market_value);
                current_market2.set(market_value.clone());
            },
            (),
        )
    };
    html! {
        <div>
            <MarketsPairSelect callback1={callback1} callback2={callback2}/>
            <CointegrationView market1={(*current_market1).clone()} market2={(*current_market2).clone()} resolution={props.resolution.clone()}  />
        </div>
    }
}

#[function_component(OhlcWithMarketView)]
pub fn ohlc_with_market_component(props: &OhlcProps) -> Html {
    let current_market = use_state(|| props.market.clone());
    let callback: Callback<String> = {
        let current_market = current_market.clone();
        use_callback(
            move |market_value: String, _| {
                info!("market_value={}", market_value);
                current_market.set(market_value.clone());
            },
            (),
        )
    };
    html! {
        <div>
            <MarketsSelect callback={callback} />
            <OhlcView market={(*current_market).clone()} resolution={"1DAY"} />
        </div>
    }
}

#[function_component(OhlcWithAverageChartView)]
pub fn ohlc_with_average_chart_component(props: &AverageChartProps) -> Html {
    let current_market = use_state(|| props.market.clone());
    let callback: Callback<String> = {
        let current_market = current_market.clone();
        use_callback(
            move |market_value: String, _| {
                info!("market_value={}", market_value);
                current_market.set(market_value.clone());
            },
            (),
        )
    };
    html! {
        <div>
            <MarketsSelect callback={callback} />
            <OhlcChartView market={(*current_market).clone()} resolution={"1DAY"} />
            <AverageChartView average_type={props.average_type.clone()} market = {(*current_market).clone()} resolution = {props.resolution.clone()} />
        </div>
    }
}

#[function_component(OhlcWithIndicatorChartView)]
pub fn ohlc_with_indicator_chart_component(props: &IndicatorChartProps) -> Html {
    let current_market = use_state(|| props.market.clone());
    let callback: Callback<String> = {
        let current_market = current_market.clone();
        use_callback(
            move |market_value: String, _| {
                info!("market_value={}", market_value);
                current_market.set(market_value.clone());
            },
            (),
        )
    };
    html! {
        <div>
            <MarketsSelect callback={callback}/>
            <OhlcChartView market={(*current_market).clone()} resolution={"1DAY"} />
            <IndicatorChartView indicator_type={props.indicator_type.clone()} market={(*current_market).clone()} resolution={props.resolution.clone()} />
        </div>
    }
}