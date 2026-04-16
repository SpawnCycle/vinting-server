use crate::service_trait::{ServiceConnection, ServiceTrait};
use entity::{order, prelude::*};
use sea_orm::{
    ColumnTrait, Condition, DbConn, DbErr, EntityLoaderTrait, EntityTrait, PrimaryKeyTrait,
    QueryFilter, SelectExt,
};

pub struct OrderService<'a, C: ServiceConnection = DbConn>(pub &'a C);

fn load_order() -> order::EntityLoader {
    Order::load()
        .with((Product, User))
        .with((Product, Category))
        .with((Product, Tag))
        .with((Product, Image))
}

impl<C: ServiceConnection> OrderService<'_, C> {
    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn load_all(&self) -> Result<Vec<order::ModelEx>, DbErr> {
        load_order()
            .filter(Self::default_filters())
            .all(self.get_db())
            .await
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn load_all_mapping<T, F>(&self, f: F) -> Result<Vec<T>, DbErr>
    where
        F: FnMut(order::ModelEx) -> T,
    {
        Ok(load_order()
            .filter(Self::default_filters())
            .all(self.get_db())
            .await?
            .into_iter()
            .map(f)
            .collect())
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn load_from_user(&self, uid: i32) -> Result<Vec<order::ModelEx>, DbErr> {
        load_order()
            .filter(Self::default_filters())
            .filter(order::Column::UserId.eq(uid))
            .all(self.get_db())
            .await
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn load_by_id(&self, id: i32) -> Result<Option<order::ModelEx>, DbErr> {
        load_order()
            .filter(Self::default_filters())
            .filter_by_id(id)
            .one(self.get_db())
            .await
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn load_by_user_product(
        &self,
        uid: i32,
        pid: i32,
    ) -> Result<Option<order::ModelEx>, DbErr> {
        load_order()
            .filter(Self::default_filters())
            .filter(order::Column::UserId.eq(uid))
            .filter(order::Column::ProductId.eq(pid))
            .one(self.get_db())
            .await
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn exists_by_user_product(&self, uid: i32, pid: i32) -> Result<bool, DbErr> {
        Order::find()
            .filter(Self::default_filters())
            .filter(order::Column::UserId.eq(uid))
            .filter(order::Column::ProductId.eq(pid))
            .exists(self.get_db())
            .await
    }
}

impl<C: ServiceConnection> ServiceTrait for OrderService<'_, C> {
    type Entity = order::Entity;
    type Connection = C;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as order::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(order::Column::DeletedAt.is_null())
    }

    fn get_db(&self) -> &C {
        self.0
    }

    fn new_active_model_ex_from_id<U>(id: U) -> <Self::Entity as EntityTrait>::ActiveModelEx
    where
        U: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        order::ActiveModel::builder().set_id(id)
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
