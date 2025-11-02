use crate::task::task_repository_trait::{TaskRepositoryTrait};
use domain::task::Task;
use std::sync::Arc;

#[derive(Clone)]
pub struct TaskService {
    task_repository: Arc<dyn TaskRepositoryTrait>,
}

impl TaskService {
    pub fn new(repository: impl TaskRepositoryTrait + 'static) -> Self {
        Self {
            task_repository: Arc::new(repository),
        }
    }

    pub async fn create_task(&self, title: &str) -> anyhow::Result<Task> {
        let new_task = Task {
            title: title.to_string(),
            ..Default::default()
        };
        self.task_repository.save(new_task).await
    }

    pub async fn mark_done(&self, id: i64) -> anyhow::Result<Option<Task>> {
        if let Some(mut task) = self.task_repository.find_by_id(id).await? {
            task.done = true;
            let updated = self.task_repository.save(task).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    pub async fn rename_task(&self, id: i64, new_title: String) -> anyhow::Result<Option<Task>> {
        if let Some(mut task) = self.task_repository.find_by_id(id).await? {
            task.title = new_title;
            let updated = self.task_repository.save(task).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    pub async fn get_task(&self, id: i64) -> anyhow::Result<Option<Task>> {
        self.task_repository.find_by_id(id).await
    }

    pub async fn list_tasks(&self) -> anyhow::Result<Vec<Task>> {
        self.task_repository.find_all().await
    }
}
