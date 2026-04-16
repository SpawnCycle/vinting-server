mod m20260316_seed_roles;
mod m20260410_seed_categories;

pub mod constants;

pub use sea_orm_migration::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260316_seed_roles::Migration),
            Box::new(m20260410_seed_categories::Migration),
        ]
    }
}

pub mod testing {
    use super::m20260316_seed_roles;

    pub use sea_orm_migration::*;

    pub struct TestMigrator;

    #[async_trait::async_trait]
    impl MigratorTrait for TestMigrator {
        fn migrations() -> Vec<Box<dyn MigrationTrait>> {
            vec![Box::new(m20260316_seed_roles::Migration)]
        }
    }
}
