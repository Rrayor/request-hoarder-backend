pub use sea_orm_migration::prelude::*;

mod m20241204_073612_create_table_project;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241204_073612_create_table_project::Migration),
        ]
    }
}
