use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct TaskFilterAndSortDto {
    pub filters: Option<TaskFilterAndSortFiltersDto>,
    pub sort: Option<TaskFilterAndSortSortDto>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaskFilterAndSortFiltersDto {
    pub done: Option<bool>,
    pub title: Option<String>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub updated_after: Option<DateTime<Utc>>,
    pub updated_before: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TaskFilterAndSortSortDto {
    pub field: Option<TaskFilterAndSortSortFieldDto>,
    pub direction: Option<TaskFilterAndSortSortDirectionDto>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskFilterAndSortSortFieldDto {
    CreatedAt,
    UpdatedAt,
    Title,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskFilterAndSortSortDirectionDto {
    Asc,
    Desc,
}
