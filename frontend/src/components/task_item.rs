use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use api_client::models::Task;
use api_client::apis::tasks_api;
use crate::api_config::config;

#[derive(Properties, PartialEq)]
pub struct TaskItemProps {
    pub task: Task,
    pub on_update: Callback<Task>, // parent gets updated Task back
}

#[function_component(TaskItem)]
pub fn task_item(props: &TaskItemProps) -> Html {
    let task = use_state(|| props.task.clone());
    let on_update = props.on_update.clone();

    let onclick_mark_done = {
        let task = task.clone();
        let on_update = on_update.clone();
        Callback::from(move |_| {
            let task = task.clone();
            let on_update = on_update.clone();
            spawn_local(async move {
                if !task.done {
                    let config = config();
                    if let Ok(updated_task) = tasks_api::mark_task_done(&config, task.id as i32).await {
                        task.set(updated_task.clone());
                        on_update.emit(updated_task);
                    }
                }
            });
        })
    };

    let done_style = if task.done {
        "line-through text-gray-500 transition-opacity duration-200 opacity-70"
    } else {
        "transition-opacity duration-200 opacity-100"
    };

    html! {
        <li class="flex justify-between items-center px-2 py-1 hover:bg-gray-50 rounded transition-colors">
            <span class={classes!(done_style, "truncate")}>{ &task.title }</span>
            {
                if !task.done {
                    html! {
                        <button
                            class="text-sm bg-green-500 hover:bg-green-600 text-white px-2 py-1 rounded transition-all duration-200"
                            onclick={onclick_mark_done}
                        >
                            { "Mark done" }
                        </button>
                    }
                } else {
                    html! {
                        <button
                            class="text-sm px-2 py-1 rounded opacity-0 pointer-events-none"
                            disabled=true
                        >
                            { "Done" }
                        </button>
                    }
                }
            }
        </li>
    }
}
