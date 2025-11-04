use yew::prelude::*;
use web_sys::HtmlInputElement;

pub fn bind_input(state: UseStateHandle<String>) -> Callback<InputEvent> {
    let state_for_closure = state.clone();

    Callback::from(move |e: InputEvent| {
        if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
            state_for_closure.set(input.value());
        }
    })
}
