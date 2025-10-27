use o2o::o2o;
use domain::task::Task;
use crate::persistence::entity::task::Model as TaskPersistenceModel;
use openapi_client::models::Task as TaskApiModel;

#[derive(Debug, Clone, o2o)]
#[map_owned(Task)]
#[from_owned(TaskPersistenceModel)]
#[owned_into(TaskPersistenceModel)]
#[from_owned(TaskApiModel)]
#[owned_into(TaskApiModel)]
pub struct TaskMapper {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

impl From<Task> for TaskPersistenceModel {
    fn from(task: Task) -> TaskPersistenceModel {
        let task_mapper: TaskMapper  = task.into();
        task_mapper.into()
    }
}

impl From<TaskPersistenceModel> for Task {
    fn from(task: TaskPersistenceModel) -> Task {
        let task_mapper: TaskMapper  = task.into();
        task_mapper.into()
    }
}

pub trait ToApiModel {
    fn to_api_model(self) -> TaskApiModel;
}

impl ToApiModel for Task {
    fn to_api_model(self) -> TaskApiModel {
        let task_mapper: TaskMapper = self.into();
        task_mapper.into()
    }
}
