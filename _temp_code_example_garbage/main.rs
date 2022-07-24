mod pages;
mod router;
use yew::prelude::*;
use yew::{html};
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    /* async fn get_contents_from_backend() {
        let greeting = String::from("Hi there1");
        web_sys::console::log_1(&greeting.into()); // console cmd for browser
        //Response from backend
        let contents_from_backend = Request::get("http://127.0.0.1:8081/articles").send().await;
        match contents_from_backend {
            Ok(content) => {
                web_sys::console::log_1(&content.as_raw()); // console cmd for browser
            }
            Err(err) => {
                web_sys::console::log_1(&JsValue::from_str(&err.to_string())); // console cmd for browser
            }
        }
    } */

    /* let onclick = Callback::from(move |_| {
        sl(get_contents_from_backend());
    }); */

    /* let move_to_second_page = Callback::from(move |_| {
        web_sys::console::log_1(&JsValue::from_str("moving to second page")); // console cmd for browser
    }); */

    return html! {
    /* <>
       <h1>{"get contents"}</h1>
       <p></p>
       <button onclick={onclick}>{ "Click" }</button>
       <h1>{"move_to_second_page"}</h1>
       <button onclick={move_to_second_page}>{ "Click" }</button>
       </> */
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
