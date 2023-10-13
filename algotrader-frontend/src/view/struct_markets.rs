use yew::prelude::*;

use algotrader_common::utils::env_utils;

use crate::utils::ui_utils::on_select_element;

pub struct StructMarkets {
    pub markets: Vec<String>,
    pub selected_market: String,
}

impl Component for StructMarkets {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        StructMarkets {
            markets: env_utils::get_markets(),
            selected_market: env_utils::get_market()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let callback: Callback<String> = Callback::from(move |e: String| {

        });

        let on_market_change = on_select_element(callback);
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
}

