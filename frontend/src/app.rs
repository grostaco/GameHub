use crate::routes::{switch, Route};
use yew::{function_component, html, use_state, ContextProvider, Html, UseStateHandle};
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Jwt {
    pub access_token: String,
    pub refresh_token: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let context = use_state(|| Jwt {
        access_token: String::new(),
        refresh_token: String::new(),
    });

    html! {
        <ContextProvider<UseStateHandle<Jwt>> context={context}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<UseStateHandle<Jwt>>>
    }
}
