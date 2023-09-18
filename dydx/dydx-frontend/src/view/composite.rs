use yew::prelude::*;
use crate::view::ohlc::OhlcChartView;
use crate::view::average::*;
use crate::view::indicator::*;

#[function_component(OhlcWithAverageChartView)]
pub fn ohlc_with_average_chart_component(AverageChartProps { average_type }: &AverageChartProps) -> Html {
    html! {
        <div>
            <OhlcChartView />
            <AverageChartView average_type={average_type.clone()} />
        </div>
    }
}

#[function_component(OhlcWithIndicatorChartView)]
pub fn ohlc_with_indicator_chart_component(IndicatorChartProps { indicator_type }: &IndicatorChartProps) -> Html {
    html! {
        <div>
            <OhlcChartView />
            <IndicatorChartView indicator_type={indicator_type.clone()} />
        </div>
    }
}