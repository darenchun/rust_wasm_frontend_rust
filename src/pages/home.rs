
use yew::{function_component, html, Properties};
use yew_router::prelude::Link;
use crate::router::Route;

/********************* functions and variables used in this page */

/* 
   this is props!! consider this as variables that are "given from giver". 
   So these cannot be decided from this page lvl, but from the former executed page.
*/
#[derive(Properties, PartialEq)]
pub struct PropsForHome {
    pub initial_contents: String,
    pub something_was_sent_back : String,
}

#[function_component(Home)]
pub fn home(props: &PropsForHome) -> Html {
    /* contents of page */
    return html! {
    <div>
        <h1>{ "home" }</h1>
        <button>{ &props.initial_contents }</button>
        if props.something_was_sent_back!="" {
            <p>{ "something was sent back" }</p>
        }
        <br/>
        <br/>
        <Link<Route> to={Route::GetFromBackend}>{ "Moving to other page called GetfromBackend" }</Link<Route>>
    </div>
    };
}
