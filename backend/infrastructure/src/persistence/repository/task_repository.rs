use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, NotSet};
use domain::task::Task;
use application::task::task_repository_trait::TaskRepositoryTrait;
use crate::persistence::entity::task::{Entity as TaskEntity, Model as TaskModel};

#[derive(Clone)]
pub struct TaskRepository {
    db: DatabaseConnection,
}

impl TaskRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TaskRepositoryTrait for TaskRepository {
    async fn save(&self, task: Task) -> anyhow::Result<Task> {
        let model: TaskModel = task.into();
        let mut active_model = model.clone().into_active_model();
        if model.id == 0 {
            active_model.id = NotSet;
            Ok(active_model.insert(&self.db).await?.into())
        } else {
            let active_model = active_model.reset_all();
            Ok(active_model.update(&self.db).await?.into())
        }
    }

    async fn find_all(&self) -> anyhow::Result<Vec<Task>> {
        Ok(TaskEntity::find().all(&self.db).await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<Task>> {
        Ok(TaskEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .map(Into::into))
    }
}
