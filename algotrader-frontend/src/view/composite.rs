use yew::prelude::*;
use crate::view::ohlc::OhlcChartView;
use crate::view::markets::MarketsSelect;
use crate::view::average::*;
use crate::view::indicator::*;

#[function_component(OhlcWithAverageChartView)]
pub fn ohlc_with_average_chart_component(AverageChartProps { average_type, market, resolution }: &AverageChartProps) -> Html {
    html! {
        <div>
            <MarketsSelect />
            <OhlcChartView />
            <AverageChartView average_type={average_type.clone()} market = {market.clone()} resolution = {resolution.clone()} />
        </div>
    }
}

#[function_component(OhlcWithIndicatorChartView)]
pub fn ohlc_with_indicator_chart_component(IndicatorChartProps { indicator_type, market, resolution }: &IndicatorChartProps) -> Html {
    html! {
        <div>
            <MarketsSelect />
            <OhlcChartView />
            <IndicatorChartView indicator_type={indicator_type.clone()} market={market.clone()} resolution={resolution.clone()} />
        </div>
    }
}