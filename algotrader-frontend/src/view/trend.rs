use std::str::FromStr;

use plotly::{Plot, Scatter};
use yew::prelude::*;
use yew_plotly::{Plotly, plotly};
use yew_plotly::plotly::color::NamedColor;

use algotrader_api::types::*;

#[derive(Properties, PartialEq)]
pub struct SpreadZScoreDataProps {
    pub data: SpreadZScoreData
}

#[function_component(SpreadZScoreDataView)]
pub fn spread_zscore_component(SpreadZScoreDataProps { data }: &SpreadZScoreDataProps) -> Html {
    let SpreadZScoreData { spread, z_score, timestamp } = data;
    let percentage = percentage_plot(timestamp.clone(), spread.clone());
    html! {
        <div>
            <Plotly plot={percentage}/>
        </div>
    }
}

fn percentage_plot(timestamp: Vec<String>, data: Vec<String>) -> Plot {
    let macd_value = data.iter()
        .map(|x| x.parse::<f64>().unwrap())
        .collect();
    let trace = Scatter::new(timestamp.clone(), macd_value).name("Signal Line")
        .line(plotly::common::Line::new().color(NamedColor::LightGreen));
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot
}