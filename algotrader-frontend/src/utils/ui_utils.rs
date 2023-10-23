use plotly::{Plot, Scatter};
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew::prelude::*;
use yew_plotly::plotly;
use yew_plotly::plotly::color::NamedColor;

use algotrader_common::utils::vec_utils::convert;

pub fn on_select_element(callback: Callback<String>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        let input = e.target()
            .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
        if let Some(input) = input {
            //info!("index={}, value={}", input.selected_index(), input.value());
            callback.emit(input.value());
        }
    })
}

pub fn plot_with_scatters(x: Vec<String>, y: Vec<(Vec<f64>, String, NamedColor)>) -> Plot {
    let scatters = convert(y, |t: (Vec<f64>, String, NamedColor)| {
        Scatter::new(x.clone(), t.0)
            .name(t.1.as_str())
            .line(plotly::common::Line::new().color(t.2))
    });
    scatters_to_plot(scatters)
}

pub fn plot_with_scatter(x: Vec<String>, y: Vec<f64>, name: String, color: NamedColor) -> Plot {
    let trace = Scatter::new(x.clone(), y)
        .name(name.as_str())
        .line(plotly::common::Line::new().color(color));
    scatters_to_plot(vec![trace])
}

pub fn scatters_to_plot(traces: Vec<Box<Scatter<String, f64>>>) -> Plot {
    let mut plot = Plot::new();
    for t in traces.iter() {
        plot.add_trace(t.clone());
    }
    plot
}