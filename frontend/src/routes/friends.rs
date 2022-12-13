use yew::{function_component, html, use_context, Html, UseStateHandle};
use yew_router::prelude::use_navigator;

use crate::{app::Jwt, components::Nav, routes::Route};

#[function_component(Friends)]
pub fn friends() -> Html {
    let jwt = use_context::<UseStateHandle<Jwt>>().unwrap();
    let navigator = use_navigator().unwrap();

    if jwt.access_token.is_empty() {
        navigator.push(&Route::Register);
    }

    html! {
        <>
        <Nav />

        </>
    }
}
