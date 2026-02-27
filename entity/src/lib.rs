pub mod prelude;

pub mod category;
pub mod image;
pub mod order;
pub mod product;
pub mod product_category;
pub mod product_image;
pub mod product_tag;
pub mod tag;
pub mod user;

pub mod active_action {
    pub trait ActiveAction {
        #[must_use]
        fn creating(self) -> Self;
        #[must_use]
        fn modifying(self) -> Self;
    }

    #[macro_export]
    macro_rules! active_actions {
        ($am:path) => {
            impl $crate::active_action::ActiveAction for $am {
                fn creating(self) -> Self {
                    let now = sea_orm::sea_query::prelude::Utc::now().naive_local();
                    self.set_created_at(now).set_modified_at(now)
                }
                fn modifying(self) -> Self {
                    let now = sea_orm::sea_query::prelude::Utc::now().naive_local();
                    self.set_modified_at(now)
                }
            }
        };
    }
}

#[cfg(test)]
mod test {
    use sea_orm::{Database, DbErr};

    #[tokio::test]
    async fn can_make_db() -> Result<(), DbErr> {
        let db = Database::connect("sqlite::memory:").await?;
        db.get_schema_registry("entity::*").sync(&db).await?;

        Ok(())
    }
}
