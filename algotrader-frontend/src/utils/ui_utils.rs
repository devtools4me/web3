use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

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