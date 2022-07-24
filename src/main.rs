mod pages;
mod router;
mod cookies;
use yew::prelude::*;
use yew::{html};
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    /* set application to utilize "cookies". */


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