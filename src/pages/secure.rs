use yew_router::prelude::*;
use yew::prelude::*;
use crate::router::Route;

#[function_component(Secure)]
pub fn secure() ->Html {
    //using 'history' to move back to page
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(Route::Home));
    return html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    };
}

