use async_trait::async_trait;
use domain::task::Task;

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn save(&self, task: Task) -> anyhow::Result<Task>;
    async fn find_all(&self) -> anyhow::Result<Vec<Task>>;
    async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<Task>>;
}
