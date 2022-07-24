use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::pages::secure::Secure;
use crate::pages::login::Login;
use yew::{html, Html};
use yew_router::prelude::*;

/* List of pages routes */
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    
    // url for the page and define a Name for the component that should be used.
    #[at("/")]
    Home, // -> used for expressing component in rust module.

    #[at("/secure")]
    Secure,

    #[not_found]
    #[at("/404")]
    NotFound,

    #[at("/login")]
    Login,
}

/* matching url addresses using match */
pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> }, // -> should return html! macro which 'yew' provides.
        Route::Secure => html! { <Secure/> },
        Route::NotFound => html! {<NotFound/>},
        Route::Login => html! { <Login /> },
    }
}
