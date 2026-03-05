use crate::service_trait::ServiceTrait;
use entity::category;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbConn, DbErr, EntityTrait, PrimaryKeyTrait,
    QueryFilter, SelectExt,
};

pub struct CategoryService<'a>(pub &'a DatabaseConnection);

impl CategoryService<'_> {
    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn exists_by_name<S>(&self, name: S) -> Result<bool, DbErr>
    where
        S: Into<String>,
    {
        let name = name.into() as String;
        category::Entity::find_by_name(name)
            .filter(Self::default_filters())
            .exists(self.get_db())
            .await
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn exists_by_name_all<S>(&self, name: S) -> Result<bool, DbErr>
    where
        S: Into<String>,
    {
        let name = name.into() as String;
        category::Entity::find_by_name(name)
            .exists(self.get_db())
            .await
    }
}

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

    fn new_active_model_ex_from_id<U>(id: U) -> <Self::Entity as EntityTrait>::ActiveModelEx
    where
        U: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        category::ActiveModel::builder().set_id(id)
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
