use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::home::main_home::MainHome;
use crate::components::profile::main_profile::MainProfile;
use crate::components::routes::Route;
use crate::components::settings::main_settings::MainSettings;
use crate::components::login_and_signin::main_login_and_signin::MainLoginAndSignin;
use gloo::console::log;

#[function_component(MainRouter)]
pub fn main_router() -> Html {
    log!("Navigation occurred! Triggering function.");

    html! {
        <Switch<Route> render={switch} />
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <MainHome /> },
        Route::Settings => html! { <MainSettings/> },
        Route::Profile => html! { <MainProfile /> },
        Route::Login => html! { <MainLoginAndSignin /> },
        Route::NotFound => html! { <h1>{ "404 - Not Found" }</h1> },
    }
}
