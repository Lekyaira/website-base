pub use sea_orm_migration::prelude::*;

mod m20241231_000001_post;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Define order of migrations.
            Box::new(m20241231_000001_post::Migration)
        ]
    }
}
