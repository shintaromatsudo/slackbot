pub use sea_orm_migration::prelude::*;

mod m20221215_000001_create_plusplus;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20221215_000001_create_plusplus::Migration)]
    }
}
