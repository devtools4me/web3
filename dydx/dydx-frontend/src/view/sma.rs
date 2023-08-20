use log::error;
use yew::prelude::*;

use yew_plotly::plotly::common::Mode;
use yew_plotly::plotly::{Plot, Scatter};
use yew_plotly::Plotly;

#[function_component(SmaChartView)]
pub fn sma_chart_component() -> Html {
    let mut plot = Plot::new();
    let x_values = vec![1, 2, 3];
    let y_values = vec![1, 3, 2];

    let trace = Scatter::new(x_values, y_values)
        .mode(Mode::LinesMarkersText)
        .name("Scatter");

    plot.add_trace(trace);

    html! { <Plotly plot={plot}/> }
}