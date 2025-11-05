use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tasks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tasks::Id)
                            .primary_key()
                            .big_integer()
                            .not_null()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Tasks::Title).string().not_null())
                    .col(ColumnDef::new(Tasks::Done).boolean().not_null())
                    .col(ColumnDef::new(Tasks::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Tasks::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tasks::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Tasks {
    Table,
    Id,
    Title,
    Done,
    CreatedAt,
    UpdatedAt,
}
