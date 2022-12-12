use yew::{function_component, html, use_context, Callback, Html, UseStateHandle};
use yew_router::prelude::use_navigator;

use crate::{app::Jwt, routes::Route};

#[function_component(Nav)]
pub fn nav() -> Html {
    let jwt = use_context::<UseStateHandle<Jwt>>().expect("Cannot find user JWT struct");
    let navigator = use_navigator().unwrap();
    let onclick_logout = {
        let jwt = jwt.clone();
        Callback::from(move |_| jwt.set(Jwt::default()))
    };

    let onclick_home = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Home);
        })
    };

    let onclick_profile = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Profile);
        })
    };

    let onclick_signup = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Register);
        })
    };
    let onclick_login = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Login);
        })
    };
    let onclick_discovery = Callback::from(move |_| {
        navigator.push(&Route::Discovery);
    });

    html!(
        <div class="navbar">
            <div class="dflex dflex-justify-center">
                <img src="gamehub.png" height=32px style="padding-right: 1rem;" />
                <div class="underline-expand" onclick={onclick_home}>{"Home"}</div>
                <div class="underline-expand" onclick={onclick_discovery}>{"Discover"}</div>
                <div class="underline-expand" href="#">{"Create"}</div>
                if !jwt.access_token.is_empty() {
                    <div class="underline-expand" href="#">{"Friends"}</div>
                }
            </div>
            <div class="dflex dflex-justify-center dflex-gap-sm">
                if jwt.access_token.is_empty() {
                    <a id="signup-btn" onclick={onclick_signup}>
                        {"Sign Up"}
                    </a>
                    <a id="signup-btn" style="background-color: #00b06f; border: #00b06f solid;" onclick={onclick_login}>
                        {"Log in"}
                    </a>
                } else {
                    <a id="signup-btn" onclick={onclick_profile}>{"Profile"}</a>
                    <a id="signup-btn" style="background-color: #b23b3b; border: #b23b3b solid; color: white;" onclick={onclick_logout}>{"Logout"}</a>
                }
            </div>
        </div>
    )
}
