use gloo_timers::callback::Timeout;
use yew::{function_component, html, use_effect, use_state, Html};

#[function_component(Loading)]
pub fn loading() -> Html {
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
        <p>{format!("Submitting{}", ".".repeat((*counter % 4) as usize))}</p>
    }
}
