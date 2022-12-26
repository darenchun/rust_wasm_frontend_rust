mod pages;
mod router;
use yew::prelude::*;
use yew::{html};
use yew_router::prelude::*;
mod data_types;

#[function_component(App)]
fn app() -> Html {
    /* init wasm_logger for console.log */
    wasm_logger::init(wasm_logger::Config::default());

    /* set application to utilize "cookies". */
    
    /* run!! */
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


