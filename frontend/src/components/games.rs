use yew::{function_component, html, use_context, Callback, Html, UseStateHandle};
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yew_router::prelude::use_navigator;

use crate::{app::Jwt, components::Loading, routes::Route, services::user::get_user_info};

#[function_component(Games)]
pub fn games() -> Html {
    let jwt = use_context::<UseStateHandle<Jwt>>().expect("Cannot find user JWT struct");
    let navigator = use_navigator().unwrap();
    let onclick_play = Callback::from(move |_| {
        navigator.push(&Route::Play);
    });

    let user_info = {
        let jwt = jwt.clone();
        use_async_with_options(
            async move {
                if !jwt.access_token.is_empty() {
                    get_user_info(jwt.access_token.as_str())
                        .await
                        .map(Option::Some)
                } else {
                    Ok(None)
                }
            },
            UseAsyncOptions::enable_auto(),
        )
    };

    html! {
        <div class="content" style="padding-top: 2.5rem;">
            <h1>
                {"Friends"}
            </h1>
            if jwt.access_token.is_empty() {
                <div>{"Log in to see your friends!"}</div>
            } else {
                if let Some(Some(friends)) = &user_info.data.as_ref().map(|data| data.as_ref().map(|d| &d.friends)) {
                    if friends.len() == 1 {
                        <div>{"You have no friends :("}</div>
                    } else {
                        {for friends.iter().skip(1).map(|_| html!{<div>{"Friend"}</div>})}
                    }
                } else {
                    <Loading text="Fetching friends" />
                }
            }
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
        </div>
    }
}
