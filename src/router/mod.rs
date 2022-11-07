use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::not_found::NotFound;
use crate::pages::secure::Secure;
use crate::pages::get_from_backend::GetFromBackend;
use yew::{html, Html};
use yew_router::prelude::*;

/* Enum of pages routes */
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    
    #[at("/")] // -> url for the page and define a Name for the component that should be used.
    Home, // -> used for expressing a component in rust module. Think of this as a representation of a page.

    #[at("/secure")]
    Secure,

    #[not_found]
    #[at("/404")]
    NotFound,

    #[at("/login")]
    Login,

    #[at("/get_from_backend")]
    GetFromBackend,
}



/* Actual pages dispatching with Route */
pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home initial_contents="initial_content" something_was_sent_back = ""/> },
        Route::Secure => html! { <Secure/> },
        Route::NotFound => html! {<NotFound/>},
        Route::Login => html! { <Login /> },
        Route::GetFromBackend => html! { <GetFromBackend /> },
    }
}
