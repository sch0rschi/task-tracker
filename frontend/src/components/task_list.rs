use crate::api_config::config;
use crate::components::task_item::TaskItem;
use crate::components::utils::bind_input;
use api_client::apis::tasks_api;
use api_client::models;
use api_client::models::{NewTask, Task};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(TaskList)]
pub fn task_list() -> Html {
    let tasks = use_state(Vec::<Task>::new);
    let title = use_state(String::new);

    // Fetch on mount
    {
        let tasks = tasks.clone();
        use_effect_with((), move |_| {
            let tasks = tasks.clone();
            spawn_local(async move {
                let config = config();
                let task_filter_and_sort = models::TaskFilterAndSort::default();
                if let Ok(fetched) = tasks_api::filter_tasks(&config, task_filter_and_sort).await {
                    tasks.set(fetched);
                }
            });
            || ()
        });
    }

    let oninput_title = bind_input(title.clone());

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
                    let new_task_req = NewTask { title: title_val };
                    if let Ok(new_task) = tasks_api::create_task(&config, new_task_req).await {
                        let mut new_list = (*tasks).clone();
                        new_list.push(new_task);
                        tasks.set(new_list);
                    }
                }
            });
        })
    };

    // When a task updates (e.g., marked done)
    let on_task_update = {
        let tasks = tasks.clone();
        Callback::from(move |updated: Task| {
            let new_tasks = (*tasks)
                .iter()
                .map(|t| {
                    if t.id == updated.id {
                        updated.clone()
                    } else {
                        t.clone()
                    }
                })
                .collect();
            tasks.set(new_tasks);
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
                    oninput={oninput_title}
                />
                <button
                    class="ml-3 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded transition-colors"
                    onclick={onclick_add}
                >
                    { "Add" }
                </button>
            </div>

            <ul class="space-y-2">
                { for (*tasks).iter().map(|task| html! {
                    <TaskItem task={task.clone()} on_update={on_task_update.clone()} />
                })}
            </ul>
        </div>
    }
}
