use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use api_client::apis::tasks_api;
use api_client::models::{Task, NewTask};
use crate::api_config::config;

#[function_component(TaskList)]
pub fn task_list() -> Html {
    let tasks = use_state(Vec::<Task>::new);
    let title = use_state(String::new);

    // Fetch tasks on mount
    {
        let tasks = tasks.clone();
        use_effect_with((), move |_| {
            let tasks = tasks.clone();
            spawn_local(async move {
                let config = config();
                if let Ok(fetched) = tasks_api::list_tasks(&config).await {
                    tasks.set(fetched);
                }
            });
            || ()
        });
    }

    // Input handler
    let oninput = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                title.set(input.value());
            }
        })
    };

    // Add new task
    let onclick_add = {
        let title = title.clone();
        let tasks = tasks.clone();
        Callback::from(move |_| {
            let title_val = (*title).clone();
            let tasks = tasks.clone();
            spawn_local(async move {
                if !title_val.is_empty() {
                    let config = config();
                    let new_task_req = NewTask { title: title_val.clone() };
                    if let Ok(new_task) = tasks_api::create_task(&config, new_task_req).await {
                        let mut new_list = (*tasks).clone();
                        new_list.push(new_task);
                        tasks.set(new_list);
                    }
                }
            });
        })
    };

    // Mark task done
    let on_mark_done = {
        let tasks = tasks.clone();
        Callback::from(move |id: i64| {
            let tasks = tasks.clone();
            spawn_local(async move {
                let config = config();
                if let Ok(updated_task) = tasks_api::mark_task_done(&config, id as i32).await {
                    let new_tasks = (*tasks)
                        .iter()
                        .map(|t| if t.id == id { updated_task.clone() } else { t.clone() })
                        .collect();
                    tasks.set(new_tasks);
                }
            });
        })
    };

    html! {
        <div class="p-6 max-w-md mx-auto">
            <h1 class="text-2xl font-bold mb-4 text-gray-800 text-center">{ "Task Tracker" }</h1>

            <div class="flex mb-6">
                <input
                    class="border border-gray-300 rounded px-3 py-2 flex-grow focus:outline-none focus:ring-2 focus:ring-blue-500"
                    placeholder="New task title"
                    value={(*title).clone()}
                    oninput={oninput}
                />
                <button
                    class="ml-3 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded transition-colors"
                    onclick={onclick_add}
                >
                    { "Add" }
                </button>
            </div>

            <ul class="space-y-2">
                { for (*tasks).iter().map(|task| {
                    let done_style = if task.done {
                        "line-through text-gray-500 transition-opacity duration-200 opacity-70"
                    } else {
                        "transition-opacity duration-200 opacity-100"
                    };

                    let id = task.id;
                    let onclick = {
                        let on_mark_done = on_mark_done.clone();
                        Callback::from(move |_| on_mark_done.emit(id))
                    };

                    html! {
                        <li class="flex justify-between items-center px-2 py-1 hover:bg-gray-50 rounded transition-colors">
                            <span class={classes!(done_style, "truncate")}>{ &task.title }</span>
                            {
                                if !task.done {
                                    html! {
                                        <button
                                            class="text-sm bg-green-500 hover:bg-green-600 text-white px-2 py-1 rounded transition-all duration-200"
                                            {onclick}
                                        >
                                            { "Mark done" }
                                        </button>
                                    }
                                } else {
                                    html! {
                                        // Invisible placeholder to preserve layout height
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
                })}
            </ul>
        </div>
    }
}
