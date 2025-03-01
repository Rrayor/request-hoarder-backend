use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250301_094714_create_table_response::Response;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(ResponseHeader::Table)
                    .if_not_exists()
                    .col(pk_auto(ResponseHeader::Id))
                    .col(string(ResponseHeader::Key))
                    .col(text(ResponseHeader::Value))
                    .col(integer(ResponseHeader::ResponseId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-response-header-response")
                            .from(ResponseHeader::Table, ResponseHeader::ResponseId)
                            .to(Response::Table, Response::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                    )
                    .col(date_time(ResponseHeader::CreatedAt).default(Expr::current_timestamp()))
                    .col(date_time(ResponseHeader::UpdatedAt).default(Expr::current_timestamp()))
                    .col(date_time_null(ResponseHeader::DeletedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ResponseHeader::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ResponseHeader {
    Table,
    Id,
    ResponseId,
    Key,
    Value,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
