use crate::{
    app::Jwt,
    components::{Loading, Nav, TextArea},
    routes::Route,
    services::user::{get_user_by_id, get_user_info, patch_user_info, UserPatchRequest},
};
use futures::future::join_all;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{Element, MouseEvent};
use yew::{
    function_component, html, use_context, use_mut_ref, use_state, Callback, Html, UseStateHandle,
};
use yew_hooks::{use_async, use_effect_once, use_effect_update_with_deps};
use yew_router::prelude::use_navigator;

#[function_component(Profile)]
pub fn profile() -> Html {
    let jwt = use_context::<UseStateHandle<Jwt>>().unwrap();
    let navigator = use_navigator().unwrap();
    let editing_bio = use_state(bool::default);
    let bio = use_mut_ref(String::new);

    let onclick_edit_bio = {
        let editing_bio = editing_bio.clone();
        Callback::from(move |_| {
            editing_bio.set(true);
        })
    };

    let onclick_cancel = {
        let editing_bio = editing_bio.clone();
        Callback::from(move |_| {
            editing_bio.set(false);
        })
    };

    let onclick_user = {
        let navigator = navigator.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event.target().unwrap();
            let event: Element = target.dyn_into().unwrap();
            navigator.push(&Route::UserProfile { id: event.id() });
        })
    };

    if jwt.access_token.is_empty() {
        navigator.push(&Route::Register);
    }
    let user_info = {
        let jwt = jwt.clone();
        use_async(async move { get_user_info(jwt.access_token.as_str()).await })
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

    let patch_user_info = {
        let jwt = jwt.clone();
        let bio = bio.clone();

        use_async(async move {
            patch_user_info(
                &jwt.access_token,
                &UserPatchRequest {
                    username: None,
                    bio: Some(bio.borrow().to_string()),
                    avatar: None,
                },
            )
            .await
        })
    };

    let onclick_submit = {
        let editing_bio = editing_bio.clone();
        let user_info = user_info.clone();

        Callback::from(move |_| {
            editing_bio.set(false);
            patch_user_info.run();
            user_info.run();
        })
    };

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
                if !*editing_bio {
                    <div class="dflex dflex-justify-center dflex-gap-sm">
                        <h3>{"About Me"}</h3>
                        <svg onclick={onclick_edit_bio} xmlns="http://www.w3.org/2000/svg" version="1.1" id="Capa_1" x="0px" y="0px" width="24px" height="24px" viewBox="0 0 494.936 494.936" fill="white" stroke="white" stroke-width="10">
                        <g>
                            <g>
                                <path d="M389.844,182.85c-6.743,0-12.21,5.467-12.21,12.21v222.968c0,23.562-19.174,42.735-42.736,42.735H67.157    c-23.562,0-42.736-19.174-42.736-42.735V150.285c0-23.562,19.174-42.735,42.736-42.735h267.741c6.743,0,12.21-5.467,12.21-12.21    s-5.467-12.21-12.21-12.21H67.157C30.126,83.13,0,113.255,0,150.285v267.743c0,37.029,30.126,67.155,67.157,67.155h267.741    c37.03,0,67.156-30.126,67.156-67.155V195.061C402.054,188.318,396.587,182.85,389.844,182.85z"></path>
                                <path d="M483.876,20.791c-14.72-14.72-38.669-14.714-53.377,0L221.352,229.944c-0.28,0.28-3.434,3.559-4.251,5.396l-28.963,65.069    c-2.057,4.619-1.056,10.027,2.521,13.6c2.337,2.336,5.461,3.576,8.639,3.576c1.675,0,3.362-0.346,4.96-1.057l65.07-28.963    c1.83-0.815,5.114-3.97,5.396-4.25L483.876,74.169c7.131-7.131,11.06-16.61,11.06-26.692    C494.936,37.396,491.007,27.915,483.876,20.791z M466.61,56.897L257.457,266.05c-0.035,0.036-0.055,0.078-0.089,0.107    l-33.989,15.131L238.51,247.3c0.03-0.036,0.071-0.055,0.107-0.09L447.765,38.058c5.038-5.039,13.819-5.033,18.846,0.005    c2.518,2.51,3.905,5.855,3.905,9.414C470.516,51.036,469.127,54.38,466.61,56.897z"></path>
                            </g>
                        </g>
                        </svg>
                    </div>
                    <span>{&user_info.bio}</span>
                } else {
                    <h3>{"About Me"}</h3>
                    <div class="dflex dflex-col dflex-gap-sm">
                        <TextArea on_change={bio} content={user_info.bio.clone()} readonly=false />
                        <div class="dflex dflex-gap-sm" style="justify-content: flex-end;">
                            <a id="submit-btn" onclick={onclick_cancel}>{"Cancel"}</a>
                            <a id="submit-btn" onclick={onclick_submit}>{"Submit"}</a>
                        </div>
                    </div>
                }
                <h3>{"Last Played"}</h3>
                <h3>{"Friends"}</h3>
                if let Some(friends) = &friends.data {
                    if friends.is_empty() {
                        <div>{"You have no friends :("}</div>
                    } else {
                        <div class="dflex dflex-gap-sm">
                            {for friends.into_iter().map(|user|
                                if let Ok(user) = user {
                                    html!{
                                        <div class="dflex dflex-col dflex-gap-tn dflex-justify-center">
                                            <img src={user.avatar.clone()} id={user.id.clone()} onclick={onclick_user.clone()} width="40px" height="40px" style="border-radius: 50%; cursor: pointer;"/>
                                            <div>{user.username.clone()}</div>
                                        </div>
                                    }
                                } else {
                                    html!{}
                                }
                            )}
                        </div>
                    }
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
