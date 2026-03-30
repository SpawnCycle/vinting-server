use crate::service_trait::ServiceTrait;
use entity::role;
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DatabaseTransaction, DbConn, DbErr, EntityTrait,
    PrimaryKeyTrait, TransactionTrait,
};

pub struct RoleService<'a, C = DbConn>(pub &'a C)
where
    C: ConnectionTrait + Send,
    C: TransactionTrait<Transaction = DatabaseTransaction>;

impl<C> ServiceTrait for RoleService<'_, C>
where
    C: ConnectionTrait + Send,
    C: TransactionTrait<Transaction = DatabaseTransaction>,
{
    type Entity = role::Entity;
    type Connection = C;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as role::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(role::Column::DeletedAt.is_null())
    }

    fn get_db(&self) -> &C {
        self.0
    }

    fn new_active_model_ex_from_id<U>(id: U) -> <Self::Entity as EntityTrait>::ActiveModelEx
    where
        U: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        role::ActiveModel::builder().set_id(id)
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
