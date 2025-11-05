use chrono::{DateTime, Utc};
use o2o::o2o;

use openapi_client::models::{
    TaskFilterAndSort as ApiTaskFilterAndSort,
    TaskFilterAndSortFilters as ApiTaskFilterAndSortFilters,
    TaskFilterAndSortSort as ApiTaskFilterAndSortSort,
    TaskFilterAndSortSortField as ApiTaskFilterAndSortSortField,
    TaskFilterAndSortSortDirection as ApiTaskFilterAndSortSortDirection,
};

use application::task::task_filter_and_sort_dto::{
    TaskFilterAndSortDto,
    TaskFilterAndSortFiltersDto,
    TaskFilterAndSortSortDto,
    TaskFilterAndSortSortFieldDto,
    TaskFilterAndSortSortDirectionDto,
};

#[derive(Debug, Clone)]
pub struct TaskFilterAndSortMapper {
    pub filters: Option<TaskFilterAndSortFiltersMapper>,
    pub sort: Option<TaskFilterAndSortSortMapper>,
}

#[derive(Debug, Clone, o2o)]
#[owned_into(TaskFilterAndSortFiltersDto)]
pub struct TaskFilterAndSortFiltersMapper {
    pub done: Option<bool>,
    pub title: Option<String>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub updated_after: Option<DateTime<Utc>>,
    pub updated_before: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct TaskFilterAndSortSortMapper {
    pub field: Option<TaskFilterAndSortSortFieldMapper>,
    pub direction: Option<TaskFilterAndSortSortDirectionMapper>,
}

#[derive(Debug, Clone, Copy, o2o)]
#[owned_into(TaskFilterAndSortSortFieldDto)]
pub enum TaskFilterAndSortSortFieldMapper {
    CreatedAt,
    UpdatedAt,
    Title,
    Done,
}

#[derive(Debug, Clone, Copy, o2o)]
#[owned_into(TaskFilterAndSortSortDirectionDto)]
pub enum TaskFilterAndSortSortDirectionMapper {
    Asc,
    Desc,
}

impl From<ApiTaskFilterAndSort> for TaskFilterAndSortMapper {
    fn from(api: ApiTaskFilterAndSort) -> Self {
        Self {
            filters: api.filters.map(TaskFilterAndSortFiltersMapper::from),
            sort: api.sort.map(TaskFilterAndSortSortMapper::from),
        }
    }
}

impl From<ApiTaskFilterAndSortFilters> for TaskFilterAndSortFiltersMapper {
    fn from(api: ApiTaskFilterAndSortFilters) -> Self {
        Self {
            done: api.done,
            title: api.title,
            created_after: api.created_after,
            created_before: api.created_before,
            updated_after: api.updated_after,
            updated_before: api.updated_before,
        }
    }
}

impl From<ApiTaskFilterAndSortSort> for TaskFilterAndSortSortMapper {
    fn from(api: ApiTaskFilterAndSortSort) -> Self {
        Self {
            field: api.field.map(TaskFilterAndSortSortFieldMapper::from),
            direction: api.direction.map(TaskFilterAndSortSortDirectionMapper::from),
        }
    }
}

impl From<ApiTaskFilterAndSortSortField> for TaskFilterAndSortSortFieldMapper {
    fn from(api: ApiTaskFilterAndSortSortField) -> Self {
        match api {
            ApiTaskFilterAndSortSortField::CreatedAt => Self::CreatedAt,
            ApiTaskFilterAndSortSortField::UpdatedAt => Self::UpdatedAt,
            ApiTaskFilterAndSortSortField::Title => Self::Title,
            ApiTaskFilterAndSortSortField::Done => Self::Done,
        }
    }
}

impl From<ApiTaskFilterAndSortSortDirection> for TaskFilterAndSortSortDirectionMapper {
    fn from(api: ApiTaskFilterAndSortSortDirection) -> Self {
        match api {
            ApiTaskFilterAndSortSortDirection::Asc => Self::Asc,
            ApiTaskFilterAndSortSortDirection::Desc => Self::Desc,
        }
    }
}

impl From<TaskFilterAndSortMapper> for TaskFilterAndSortDto {
    fn from(mapper: TaskFilterAndSortMapper) -> Self {
        TaskFilterAndSortDto {
            filters: mapper.filters.map(Into::into),
            sort: mapper.sort.map(Into::into),
        }
    }
}

impl From<TaskFilterAndSortSortMapper> for TaskFilterAndSortSortDto {
    fn from(mapper: TaskFilterAndSortSortMapper) -> Self {
        TaskFilterAndSortSortDto {
            field: mapper.field.map(Into::into),
            direction: mapper.direction.map(Into::into),
        }
    }
}

impl TaskFilterAndSortMapper {
    pub fn from_api(api: ApiTaskFilterAndSort) -> Self {
        api.into()
    }

    pub fn into_dto(self) -> TaskFilterAndSortDto {
        self.into()
    }
}

pub trait ToTaskFilterAndSortDto {
    fn to_dto(self) -> TaskFilterAndSortDto;
}

impl ToTaskFilterAndSortDto for ApiTaskFilterAndSort {
    fn to_dto(self) -> TaskFilterAndSortDto {
        let mapper: TaskFilterAndSortMapper = self.into();
        mapper.into()
    }
}