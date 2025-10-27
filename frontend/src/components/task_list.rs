use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use api_client::apis::tasks_api;
use api_client::models::{Task, NewTask};
use crate::api_config::config;

#[function_component(TaskList)]
pub fn task_list() -> Html {
    let tasks = use_state(Vec::<Task>::new);
    let title = use_state(String::new);

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

    let oninput = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                title.set(input.value());
            }
        })
    };

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
        <div class="p-4 max-w-md mx-auto">
            <h1 class="text-2xl font-bold mb-4">{ "Task Tracker" }</h1>

            <div class="flex mb-4">
                <input
                    class="border rounded px-2 py-1 flex-grow"
                    placeholder="New task title"
                    value={(*title).clone()}
                    oninput={oninput}
                />
                <button class="ml-2 px-4 py-1 bg-blue-600 text-white rounded" onclick={onclick_add}>
                    { "Add" }
                </button>
            </div>

            <ul>
                { for (*tasks).iter().map(|task| {
                    let done_style = if task.done { "line-through text-gray-500" } else { "" };
                    let id = task.id;
                    let onclick = {
                        let on_mark_done = on_mark_done.clone();
                        Callback::from(move |_| on_mark_done.emit(id))
                    };
                    html! {
                        <li class="flex justify-between items-center mb-2">
                            <span class={classes!(done_style)}>{ &task.title }</span>
                            { if !task.done {
                                html! {
                                    <button class="text-sm bg-green-500 text-white px-2 py-1 rounded" {onclick}>
                                        { "Mark done" }
                                    </button>
                                }
                            } else {
                                html! {}
                            }}
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}
