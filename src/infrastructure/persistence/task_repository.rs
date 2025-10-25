use crate::domain::{task, task::Entity as Task};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, NotSet};

#[derive(Clone)]
pub struct TaskRepository {
    db: DatabaseConnection,
}

impl TaskRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn save(&self, task: &task::ActiveModel) -> anyhow::Result<task::Model> {
        let mut active = task.clone().into_active_model();
        if active.clone().id.unwrap() == 0 {
            active.id = NotSet;
            Ok(active.insert(&self.db).await?)
        } else {
            Ok(active.update(&self.db).await?)
        }
    }

    pub async fn find_all(&self) -> anyhow::Result<Vec<task::Model>> {
        Ok(Task::find().all(&self.db).await?)
    }

    pub async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<task::Model>> {
        Ok(Task::find_by_id(id).one(&self.db).await?)
    }
}
