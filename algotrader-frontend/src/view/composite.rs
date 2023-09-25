use yew::prelude::*;
use log::info;
use crate::view::ohlc::OhlcChartView;
use crate::view::markets::MarketsSelect;
use crate::view::average::*;
use crate::view::indicator::*;

#[function_component(OhlcWithAverageChartView)]
pub fn ohlc_with_average_chart_component(AverageChartProps { average_type, market, resolution }: &AverageChartProps) -> Html {
    let callback: Callback<String> = {
        use_callback(
            move |market_value, _| {
                info!("market_value={}", market_value);
            },
            (),
        )
    };
    html! {
        <div>
            <MarketsSelect callback={callback}/>
            <OhlcChartView />
            <AverageChartView average_type={average_type.clone()} market = {market.clone()} resolution = {resolution.clone()} />
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
            <OhlcChartView />
            <IndicatorChartView indicator_type={props.indicator_type.clone()} market={(*current_market).clone()} resolution={props.resolution.clone()} />
        </div>
    }
}