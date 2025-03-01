use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241204_073612_create_table_project::Project;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(pk_auto(Group::Id))
                    .col(string(Group::Name).not_null())
                    .col(string_null(Group::Description))
                    .col(integer(Group::ProjectId))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-group-project")
                            .from(Group::Table, Group::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(date_time(Group::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time(Group::UpdatedAt).default(Expr::current_timestamp()))
                    .col(date_time_null(Group::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Group {
    Table,
    Id,
    Name,
    Description,
    ProjectId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
