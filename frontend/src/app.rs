use std::rc::Rc;

use crate::routes::{switch, Route};
use yew::{function_component, html, use_memo, ContextProvider, Html};
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Jwt {
    access_token: String,
    refresh_token: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let context = use_memo(
        |_| Jwt {
            access_token: String::new(),
            refresh_token: String::new(),
        },
        (),
    );

    html! {
        <ContextProvider<Rc<Jwt>> context={context}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<Rc<Jwt>>>
    }
}
