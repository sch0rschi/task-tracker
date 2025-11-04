use crate::api_config::config;
use crate::components::utils::bind_input;
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
    let TaskItemProps {
        task: task_prop,
        on_update,
    } = props;
    let task = use_state(|| task_prop.clone());
    let on_update = on_update.clone();
    let title_input = use_state(|| task.title.clone());
    let editing = use_state(|| false);

    {
        let task_for_closure = task.clone();
        use_effect_with(props.task.clone(), move |new_task| {
            task_for_closure.clone().set(new_task.clone());
            || ()
        });
    }

    let onclick_edit = {
        let editing_for_closure = editing.clone();
        let title_input_for_closure = title_input.clone();
        let title_for_closure = task.title.clone();
        Callback::from(move |_| {
            title_input_for_closure.set(title_for_closure.clone());
            editing_for_closure.set(true);
        })
    };

    let onclick_save_name = {
        let task_for_closure = task.clone();
        let title_input_for_closure = title_input.clone();
        let on_update_for_closure = on_update.clone();
        let editing_for_closure = editing.clone();
        Callback::from(move |_| {
            let task = task_for_closure.clone();
            let title_input = title_input_for_closure.clone();
            let on_update = on_update_for_closure.clone();
            let editing = editing_for_closure.clone();

            spawn_local(async move {
                let config = config();
                let body = RenameTask {
                    title: (*title_input).clone(),
                };
                if let Ok(updated_task) = tasks_api::rename_task(&config, task.id, body).await {
                    task.set(updated_task.clone());
                    on_update.emit(updated_task);
                    editing.set(false);
                }
            });
        })
    };

    let onclick_mark_done = {
        let task_for_closure = task.clone();
        let on_update_for_closure = on_update.clone();
        let editing_for_closure = editing.clone();
        Callback::from(move |_| {
            let task = task_for_closure.clone();
            let on_update = on_update_for_closure.clone();
            let editing = editing_for_closure.clone();
            spawn_local(async move {
                let config = config();
                let current_task = (*task).clone();
                task.set(current_task.clone());
                editing.set(false);

                if let Ok(done_task) = tasks_api::mark_task_done(&config, current_task.id).await {
                    task.set(done_task.clone());
                    on_update.emit(done_task);
                }
            });
        })
    };

    let oninput_title = bind_input(title_input.clone());

    html! {
        <li class="flex justify-between items-center gap-2 px-2 py-1 hover:bg-gray-50 rounded transition-colors h-10">
            <div class="flex justify-between items-center w-full gap-2">
                {
                    if *editing {
                        html! {
                            <input
                                class="border rounded flex-1 px-1 py-1 duration-200"
                                value={(*title_input).clone()}
                                oninput={oninput_title}
                            />
                        }
                    } else {
                        html! {
                            <span class={classes!(
                                task.done.then_some("line-through text-gray-500"),
                                "transition-opacity", "duration-200", "opacity-100",
                                "flex-1", "px-1", "py-1", "border", "border-transparent", "rounded"
                            )}>
                                { &task.title }
                            </span>
                        }
                    }
                }
                <button
                    type="button"
                    title={if *editing {"Save changes"} else {"Edit title"}}
                    onclick={if *editing {onclick_save_name} else {onclick_edit}}
                    class={classes!(
                        task.done.then_some("hidden"),
                        if *editing {"bg-green-500"} else {"bg-blue-500"},
                        if *editing {"hover:bg-green-600"} else {"hover:bg-blue-600"},
                        "flex", "px-3", "py-1", "rounded-md", "items-center", "gap-1", "duration-200",
                    )}>
                    {if *editing {"💾"} else {"✏️"}}
                </button>
                <button
                    type="button"
                    class={classes!(
                        task.done.then_some("hidden"),
                        if *editing {"bg-green-500"} else {"bg-blue-500"},
                        if *editing {"hover:bg-green-600"} else {"hover:bg-blue-600"},
                        "bg-green-500", "hover:bg-green-600", "disabled:bg-gray-400", "disabled:hover:bg-gray-500", "disabled:cursor-not-allowed",
                        "px-3", "py-1", "rounded-md", "duration-200"
                    )}
                    onclick={onclick_mark_done}
                    title="Mark task as done"
                    disabled={*editing}>
                    {"✔"}
                </button>
            </div>
        </li>
    }
}
