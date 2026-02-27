use crate::service_trait::ServiceTrait;
use entity::image;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, DbConn, DbErr, EntityTrait};

pub struct ImageService<'a>(pub &'a DatabaseConnection);

impl ServiceTrait for ImageService<'_> {
    type Entity = image::Entity;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as image::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(image::Column::DeletedAt.is_null())
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
