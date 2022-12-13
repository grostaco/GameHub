use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlTextAreaElement, InputEvent};
use yew::{
    function_component, html, use_effect, use_effect_with_deps, use_state, Callback, Html,
    Properties,
};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_change: Rc<RefCell<String>>,
    pub content: String,
    #[prop_or(true)]
    pub readonly: bool,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    e.prevent_default();
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();

    let target: HtmlTextAreaElement = event_target.dyn_into().unwrap_throw();
    //web_sys::console::log_1(&target.value().into());
    target.value()
}

#[function_component(TextArea)]
pub fn text_area(props: &Props) -> Html {
    let Props {
        on_change,
        content,
        readonly,
    } = props.clone();
    let character_count = use_state(usize::default);
    let content = use_state(|| content);

    let oninput = {
        let content = content.clone();
        Callback::from(move |input_event: InputEvent| {
            let input = get_value_from_input_event(input_event);
            if input.len() <= 1024 {
                content.set(input.clone());
                *on_change.borrow_mut() = input;
            }
        })
    };

    {
        let character_count = character_count.clone();
        let content = content.clone();
        use_effect_with_deps(
            move |content| {
                character_count.set(content.len());
            },
            content,
        )
    }

    use_effect(move || {
        let element = gloo::utils::document()
            .get_element_by_id("text-input")
            .unwrap();
        let text_area: HtmlTextAreaElement = element.dyn_into().unwrap();
        text_area.set_value(&content);
        || ()
    });

    html! {
        <>
        <textarea id="text-input" style="flex: 1" data-gramm="false" rows=10 maxlength="1024" {readonly} {oninput} />
        <div class="dflex dflex-gap-sm" style="justify-content: flex-end;">{format!("{}/1024", *character_count)}</div>
        </>
    }
}
