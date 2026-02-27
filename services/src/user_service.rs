use crate::service_trait::ServiceTrait;
use entity::user;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbConn, DbErr, EntityTrait, QueryFilter, SelectExt,
};

pub struct UserService<'a>(pub &'a DatabaseConnection);

impl UserService<'_> {
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
}

impl ServiceTrait for UserService<'_> {
    type Entity = user::Entity;

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
