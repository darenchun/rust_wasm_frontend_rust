use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::secure::Secure;
use yew::{html, Html};
use yew_router::prelude::*;

/* List of pages routes */
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/* matching url addresses using match */
pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> }, // -> shoud return html! macro which 'yew' provides.
        Route::Secure => html! { <Secure/> },
        Route::NotFound => html! {<NotFound/>},
    }
}
