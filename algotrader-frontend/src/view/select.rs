use yew::prelude::*;

use algotrader_api::types::*;

use crate::utils::ui_utils::on_select_element;

#[derive(Properties, PartialEq)]
pub struct SelectProps {
    pub values: Vec<String>,
    pub selected_value: String,
    pub callback: Callback<String>,
}

#[function_component(SelectView)]
pub fn select_component(props: &SelectProps) -> Html {
    let SelectProps { values, selected_value, callback } = props;
    let callback = callback.clone();
    let on_change = on_select_element(callback);
    let values_html = values.iter().map(|x| {
        let selected = x.eq(selected_value);
        html! {
            <option selected={selected} value={x.clone()}>{x.clone()}</option>
        }
    });
    html! {
        <div class="select">
            <select class="is-focused" onchange={on_change}>
                {for values_html}
            </select>
        </div>
    }
}