use crate::persistence::entity::task::{Column, Entity as TaskEntity, Model as TaskModel};
use application::task::task_filter_and_sort_dto::{TaskFilterAndSortDto, TaskFilterAndSortSortDirectionDto, TaskFilterAndSortSortFieldDto};
use application::task::task_repository_trait::TaskRepositoryTrait;
use async_trait::async_trait;
use chrono::Utc;
use domain::task::Task;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, NotSet};
use sea_orm::{ColumnTrait, Condition, QueryFilter, QueryOrder};
use std::sync::Arc;

#[derive(Clone)]
pub struct TaskRepository {
    database_connection: Arc<DatabaseConnection>,
}

impl TaskRepository {
    pub fn new(database_connection: Arc<DatabaseConnection>) -> Self {
        Self { database_connection }
    }
}

#[async_trait]
impl TaskRepositoryTrait for TaskRepository {
    async fn save(&self, task: Task) -> anyhow::Result<Task> {
        let model: TaskModel = task.into();
        let mut active_model = model.clone().into_active_model();
        if model.id == 0 {
            active_model.id = NotSet;
            active_model.created_at = NotSet;
            active_model.updated_at = NotSet;
            Ok(active_model.insert(&*self.database_connection).await?.into())
        } else {
            active_model.updated_at.set_if_not_equals(Utc::now());
            let active_model = active_model.reset_all();
            Ok(active_model.update(&*self.database_connection).await?.into())
        }
    }

    async fn find_tasks(&self, filter_and_sort: TaskFilterAndSortDto) -> anyhow::Result<Vec<Task>> {
        let mut query = TaskEntity::find();

        if let Some(filters) = filter_and_sort.filters {
            let mut condition = Condition::all();

            if let Some(done) = filters.done {
                condition = condition.add(Column::Done.eq(done));
            }

            if let Some(title) = filters.title {
                // case-insensitive substring match
                condition = condition.add(Column::Title.contains(title));
            }

            if let Some(created_after) = filters.created_after {
                condition = condition.add(Column::CreatedAt.gt(created_after));
            }

            if let Some(created_before) = filters.created_before {
                condition = condition.add(Column::CreatedAt.lt(created_before));
            }

            if let Some(updated_after) = filters.updated_after {
                condition = condition.add(Column::UpdatedAt.gt(updated_after));
            }

            if let Some(updated_before) = filters.updated_before {
                condition = condition.add(Column::UpdatedAt.lt(updated_before));
            }

            query = query.filter(condition);
        }

        if let Some(sort) = filter_and_sort.sort {
            if let Some(field) = sort.field {
                let ascending = matches!(sort.direction, Some(TaskFilterAndSortSortDirectionDto::Asc));

                query = match field {
                    TaskFilterAndSortSortFieldDto::CreatedAt => {
                        if ascending {
                            query.order_by_asc(Column::CreatedAt)
                        } else {
                            query.order_by_desc(Column::CreatedAt)
                        }
                    }
                    TaskFilterAndSortSortFieldDto::UpdatedAt => {
                        if ascending {
                            query.order_by_asc(Column::UpdatedAt)
                        } else {
                            query.order_by_desc(Column::UpdatedAt)
                        }
                    }
                    TaskFilterAndSortSortFieldDto::Title => {
                        if ascending {
                            query.order_by_asc(Column::Title)
                        } else {
                            query.order_by_desc(Column::Title)
                        }
                    }
                    TaskFilterAndSortSortFieldDto::Done => {
                        if ascending {
                            query.order_by_asc(Column::Done)
                        } else {
                            query.order_by_desc(Column::Done)
                        }
                    }
                };
            }
        }

        let tasks = query.all(&*self.database_connection).await?;
        Ok(tasks.into_iter().map(Into::into).collect())
    }

    async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<Task>> {
        Ok(TaskEntity::find_by_id(id)
            .one(&*self.database_connection)
            .await?
            .map(Into::into))
    }
}
