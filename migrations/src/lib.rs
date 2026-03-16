pub use sea_orm_migration::*;

pub mod m20260316_seed_roles;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260316_seed_roles::Migration)]
    }
}
