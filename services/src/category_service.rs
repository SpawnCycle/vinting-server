use crate::service_trait::ServiceTrait;
use entity::category;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, DbConn, DbErr, EntityTrait};

pub struct CategoryService<'a>(pub &'a DatabaseConnection);

impl ServiceTrait for CategoryService<'_> {
    type Entity = category::Entity;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as category::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(category::Column::DeletedAt.is_null())
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
