use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250301_091844_create_table_endpoint::Endpoint;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Request::Table)
                    .if_not_exists()
                    .col(pk_auto(Request::Id))
                    .col(string(Request::Name))
                    .col(text(Request::Body))
                    .col(integer(Request::SentStatus))
                    .col(date_time(Request::SentAt))
                    .col(integer(Request::EndpointId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-request-endpoint")
                            .from(Request::Table, Request::EndpointId)
                            .to(Endpoint::Table, Endpoint::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(date_time(Request::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time(Request::UpdatedAt).default(Expr::current_timestamp()))
                    .col(date_time_null(Request::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Request::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Request {
    Table,
    Id,
    EndpointId,
    Name,
    Body,
    SentStatus,
    SentAt,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
