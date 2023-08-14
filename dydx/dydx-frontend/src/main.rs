use yew::prelude::*;

mod data_source;
pub mod mock;
mod gloonet;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let data = data_source::get_data().unwrap();
        let cur_data_html = data.iter().map(|x| {
            html! {
                <tr>
                    <td>{x.open.clone()}</td>
                    <td>{x.high.clone()}</td>
                    <td>{x.low.clone()}</td>
                    <td>{x.close.clone()}</td>
                    <td>{x.timestamp.clone()}</td>
                </tr>
            }
        });

        html! {
            <div class="section">
                <div class="container">
                    <h1 class="title">{"Main page"}</h1>
                    <div>
                        <table class="table is-hoverable is-striped">
                            <thead>
                                <tr>
                                    <th>{"Open"}</th>
                                    <th>{"High"}</th>
                                    <th>{"Low"}</th>
                                    <th>{"Close"}</th>
                                    <th>{"Timestamp"}</th>
                                </tr>
                            </thead>
                            <tbody>
                            {for cur_data_html}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}