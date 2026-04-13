use crate::service_trait::ServiceTrait;
use entity::{active_action::ActiveAction, user};
use sea_orm::{
    ActiveValue::{NotSet, Set, Unchanged},
    ColumnTrait, Condition, ConnectionTrait, DatabaseTransaction, DbConn, DbErr, EntityTrait,
    PrimaryKeyTrait, QueryFilter, SelectExt, TransactionTrait,
    prelude::async_trait::async_trait,
};

pub struct UserService<'a, C = DbConn>(pub &'a C)
where
    C: ConnectionTrait + Send,
    C: TransactionTrait<Transaction = DatabaseTransaction>;

impl<C> UserService<'_, C>
where
    C: ConnectionTrait + Send,
    C: TransactionTrait<Transaction = DatabaseTransaction>,
{
    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn get_by_email<S>(&self, email: S) -> Result<Option<user::Model>, DbErr>
    where
        S: Into<String>,
    {
        let email = email.into() as String;
        user::Entity::find_by_email(email)
            .filter(Self::default_filters())
            .one(self.get_db())
            .await
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn exists_by_email<S>(&self, email: S) -> Result<bool, DbErr>
    where
        S: Into<String>,
    {
        let email = email.into() as String;
        user::Entity::find_by_email(email)
            .filter(Self::default_filters())
            .exists(self.get_db())
            .await
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn exists_by_email_all<S>(&self, email: S) -> Result<bool, DbErr>
    where
        S: Into<String>,
    {
        let email = email.into() as String;
        user::Entity::find_by_email(email)
            .exists(self.get_db())
            .await
    }
}

#[async_trait]
impl<C> ServiceTrait for UserService<'_, C>
where
    C: ConnectionTrait + Send,
    C: TransactionTrait<Transaction = DatabaseTransaction>,
{
    type Entity = user::Entity;
    type Connection = C;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as user::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(user::Column::DeletedAt.is_null())
    }

    fn get_db(&self) -> &C {
        self.0
    }

    fn new_active_model_ex_from_id<U>(id: U) -> <Self::Entity as EntityTrait>::ActiveModelEx
    where
        U: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        user::ActiveModel::builder().set_id(id)
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
        let am = active_model.into() as user::ActiveModelEx;
        let name = match am.email.clone() {
            Set(name) | Unchanged(name) => name,
            NotSet => {
                return Err(DbErr::Custom(
                    "Can't insert a user without an email".to_string(),
                ));
            }
        };

        match user::Entity::find_by_email(name).one(self.get_db()).await? {
            Some(existing_user) => {
                let am = am.set_id(existing_user.id).creating();

                am.creating()
                    .set_id(existing_user.id)
                    .update(self.get_db())
                    .await
            }
            None => am.creating().insert(self.get_db()).await,
        }
    }
}
