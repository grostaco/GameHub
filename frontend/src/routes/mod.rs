use yew::{html, Html};
use yew_router::Routable;

mod home;
use home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    //#[at("/play")]
    //Play,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        //Route::Play => html! { <Play /> },
    }
}
