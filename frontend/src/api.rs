use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewTask {
    pub title: String,
}

const API_URL: &str = "http://127.0.0.1:8080";

pub async fn list_tasks() -> Result<Vec<Task>, String> {
    Request::get(&format!("{API_URL}/tasks"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<Task>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_task(new_task: &NewTask) -> Result<Task, String> {
    Request::post(&format!("{API_URL}/tasks"))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(new_task).unwrap())
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Task>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn mark_task_done(id: i64) -> Result<Task, String> {
    Request::put(&format!("{API_URL}/tasks/{id}/done"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<Task>()
        .await
        .map_err(|e| e.to_string())
}
