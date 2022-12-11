use yew::{function_component, html, use_context, Html, UseStateHandle};

use crate::app::Jwt;

#[function_component(Games)]
pub fn games() -> Html {
    let jwt = use_context::<UseStateHandle<Jwt>>().expect("Cannot find user JWT struct");

    html! {
        <div class="content" style="padding-top: 2.5rem;">
            <h1>
                {"Games"}
            </h1>
            <div class="dflex">
                <div>
                    <img class="game-thumb" src="https://www.htmlgames.com/uploaded/game/thumb/soldierattack1300200.jpg"/>
                    <div class="game-title font-sm">{"Soldier Attack 1"}</div>
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
                    {"Friends (69)"}
                </h1>
            }

        </div>
    }
}
