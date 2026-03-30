use crate::service_trait::ServiceTrait;
use entity::tag;
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DatabaseTransaction, DbConn, DbErr, EntityTrait,
    PrimaryKeyTrait, QueryFilter, SelectExt, TransactionTrait,
};

pub struct TagService<'a, C = DbConn>(pub &'a C)
where
    C: ConnectionTrait + Send,
    C: TransactionTrait<Transaction = DatabaseTransaction>;

impl<C> TagService<'_, C>
where
    C: ConnectionTrait + Send,
    C: TransactionTrait<Transaction = DatabaseTransaction>,
{
    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn exists_by_name<S>(&self, name: S) -> Result<bool, DbErr>
    where
        S: Into<String>,
    {
        let name = name.into() as String;
        tag::Entity::find_by_name(name)
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
        tag::Entity::find_by_name(name)
            .filter(Self::default_filters())
            .exists(self.get_db())
            .await
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn get_by_name<S>(&self, name: S) -> Result<Option<tag::Model>, DbErr>
    where
        S: Into<String>,
    {
        let name = name.into() as String;
        tag::Entity::find_by_name(name).one(self.get_db()).await
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn get_by_name_all<S>(&self, name: S) -> Result<Option<tag::Model>, DbErr>
    where
        S: Into<String>,
    {
        let name = name.into() as String;
        tag::Entity::find_by_name(name).one(self.get_db()).await
    }
}

impl<C> ServiceTrait for TagService<'_, C>
where
    C: ConnectionTrait + Send,
    C: TransactionTrait<Transaction = DatabaseTransaction>,
{
    type Entity = tag::Entity;
    type Connection = C;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as tag::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(tag::Column::DeletedAt.is_null())
    }

    fn get_db(&self) -> &C {
        self.0
    }

    fn new_active_model_ex_from_id<U>(id: U) -> <Self::Entity as EntityTrait>::ActiveModelEx
    where
        U: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        tag::ActiveModel::builder().set_id(id)
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
