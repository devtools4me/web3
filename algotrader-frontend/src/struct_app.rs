// use std::rc::Rc;
// use yew::prelude::*;
// use yewdux::prelude::*;
// use crate::stores::AppStore;
//
// pub enum Msg {
//     Store(Rc<AppStore>),
// }
//
// pub struct MarketsSelect {
//     dispatch: Dispatch<AppStore>,
// }
//
// impl Component for MarketsSelect {
//     type Message = Msg;
//
//     type Properties = ();
//
//     fn create(ctx: &Context<Self>) -> Self {
//         let callback: Callback<Rc<AppStore>> = ctx.link().callback(Msg::Store);
//         let dispatch = Dispatch::<AppStore>::subscribe(callback);
//         Self { dispatch }
//     }
//
//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::Store(_) => true,
//         }
//     }
//
//     fn view(&self, _ctx: &Context<Self>) -> Html {
//         let state = self.dispatch.get();
//
//         let market_data_html = state.markets.iter().map(|x| {
//             let selected = x.starts_with("BTC");
//             html! {
//                 <option selected={selected} value={x.clone()}>{x.clone()}</option>
//             }
//         });
//         html! {
//             <div class="section">
//                 <div class="container">
//                     <h1 class="title">{"Market"}</h1>
//                     <div class="select">
//                         <select class="is-focused">// onchange={on_market_change}>
//                             {for market_data_html}
//                         </select>
//                     </div>
//                 </div>
//             </div>
//         }
//     }
// }
//
// #[function_component(App2)]
// fn app() -> Html {
//     html! {
//         <MarketsSelect />
//     }
// }

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux::prelude::use_store;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Store)]
struct Counter {
    count: u32,
}

#[function_component(App)]
pub fn app() -> Html {
    let (store, dispatch) = use_store::<Counter>();
    let onclick = dispatch.reduce_mut_callback(|counter| counter.count += 1);

    html! {
        <>
        <p>{ store.count }</p>
        <button {onclick}>{"+1"}</button>
        </>
    }
}