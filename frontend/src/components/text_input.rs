use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement};
use yew::{function_component, html, Callback, Html, Properties, UseStateHandle};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_change: UseStateHandle<String>,
    #[prop_or("text")]
    pub input_type: &'static str,
    #[prop_or_default]
    pub placeholder: &'static str,
}

fn get_value_from_input_event(e: Event) -> String {
    e.prevent_default();
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();

    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let Props {
        on_change,
        input_type,
        placeholder,
    } = props.clone();
    let onchange = {
        Callback::from(move |event: Event| {
            on_change.set(get_value_from_input_event(event));
        })
    };

    html! {
        <input class="text-inp1" type={input_type} {placeholder} {onchange} />
    }
}
