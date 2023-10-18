use std::str::FromStr;

use plotly::{Plot, Scatter};
use yew::prelude::*;
use yew_plotly::{Plotly, plotly};
use yew_plotly::plotly::color::NamedColor;

use algotrader_api::types::*;
use algotrader_common::utils::vec_utils::convert;

#[derive(Properties, PartialEq)]
pub struct SpreadZScoreDataProps {
    pub market1: String,
    pub market2: String,
    pub data: SpreadZScoreData
}

#[function_component(SpreadZScoreDataView)]
pub fn spread_zscore_component(SpreadZScoreDataProps { market1, market2, data }: &SpreadZScoreDataProps) -> Html {
    let SpreadZScoreData { spread, z_score, timestamp } = data;
    let spread_title = format!("Spread {} {}", market1, market2);
    let spread_plot = plot_with_scatter(
        timestamp.clone(),
        convert(spread.clone(), |x| x.parse::<f64>().unwrap()),
        "Spread".to_owned(),
        NamedColor::LightGreen);
    let zscore_title = format!("Z-Score {} {}", market1, market2);
    let zscore_plot = plot_with_scatter(
        timestamp.clone(),
        convert(z_score.clone(), |x| {
            match x.parse::<f64>() {
                Ok(v) => v,
                Err(_) => f64::NAN
            }
        }),
        "ZScore".to_owned(),
        NamedColor::Blue);
    html! {
        <div>
            <h1 class="title">{spread_title.as_str()}</h1>
            <Plotly plot={spread_plot}/>
            <h1 class="title">{zscore_title.as_str()}</h1>
            <Plotly plot={zscore_plot}/>
        </div>
    }
}

fn plot_with_scatter(x: Vec<String>, y: Vec<f64>, name: String, color: NamedColor) -> Plot {
    let trace = Scatter::new(x.clone(), y)
        .name(name.as_str())
        .line(plotly::common::Line::new().color(color));
    scatters_to_plot(vec![trace])
}

fn scatters_to_plot(traces: Vec<Box<Scatter<String, f64>>>) -> Plot {
    let mut plot = Plot::new();
    for t in traces.iter() {
        plot.add_trace(t.clone());
    }
    plot
}