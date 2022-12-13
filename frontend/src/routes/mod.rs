use yew::{html, Html};
use yew_router::Routable;

mod discovery;
mod friends;
mod home;
mod login;
mod play;
mod profile;
mod register;

use discovery::Discovery;
use friends::Friends;
use home::Home;
use login::Login;
use play::Play;
use profile::Profile;
use register::Register;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/discovery")]
    Discovery,
    #[at("/profile")]
    Profile,
    #[at("/play")]
    Play,
    #[at("/friends")]
    Friends,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <Login /> },
        Route::Register => html! { <Register /> },
        Route::Discovery => html! { <Discovery /> },
        Route::Profile => html! { <Profile /> },
        Route::Play => html! { <Play /> },
        Route::Friends => html! { <Friends /> },
    }
}
