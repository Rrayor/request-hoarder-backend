use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250105_123300_create_table_group::Group;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Endpoint::Table)
                    .if_not_exists()
                    .col(pk_auto(Endpoint::Id))
                    .col(string(Endpoint::Name))
                    .col(string(Endpoint::Url))
                    .col(string(Endpoint::Method))
                    .col(integer(Endpoint::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-endpoint-group")
                            .from(Endpoint::Table, Endpoint::GroupId)
                            .to(Group::Table, Group::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(date_time(Endpoint::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time(Endpoint::UpdatedAt).default(Expr::current_timestamp()))
                    .col(date_time_null(Endpoint::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Endpoint::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Endpoint {
    Table,
    Id,
    Name,
    Url,
    Method,
    GroupId,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
