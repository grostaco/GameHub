use crate::app::Jwt;
use crate::components::{Loading, Nav, TextInput};
use crate::routes::Route;
use crate::services::auth;
use yew::{
    function_component, html, use_context, use_effect_with_deps, use_state, Callback, Html,
    UseStateHandle,
};
use yew_hooks::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(Register)]
pub fn register() -> Html {
    let navigator = use_navigator().unwrap();
    let username = use_state(String::new);
    let password = use_state(String::new);
    let email = use_state(String::new);
    let submitting = use_state(bool::default);

    let context = use_context::<UseStateHandle<Jwt>>().expect("Expected JWT context");

    let register = {
        let username = username.clone();
        let password = password.clone();
        let email = email.clone();
        use_async(async move {
            auth::register(username.as_str(), password.as_str(), email.as_str()).await
        })
    };

    let onclick = {
        let register = register.clone();
        let submitting = submitting.clone();
        Callback::from(move |_| {
            register.run();
            submitting.set(true);
        })
    };

    let onclick_login = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Login);
        })
    };

    {
        let register = register.clone();
        use_effect_update_with_deps(
            move |register| {
                if let Some(response) = register.data.as_ref() {
                    context.set(Jwt {
                        access_token: response.access_token.clone(),
                        refresh_token: response.refresh_token.clone(),
                    });
                    navigator.push(&Route::Home);
                }

                || ()
            },
            register,
        )
    }

    {
        let username = username.clone();
        let password = password.clone();
        let email = email.clone();
        use_effect_with_deps(
            move |(username, password, email)| {
                let document = gloo::utils::document();
                let btn = document
                    .get_element_by_id("submit-btn")
                    .expect("Cannot find submit button");
                if username.is_empty() || password.is_empty() || email.is_empty() {
                    btn.set_attribute("disabled", "").unwrap();
                } else {
                    btn.remove_attribute("disabled").unwrap();
                }
            },
            (username, password, email),
        );
    }

    let submitting = *submitting;
    html! {
        <>
        <Nav/>

        <div class="form-container" style="margin-top: 2.5rem">
            <label>{"Username"}</label>
            <TextInput on_change={username} placeholder="Enter Username"/>
            <label>{"Password"}</label>
            <TextInput on_change={password} placeholder="Enter Password" input_type="password" />
            <label>{"Email"}</label>
            <TextInput on_change={email} placeholder="Enter Email"/>
            <button type="button" id="submit-btn" {onclick} disabled={submitting}>{"Submit"}</button>
            if submitting {
                <Loading text="Registering"/>
            }
        </div>
        <div class="dflex dflex-gap-tn" style="justify-content: center; padding-top: 0.5rem;">
            <span>{"Already got an account? "}</span>
            <a style="color: #f8981f; cursor: pointer;" onclick={onclick_login}>{"Log in"}</a>
        </div>
        </>
    }
}
