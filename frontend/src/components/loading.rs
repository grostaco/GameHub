use gloo_timers::callback::Timeout;
use yew::{function_component, html, use_effect, use_state, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: &'static str,
}
#[function_component(Loading)]
pub fn loading(props: &Props) -> Html {
    let counter = use_state(|| 0);
    {
        let counter = counter.clone();
        use_effect(move || {
            let timeout = Timeout::new(300, move || {
                counter.set(*counter + 1);
            });
            || {
                timeout.cancel();
            }
        });
    }

    html! {
        <p>{format!("{}{}", props.text, ".".repeat((*counter % 4) as usize))}</p>
    }
}
