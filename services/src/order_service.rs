use crate::service_trait::ServiceTrait;
use entity::{order, prelude::*};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DatabaseTransaction, DbConn, DbErr, EntityTrait,
    PrimaryKeyTrait, QueryFilter, SelectExt, TransactionTrait,
};

pub struct OrderService<'a, C = DbConn>(pub &'a C)
where
    C: ConnectionTrait + TransactionTrait<Transaction = DatabaseTransaction> + Send;

impl<C> OrderService<'_, C>
where
    C: ConnectionTrait + TransactionTrait<Transaction = DatabaseTransaction> + Send,
{
    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn exists_from_user(&self, uid: i32, pid: i32) -> Result<bool, DbErr> {
        Order::find()
            .filter(Self::default_filters())
            .filter(order::Column::UserId.eq(uid))
            .filter(order::Column::ProductId.eq(pid))
            .exists(self.get_db())
            .await
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn get_by_user(&self, uid: i32, pid: i32) -> Result<Option<order::Model>, DbErr> {
        Order::find()
            .filter(Self::default_filters())
            .filter(order::Column::UserId.eq(uid))
            .filter(order::Column::ProductId.eq(pid))
            .one(self.get_db())
            .await
    }
}

impl<C> ServiceTrait for OrderService<'_, C>
where
    C: ConnectionTrait + Send,
    C: TransactionTrait<Transaction = DatabaseTransaction>,
{
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
