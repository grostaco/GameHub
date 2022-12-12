use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::KeyboardEvent;
use yew::{
    function_component, html, use_context, use_effect_with_deps, use_mut_ref, use_state, Html,
    UseStateHandle,
};
use yew_hooks::use_effect_once;
use yew_router::prelude::use_navigator;

use crate::{app::Jwt, components::Nav, routes::Route};

#[function_component(Play)]
pub fn play() -> Html {
    let jwt = use_context::<UseStateHandle<Jwt>>().unwrap();
    let navigator = use_navigator().unwrap();

    if jwt.access_token.is_empty() {
        navigator.push(&Route::Register);
    }
    let canvas = use_state(Option::default);
    let key = use_state(Option::default);
    let x = use_mut_ref(f64::default);
    let y = use_mut_ref(f64::default);

    {
        let outer_canvas = canvas.clone();
        let x = x.clone();
        let y = y.clone();
        use_effect_once(move || {
            let document = web_sys::window().unwrap().document().unwrap();
            let canvas = document.get_element_by_id("canvas").unwrap();
            let canvas: web_sys::HtmlCanvasElement = canvas
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .map_err(|_| ())
                .unwrap();

            *x.borrow_mut() = canvas.width() as f64 / 2.;
            *y.borrow_mut() = canvas.height() as f64 / 2.;

            let canvas = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            outer_canvas.set(Some(canvas));
            || ()
        });
    }

    {
        let key = key.clone();
        use_effect_once(move || {
            let closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                key.set(Some(event.key()));
            });
            let document = gloo::utils::document();
            document
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
                .unwrap();
            move || {
                document
                    .remove_event_listener_with_callback(
                        "keydown",
                        closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        });
    }

    {
        let key = key.clone();
        let canvas = canvas;
        let x = x.clone();
        let y = y.clone();
        use_effect_with_deps(
            move |key| {
                if let Some(s) = key.as_ref() {
                    if let Some(canvas) = canvas.as_ref() {
                        canvas.begin_path();

                        let mut x = x.borrow_mut();
                        let mut y = y.borrow_mut();
                        canvas.move_to(*x, *y);
                        match s.as_str() {
                            "w" => *y -= 10.,
                            "s" => *y += 10.,
                            "a" => *x -= 10.,
                            "d" => *x += 10.,
                            _ => {}
                        }
                        key.set(None);

                        canvas.line_to(*x, *y);
                        canvas.stroke();
                    }
                }
            },
            key,
        );
    };

    html! {
        <>
        <Nav />
        <h1>{"Line Adventure"}</h1>
        <canvas id="canvas" width="1024px" height="1024px">

        </canvas>
        </>
    }
}
