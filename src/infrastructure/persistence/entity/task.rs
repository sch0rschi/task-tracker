use o2o::o2o;
use sea_orm::prelude::*;
use crate::domain::task::Task;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, o2o, Default)]
#[sea_orm(table_name = "tasks")]
#[from_owned(Task)]
#[owned_into(Task)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub title: String,
    pub done: bool,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
