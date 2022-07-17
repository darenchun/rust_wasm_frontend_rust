use yew_router::prelude::*;
use yew::prelude::*;

use super::page_route::Route;

#[function_component(Secure)]
pub fn secure() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

