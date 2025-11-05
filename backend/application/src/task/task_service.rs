use crate::task::task_repository_trait::TaskRepositoryTrait;
use crate::task::task_service_trait::TaskServiceTrait;
use async_trait::async_trait;
use domain::task::Task;
use std::sync::Arc;
use crate::task::task_filter_and_sort_dto::TaskFilterAndSortDto;

#[derive(Clone)]
pub struct TaskService {
    task_repository: Arc<dyn TaskRepositoryTrait>,
}

impl TaskService {
    pub fn new(task_repository: Arc<dyn TaskRepositoryTrait>) -> Self {
        Self { task_repository }
    }
}

#[async_trait]
impl TaskServiceTrait for TaskService {
    async fn create_task(&self, title: &str) -> anyhow::Result<Task> {
        let new_task = Task {
            title: title.to_string(),
            ..Default::default()
        };
        self.task_repository.save(new_task).await
    }

    async fn mark_done(&self, id: i64) -> anyhow::Result<Option<Task>> {
        if let Some(mut task) = self.task_repository.find_by_id(id).await? {
            task.done = true;
            let updated = self.task_repository.save(task).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    async fn rename_task(&self, id: i64, new_title: String) -> anyhow::Result<Option<Task>> {
        if let Some(mut task) = self.task_repository.find_by_id(id).await? {
            task.title = new_title;
            let updated = self.task_repository.save(task).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    async fn get_task(&self, id: i64) -> anyhow::Result<Option<Task>> {
        self.task_repository.find_by_id(id).await
    }

    async fn find_tasks(&self, filter_and_sort: TaskFilterAndSortDto) -> anyhow::Result<Vec<Task>> {
        self.task_repository.find_tasks(filter_and_sort).await
    }
}
