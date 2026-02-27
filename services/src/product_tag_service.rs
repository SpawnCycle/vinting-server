use crate::service_trait::ServiceTrait;
use entity::product_tag;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, DbConn, DbErr, EntityTrait};

pub struct ProductTagService<'a>(pub &'a DatabaseConnection);

impl ServiceTrait for ProductTagService<'_> {
    type Entity = product_tag::Entity;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as product_tag::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(product_tag::Column::DeletedAt.is_null())
    }

    fn get_db(&self) -> &DatabaseConnection {
        self.0
    }

    fn insert_active_model_ex(
        am: <Self::Entity as EntityTrait>::ActiveModelEx,
        db: &DbConn,
    ) -> impl Future<Output = Result<<Self::Entity as EntityTrait>::ModelEx, DbErr>> + Send {
        am.insert(db)
    }

    fn update_active_model_ex(
        am: <Self::Entity as EntityTrait>::ActiveModelEx,
        db: &DbConn,
    ) -> impl Future<Output = Result<<Self::Entity as EntityTrait>::ModelEx, DbErr>> + Send {
        am.update(db)
    }
}
