pub use sea_orm_migration::prelude::*;

mod m20241204_073612_create_table_project;
mod m20250105_123300_create_table_group;
mod m20250301_091844_create_table_endpoint;
mod m20250301_093349_create_table_request;
mod m20250301_093639_create_table_request_header;
mod m20250301_094654_create_table_response_header;
mod m20250301_094714_create_table_response;
mod m20250301_100946_create_table_session;
mod m20250301_101147_create_table_sessions_to_endpoints;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241204_073612_create_table_project::Migration),
            Box::new(m20250105_123300_create_table_group::Migration),
            Box::new(m20250301_091844_create_table_endpoint::Migration),
            Box::new(m20250301_093349_create_table_request::Migration),
            Box::new(m20250301_093639_create_table_request_header::Migration),
            Box::new(m20250301_094654_create_table_response_header::Migration),
            Box::new(m20250301_094714_create_table_response::Migration),
            Box::new(m20250301_100946_create_table_session::Migration),
            Box::new(m20250301_101147_create_table_sessions_to_endpoints::Migration),
        ]
    }
}
