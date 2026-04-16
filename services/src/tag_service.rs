use crate::service_trait::{ServiceConnection, ServiceTrait};
use entity::{active_action::ActiveAction, tag};
use sea_orm::{
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, Condition, DbConn, DbErr, EntityTrait, PrimaryKeyTrait, QueryFilter, SelectExt,
    TransactionTrait,
    prelude::async_trait::async_trait,
};

pub struct TagService<'a, C: ServiceConnection = DbConn>(pub &'a C);

impl<C: ServiceConnection> TagService<'_, C> {
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

#[async_trait]
impl<C: ServiceConnection> ServiceTrait for TagService<'_, C> {
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

    // overrides

    async fn insert<M>(
        &self,
        active_model: M,
    ) -> Result<<Self::Entity as EntityTrait>::ModelEx, DbErr>
    where
        Self::Connection: TransactionTrait,
        M: Into<<Self::Entity as EntityTrait>::ActiveModelEx> + Send,
        <Self::Entity as EntityTrait>::ActiveModelEx: ActiveAction + Send,
    {
        let am = active_model.into() as tag::ActiveModelEx;
        let name = match am.name.clone() {
            Set(name) | Unchanged(name) => name,
            NotSet => {
                return Err(DbErr::Custom(
                    "Can't insert a tag without a name".to_string(),
                ));
            }
        };

        match tag::Entity::find_by_name(name).one(self.get_db()).await? {
            Some(existing_tag) => {
                let am = am.set_id(existing_tag.id).creating();

                am.creating()
                    .set_id(existing_tag.id)
                    .update(self.get_db())
                    .await
            }
            None => am.creating().insert(self.get_db()).await,
        }
    }
}
