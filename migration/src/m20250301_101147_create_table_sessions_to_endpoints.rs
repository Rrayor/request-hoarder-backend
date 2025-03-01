use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250301_100946_create_table_session::Session;
use crate::m20250301_091844_create_table_endpoint::Endpoint;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(SessionsToEndpoints::Table)
                    .if_not_exists()
                    .primary_key(
                        Index::create()
                            .table(SessionsToEndpoints::Table)
                            .col(SessionsToEndpoints::SessionId)
                            .col(SessionsToEndpoints::EndpointId),
                    )
                    .col(integer(SessionsToEndpoints::SessionId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sessions-to-ids-session")
                            .from(SessionsToEndpoints::Table, SessionsToEndpoints::SessionId)
                            .to(Session::Table, Session::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(integer(SessionsToEndpoints::EndpointId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sessions-to-ids-endpoint")
                            .from(SessionsToEndpoints::Table, SessionsToEndpoints::EndpointId)
                            .to(Endpoint::Table, Endpoint::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SessionsToEndpoints::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SessionsToEndpoints {
    Table,
    SessionId,
    EndpointId,
}
