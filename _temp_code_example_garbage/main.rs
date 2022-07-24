mod pages;
mod router;
use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local as sl;
use yew::prelude::*;
use yew::{html, Callback};
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    return html! {
       html! {
           <BrowserRouter>
               <Switch<router::Route> render={Switch::render(router::switch)} />
           </BrowserRouter>
       }
       };
}

fn main() {
    yew::start_app::<App>();
}
