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
                    .table(RequestHeader::Table)
                    .if_not_exists()
                    .col(pk_auto(RequestHeader::Id))
                    .col(string(RequestHeader::Key))
                    .col(text(RequestHeader::Value))
                    .col(integer(RequestHeader::RequestId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-request-header-request")
                            .from(RequestHeader::Table, RequestHeader::RequestId)
                            .to(Request::Table, Request::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(date_time(RequestHeader::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time(RequestHeader::UpdatedAt).default(Expr::current_timestamp()))
                    .col(date_time_null(RequestHeader::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RequestHeader::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RequestHeader {
    Table,
    Id,
    RequestId,
    Key,
    Value,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
