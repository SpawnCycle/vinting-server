use crate::service_trait::{ServiceFilter, ServiceTrait};
use entity::{prelude::*, product};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DatabaseTransaction, DbConn, DbErr, EntityLoaderTrait,
    EntityTrait, PrimaryKeyTrait, QueryFilter, TransactionTrait,
};

// TODO: if possible fix the service types now having a mandatory type
pub struct ProductService<'a, C = DbConn>(pub &'a C)
where
    C: ConnectionTrait + Send,
    C: TransactionTrait<Transaction = DatabaseTransaction>;

impl<C> ProductService<'_, C>
where
    C: ConnectionTrait + Send,
    C: TransactionTrait<Transaction = DatabaseTransaction>,
{
    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn load_by_id(&self, id: i32) -> Result<Option<product::ModelEx>, DbErr> {
        Product::load()
            .filter_by_id(id)
            .with(User)
            .with(Category)
            .with(Tag)
            .with(Image)
            .service_filter::<Self>()
            .one(self.get_db())
            .await
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn load_by_id_mutating<T, F>(&self, id: i32, f: F) -> Result<Option<T>, DbErr>
    where
        F: FnMut(product::ModelEx) -> T,
    {
        Ok(Product::load()
            .filter_by_id(id)
            .with(User)
            .with(Category)
            .with(Tag)
            .with(Image)
            .service_filter::<Self>()
            .one(self.get_db())
            .await?
            .map(f))
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn load_all(&self) -> Result<Vec<product::ModelEx>, DbErr> {
        Product::load()
            .with(User)
            .with(Category)
            .with(Tag)
            .with(Image)
            .service_filter::<Self>()
            .all(self.get_db())
            .await
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn load_all_mutating<T, F>(&self, f: F) -> Result<Vec<T>, DbErr>
    where
        F: FnMut(product::ModelEx) -> T,
    {
        Ok(Product::load()
            .with(User)
            .with(Category)
            .with(Tag)
            .with(Image)
            .service_filter::<Self>()
            .all(self.get_db())
            .await?
            .into_iter()
            .map(f)
            .collect())
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn load_all_with(&self, filter: Condition) -> Result<Vec<product::ModelEx>, DbErr> {
        Product::load()
            .with(User)
            .with(Category)
            .with(Tag)
            .with(Image)
            .service_filter::<Self>()
            .filter(filter)
            .all(self.get_db())
            .await
    }

    /// # Errors
    /// Returns the error produced by sea-orm
    pub async fn load_all_with_mutating<T, F>(
        &self,
        filter: Condition,
        f: F,
    ) -> Result<Vec<T>, DbErr>
    where
        F: FnMut(product::ModelEx) -> T,
    {
        Ok(Product::load()
            .with(User)
            .with(Category)
            .with(Tag)
            .with(Image)
            .service_filter::<Self>()
            .filter(filter)
            .all(self.get_db())
            .await?
            .into_iter()
            .map(f)
            .collect())
    }
}

impl<C> ServiceTrait for ProductService<'_, C>
where
    C: ConnectionTrait + Send,
    C: TransactionTrait<Transaction = DatabaseTransaction>,
{
    type Entity = Product;
    type Connection = C;

    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as sea_orm::EntityTrait>::Model>,
    {
        let m = m.into() as product::Model;

        m.deleted_at.is_none()
    }

    fn default_filters() -> Condition {
        Condition::all().add(product::Column::DeletedAt.is_null())
    }

    fn get_db(&self) -> &C {
        self.0
    }

    fn new_active_model_ex_from_id<U>(id: U) -> <Self::Entity as EntityTrait>::ActiveModelEx
    where
        U: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    {
        product::ActiveModel::builder().set_id(id)
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
