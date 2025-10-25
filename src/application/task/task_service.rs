use sea_orm::{IntoActiveModel, Set};
use crate::domain::task;
use crate::infrastructure::persistence::task_repository;

#[derive(Clone)]
pub struct TaskService {
    repository: task_repository::TaskRepository,
}

impl TaskService {
    pub fn new(repository: task_repository::TaskRepository) -> Self {
        Self { repository }
    }

    pub async fn create_task(&self, title: &str) -> anyhow::Result<task::Model> {
        let new_task = task::Model {
            title: title.to_string(),
            ..Default::default()
        };
        self.repository.save(&new_task.into_active_model()).await
    }

    pub async fn mark_done(&self, id: i64) -> anyhow::Result<Option<task::Model>> {
        if let Some(task) = self.repository.find_by_id(id).await? {
            let mut task = task.into_active_model();
            task.done = Set(true);
            let updated = self.repository.save(&task).await?;
            Ok(Some(updated))
        } else {
            Ok(None)
        }
    }

    pub async fn get_task(&self, id: i64) -> anyhow::Result<Option<task::Model>> {
        self.repository.find_by_id(id).await
    }

    pub async fn list_tasks(&self) -> anyhow::Result<Vec<task::Model>> {
        self.repository.find_all().await
    }
}
