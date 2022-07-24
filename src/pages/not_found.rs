
use yew::prelude::*;


#[function_component(NotFound)]
pub fn not_found() -> Html {
    return html! {
        <div>
            <h1>{ "404 Not Found" }</h1>
        </div>
    };
}