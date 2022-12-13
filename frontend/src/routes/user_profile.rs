use crate::{
    app::Jwt,
    components::{Loading, Nav},
    routes::Route,
    services::user::get_user_by_id,
};
use futures::future::join_all;
use gloo::console::log;
use yew::{function_component, html, use_context, Html, Properties, UseStateHandle};
use yew_hooks::{use_async, use_effect_once, use_effect_update_with_deps};
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(UserProfile)]
pub fn user_profile(props: &Props) -> Html {
    let jwt = use_context::<UseStateHandle<Jwt>>().unwrap();
    let navigator = use_navigator().unwrap();

    let id = props.id.clone();

    if jwt.access_token.is_empty() {
        navigator.push(&Route::Register);
    }
    let user_info = {
        let jwt = jwt.clone();
        use_async(async move { get_user_by_id(&id, jwt.access_token.as_str()).await })
    };

    let friends = {
        let jwt = jwt.clone();
        let user_info = user_info.clone();
        use_async(async move {
            if let Some(user_info) = user_info.data.as_ref() {
                let fut = join_all(
                    user_info
                        .friends
                        .iter()
                        .skip(1)
                        .map(|id| get_user_by_id(id, jwt.access_token.as_str())),
                )
                .await;
                log!(format!("{fut:#?}"));
                return Ok::<_, ()>(fut);
            }

            Ok(Vec::new())
        })
    };

    {
        let friends = friends.clone();
        let user_info = user_info.clone();
        use_effect_update_with_deps(
            move |_| {
                friends.run();
                || ()
            },
            user_info,
        );
    }
    {
        let user_info = user_info.clone();
        use_effect_once(move || {
            user_info.run();
            || ()
        });
    }

    html! {
        <>
        <Nav />
        if let Some(user_info) = &user_info.data {
            <div class="form-container" style="justify-content: center; margin-top: 1.5rem;">
                <div class="dflex dflex-gap-sm dflex-justify-center">
                    <img src={user_info.avatar.clone()} width="40px" height="40%" style="border-radius: 50%"/>
                    <span>{format!("{}'s profile", user_info.username)}</span>
                </div>
                <div>{format!("User ID: {}", user_info.id)}</div>
                <h3>{"About Me"}</h3>
                <span>{&user_info.bio}</span>

                <h3>{"Last Played"}</h3>
                <h3>{"Friends"}</h3>
                if let Some(friends) = &friends.data {
                    if friends.is_empty() {
                        <div>{"They have no friends :("}</div>
                    } else {
                        <div class="dflex dflex-gap-sm">
                            {for friends.into_iter().map(|user|
                                if let Ok(user) = user {
                                    html!{
                                        <div class="dflex dflex-col dflex-gap-tn dflex-justify-center">
                                            <img src={user.avatar.clone()} uid={user.id.clone()} width="40px" height="40px" style="border-radius: 50%"/>
                                            <div>{user.username.clone()}</div>
                                        </div>
                                    }
                                } else {
                                    html!{}
                                }
                            )}
                        </div>
                    }
                    <a style="color: #f8981f; cursor: pointer;">{"Add as friend"}</a>
                } else {
                    <Loading text="Fetching friends"/>
                }
            </div>
        } else {
            <Loading text="Fetching profile data"/>
        }
        </>
    }
}
