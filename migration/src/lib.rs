pub use sea_orm_migration::prelude::*;

mod m20241204_073612_create_table_project;
mod m20250105_123300_create_table_group;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241204_073612_create_table_project::Migration),
            Box::new(m20250105_123300_create_table_group::Migration),
        ]
    }
}
