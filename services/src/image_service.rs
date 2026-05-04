use crate::service_trait::{ServiceConnection, ServiceTrait};
use entity::image;
use sea_orm::{
    ColumnTrait, Condition, DbConn, DbErr, EntityTrait, PrimaryKeyTrait, QueryFilter, SelectExt,
};

#[derive(Debug)]
pub struct ImageService<'a, C: ServiceConnection = DbConn>(pub &'a C);

impl<C: ServiceConnection> ImageService<'_, C> {
    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn exists_by_path(&self, path: &str) -> Result<bool, DbErr> {
        image::Entity::find()
            .filter(Self::default_filters())
            .filter(image::Column::Path.eq(path))
            .exists(self.get_db())
            .await
    }
}

impl<C: ServiceConnection> ServiceTrait for ImageService<'_, C> {
    type Entity = image::Entity;
    type Connection = C;

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

    fn get_db(&self) -> &C {
        self.0
    }

    fn new_active_model_ex_from_id<U>(id: U) -> <Self::Entity as EntityTrait>::ActiveModelEx
    where
        U: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        image::ActiveModel::builder().set_id(id)
    }

    fn insert_active_model_ex(
        am: <Self::Entity as EntityTrait>::ActiveModelEx,
        db: &C,
    ) -> impl Future<Output = Result<<Self::Entity as EntityTrait>::ModelEx, DbErr>> {
        am.insert(db)
    }

    fn update_active_model_ex(
        am: <Self::Entity as EntityTrait>::ActiveModelEx,
        db: &C,
    ) -> impl Future<Output = Result<<Self::Entity as EntityTrait>::ModelEx, DbErr>> {
        am.update(db)
    }
}
