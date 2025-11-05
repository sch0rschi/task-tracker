use async_trait::async_trait;
use domain::task::Task;
use crate::task::task_filter_and_sort_dto::TaskFilterAndSortDto;

#[async_trait]
pub trait TaskRepositoryTrait: Send + Sync {
    async fn save(&self, task: Task) -> anyhow::Result<Task>;
    async fn find_tasks(&self, filter_and_sort: TaskFilterAndSortDto) -> anyhow::Result<Vec<Task>>;
    async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<Task>>;
}
