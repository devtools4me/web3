use std::ops::Div;
use std::str::FromStr;

use yew::prelude::*;
use yew_plotly::Plotly;
use yew_plotly::plotly::color::NamedColor;

use algotrader_api::types::*;
use algotrader_common::utils::vec_utils::convert;

use crate::utils::ui_utils::*;

#[derive(Properties, PartialEq)]
pub struct SpreadZScoreDataProps {
    pub market1: String,
    pub market2: String,
    pub data: SpreadZScoreData,
}

#[function_component(SpreadZScoreDataView)]
pub fn spread_zscore_component(SpreadZScoreDataProps { market1, market2, data }: &SpreadZScoreDataProps) -> Html {
    let SpreadZScoreData { series_1, series_2, spread, z_score, timestamp } = data;
    let percentage_title = format!("Percentage {} {}", market1, market2);
    let percentage_plot = plot_with_scatters(
        timestamp.clone(),
        vec![
            (
                percentage(series_1), market1.clone(), NamedColor::Blue
            ),
            (
                percentage(series_2), market2.clone(), NamedColor::Red
            ),
        ]);
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
            <h1 class="title">{percentage_title.as_str()}</h1>
            <Plotly plot={percentage_plot}/>
            <h1 class="title">{spread_title.as_str()}</h1>
            <Plotly plot={spread_plot}/>
            <h1 class="title">{zscore_title.as_str()}</h1>
            <Plotly plot={zscore_plot}/>
        </div>
    }
}

fn percentage(v: &Vec<f32>) -> Vec<f64> {
    if v.is_empty() {
        vec![]
    } else {
        let head = f64::from(v.first().unwrap().clone());
        convert(v.clone(), |x: f32| f64::from(x).div(head))
    }
}