use yew::prelude::*;
use yew_router::prelude::{Link, Redirect};
use crate::router::Route;


#[function_component(GetFromBackend)]
pub fn get_from_backend() -> Html {
    return html! {
        <div>
        /* redirect to other pages in router! how to send data and how to use it in that page is a mistery to me for now... */
        <Redirect<Route> to={Route::Home}/>
        <br/>
        <h1>{ "get something back from backend" }</h1>
            <button onclick={Callback::from(|_| (log::info!("testing")))}>{ "Get JSON information" }</button>
            
        </div>
    };
}
