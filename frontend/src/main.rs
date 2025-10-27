mod components;
mod api_config;

use components::task_list::TaskList;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <TaskList />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
