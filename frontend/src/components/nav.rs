use yew::{function_component, html, use_context, Callback, Html, UseStateHandle};

use crate::app::Jwt;

#[function_component(Nav)]
pub fn nav() -> Html {
    let jwt = use_context::<UseStateHandle<Jwt>>().expect("Cannot find user JWT struct");
    let onclick = {
        let jwt = jwt.clone();
        Callback::from(move |_| jwt.set(Jwt::default()))
    };
    html!(
        <div class="navbar">
            <div class="dflex dflex-justify-center">
                <img src="gamehub.png" height=32px style="padding-right: 1rem;" />
                <div class="underline-expand" href="#">{"Home"}</div>
                <div class="underline-expand" href="#">{"Discover"}</div>
                <div class="underline-expand" href="#">{"Create"}</div>
                if !jwt.access_token.is_empty() {
                    <div class="underline-expand" href="#">{"Friends"}</div>
                    <div class="underline-expand" href="#">{"Profile"}</div>
                }
            </div>
            <div class="dflex dflex-justify-center dflex-gap-sm">
                if jwt.access_token.is_empty() {
                    <a id="signup-btn" href="/register">
                        {"Sign Up"}
                    </a>
                    <a id="signup-btn" style="background-color: #00b06f; border: #00b06f solid;" href="/login">
                        {"Log in"}
                    </a>
                } else {
                    <span>{"Profile"}</span>
                    <button id="signup-btn" style="background-color: #00b06f; border: #00b06f solid;" {onclick}>{"Logout"}</button>
                }
            </div>
        </div>
    )
}
