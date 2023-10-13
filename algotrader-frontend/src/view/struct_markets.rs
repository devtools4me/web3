use log::info;
use yew::prelude::*;

use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

pub struct StructMarkets {
    pub markets: Vec<String>,
    pub selected_market: String,
}

pub enum StructMarketsMessage {
    MarketSelected(Event)
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub markets: Vec<String>,
    pub selected_market: String,
}

impl Component for StructMarkets {
    type Message = StructMarketsMessage;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        Self {
            markets: props.markets.clone(),
            selected_market: props.selected_market.clone()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback: Callback<String> = Callback::from(move |e: String| {

        });

        //let on_market_change = on_select_element(callback);
        let on_market_change = ctx.link().callback(|e| StructMarketsMessage::MarketSelected(e));
        let market_data_html = self.markets.iter().map(|x| {
            let selected = x.eq(self.selected_market.as_str());
            html! {
                <option selected={selected} value={x.clone()}>{x.clone()}</option>
            }
        });
        html! {
            <div class="select">
                <select class="is-focused" onchange={on_market_change}>
                    {for market_data_html}
                </select>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            StructMarketsMessage::MarketSelected(event) => {
                let input = event.target()
                    .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
                if let Some(input) = input {
                    info!("index={}, value={}", input.selected_index(), input.value());
                    self.selected_market = input.value().clone();
                    //callback.emit(input.value());
                    true
                } else {
                    false
                }
            }
        }
    }
}

