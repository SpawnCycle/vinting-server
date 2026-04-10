use entity::category;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter,
    sea_query::prelude::Utc,
};
use sea_orm_migration::prelude::*;

use crate::constants::CATEGORY_NAMES;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let now = Utc::now().naive_local();

        for category in CATEGORY_NAMES {
            category::ActiveModel {
                created_at: Set(now),
                modified_at: Set(now),
                deleted_at: Set(None),
                name: Set(category.to_string()),
                ..Default::default()
            }
            .insert(db)
            .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        category::Entity::delete_many()
            .filter(category::Column::Name.is_in(CATEGORY_NAMES))
            .exec(db)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use entity::category;
    use sea_orm::{Database, DbConn, DbErr};
    use sea_orm_migration::{MigrationTrait, SchemaManager};

    async fn prepare_db() -> Result<DbConn, DbErr> {
        let db = Database::connect("sqlite::memory:").await?;
        db.get_schema_registry("entity::*").sync(&db).await?;
        Ok(db)
    }

    #[tokio::test]
    async fn can_run_migration() -> anyhow::Result<()> {
        let db = prepare_db().await?;

        let manager = SchemaManager::new(&db);
        super::Migration.up(&manager).await?;

        Ok(())
    }

    #[tokio::test]
    async fn migration_created_all_items() -> anyhow::Result<()> {
        let db = prepare_db().await?;

        let manager = SchemaManager::new(&db);
        super::Migration.up(&manager).await?;

        for name in super::CATEGORY_NAMES {
            let _role = category::Entity::find_by_name(name)
                .one(&db)
                .await?
                .ok_or(anyhow::anyhow!("Could not find a category in the db"))?;
        }

        Ok(())
    }
}
