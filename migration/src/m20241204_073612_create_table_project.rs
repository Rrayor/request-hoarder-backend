use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(pk_auto(Project::Id))
                    .col(string(Project::Name).not_null())
                    .col(string_null(Project::Description))
                    .col(date_time(Project::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time(Project::UpdatedAt).default(Expr::current_timestamp()))
                    .col(date_time_null(Project::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Project {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
