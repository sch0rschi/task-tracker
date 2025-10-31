use crate::api_config::config;
use api_client::apis::tasks_api;
use api_client::models::{RenameTask, Task};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TaskItemProps {
    pub task: Task,
    pub on_update: Callback<Task>,
}

#[function_component(TaskItem)]
pub fn task_item(props: &TaskItemProps) -> Html {
    let task = use_state(|| props.task.clone());
    let on_update = props.on_update.clone();

    let title_input = use_state(|| task.title.clone());
    let editing = use_state(|| false);

    // MARK DONE HANDLER
    let onclick_mark_done = {
        let task = task.clone();
        let on_update = on_update.clone();
        Callback::from(move |_| {
            let task = task.clone();
            let on_update = on_update.clone();
            spawn_local(async move {
                if !task.done {
                    let config = config();
                    if let Ok(updated_task) =
                        tasks_api::mark_task_done(&config, task.id as i32).await
                    {
                        task.set(updated_task.clone());
                        on_update.emit(updated_task);
                    }
                }
            });
        })
    };

    // START EDIT HANDLER
    let onclick_edit = {
        let editing = editing.clone();
        Callback::from(move |_| editing.set(true))
    };

    // SAVE NAME HANDLER
    let onclick_save_name = {
        let task = task.clone();
        let title_input = title_input.clone();
        let on_update = on_update.clone();
        let editing = editing.clone();
        Callback::from(move |_| {
            let task = task.clone();
            let title = (*title_input).clone();
            let on_update = on_update.clone();
            let editing = editing.clone();

            spawn_local(async move {
                let config = config();
                let body = RenameTask {
                    title: title.clone(),
                };
                if let Ok(updated_task) =
                    tasks_api::rename_task(&config, task.id as i32, body).await
                {
                    task.set(updated_task.clone());
                    on_update.emit(updated_task);
                    editing.set(false);
                }
            });
        })
    };

    let oninput_name = {
        let title_input = title_input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                title_input.set(input.value());
            }
        })
    };

    let done_style = if task.done {
        "line-through text-gray-500 transition-opacity duration-200 opacity-70"
    } else {
        "transition-opacity duration-200 opacity-100"
    };

    html! {
        <li class="flex justify-between items-center gap-2 px-2 py-1 hover:bg-gray-50 rounded transition-colors">
            <div class="flex-1">
                {
                    if *editing {
                        html! {
                            <input
                                class={classes!("border", "rounded", "px-2", "py-1", "w-full")}
                                value={(*title_input).clone()}
                                oninput={oninput_name}
                            />
                        }
                    } else {
                        html! {
                            <span class={classes!(done_style)}>{ &task.title }</span>
                        }
                    }
                }
            </div>

            <div class="flex gap-2 items-center">
                {
                    if !*editing && !task.done {
                        html! {
                            <button class="bg-blue-500 hover:bg-blue-600 text-white text-sm px-3 py-1 rounded-md transition-all duration-200 flex items-center gap-1"
                                onclick={onclick_edit}
                                title="Edit task">
                                <span>{ "✎" }</span>
                            </button>
                        }
                    } else if *editing {
                        html! {
                            <button
                                class="text-sm bg-blue-500 hover:bg-blue-600 text-white px-3 py-1 rounded-md transition-all duration-200"
                                onclick={onclick_save_name}
                            >
                                { "Save" }
                            </button>
                        }
                    } else {
                        html! {}
                    }
                }

                {
                    if !task.done {
                        html! {
                            <button
                                class="text-sm bg-green-500 hover:bg-green-600 text-white px-3 py-1 rounded-md transition-all duration-200"
                                onclick={onclick_mark_done}
                            >
                                { "Mark done" }
                            </button>
                        }
                    } else {
                        html! {
                            <button class="text-sm px-3 py-1 rounded-md opacity-0 pointer-events-none" disabled=true>
                                { "Done" }
                            </button>
                        }
                    }
                }
            </div>
        </li>
    }
}
