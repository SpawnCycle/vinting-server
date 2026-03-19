use crate::service_trait::{ServiceFilter, ServiceTrait};
use entity::{prelude::*, product};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbConn, DbErr, EntityLoaderTrait, EntityTrait,
    PrimaryKeyTrait,
};

pub struct ProductService<'a>(pub &'a DatabaseConnection);

impl ProductService<'_> {
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
}

impl ServiceTrait for ProductService<'_> {
    type Entity = Product;

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

    fn get_db(&self) -> &DatabaseConnection {
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
