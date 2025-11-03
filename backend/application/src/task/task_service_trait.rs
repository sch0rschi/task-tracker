use domain::task::Task;
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait TaskServiceTrait: Send + Sync {
    async fn create_task(&self, title: &str) -> Result<Task>;
    async fn mark_done(&self, id: i64) -> Result<Option<Task>>;
    async fn rename_task(&self, id: i64, new_title: String) -> Result<Option<Task>>;
    async fn get_task(&self, id: i64) -> Result<Option<Task>>;
    async fn list_tasks(&self) -> Result<Vec<Task>>;
}
