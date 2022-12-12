use yew::{function_component, html, use_context, Callback, Html, UseStateHandle};
use yew_router::prelude::use_navigator;

use crate::{app::Jwt, routes::Route};

#[function_component(Games)]
pub fn games() -> Html {
    let jwt = use_context::<UseStateHandle<Jwt>>().expect("Cannot find user JWT struct");
    let navigator = use_navigator().unwrap();
    let onclick_play = Callback::from(move |_| {
        navigator.push(&Route::Play);
    });

    html! {
        <div class="content" style="padding-top: 2.5rem;">
            <h1>
                {"Games"}
            </h1>
            <div class="dflex">
                <div onclick={onclick_play}>
                    <img class="game-thumb" src="https://www.htmlgames.com/uploaded/game/thumb/soldierattack1300200.jpg"/>
                    <div class="game-title font-sm">{"Line Adventure"}</div>
                    <div class="dflex dflex-gap-tn">
                        <span class="play-count"></span>
                        <span class="play-count-text">{"69K"}</span>
                    </div>
                </div>
            </div>
            if jwt.access_token.is_empty() {
                <h1>
                    {"Friends (0)"}
                </h1>
                <div>{"Log in to see your friends!"}</div>
            } else {
                <h1>
                    {"Friends (0)"}
                </h1>
            }

        </div>
    }
}
