use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250301_093349_create_table_request::Request;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Response::Table)
                    .if_not_exists()
                    .col(pk_auto(Response::Id))
                    .col(integer(Response::RequestId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-response-request")
                            .from(Response::Table, Response::RequestId)
                            .to(Request::Table, Request::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Response::Status))
                    .col(text(Response::Body))
                    .col(date_time(Response::ReceivedAt))
                    .col(date_time(Response::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time(Response::UpdatedAt).default(Expr::current_timestamp()))
                    .col(date_time_null(Response::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Response::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Response {
    Table,
    Id,
    RequestId,
    Status,
    Body,
    ReceivedAt,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
