use crate::components::*;
use yew::{function_component, html, Html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <Nav />
            <Games />
        </>
    }
}
