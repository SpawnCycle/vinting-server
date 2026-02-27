mod filter;

use entity::active_action::ActiveAction;
pub use filter::*;

use sea_orm::{
    Condition, DatabaseConnection, DbConn, DbErr, EntityTrait, PrimaryKeyTrait, QueryFilter,
    Select, SelectExt, prelude::async_trait::async_trait,
};

/// trait for getting tables via service
///
/// the trait has an associate type:
/// - `Entity`, which has to implement `EntityTrait` (or in other words, it must be an entity)
///
/// functions that should be provided:
///  - `default_filters` (returns the filters the queries should run with by default)
///  - `iter_filter` (ideally should be the same as `default_filters`, but for use with iterators)
///  - `get_db` (returns the db the queries should be run with)
///  - `insert_active_model_ex` (exposes the new `ActiveModelEx`'s insert function)
///  - `update_active_model_ex` (exposes the new `ActiveModelEx`'s update function)
#[async_trait]
pub trait ServiceTrait {
    type Entity: EntityTrait;

    fn default_filters() -> Condition;
    fn get_db(&self) -> &DatabaseConnection;
    fn iter_filter<M>(m: M) -> bool
    where
        M: Into<<Self::Entity as EntityTrait>::Model>;

    fn insert_active_model_ex(
        am: <Self::Entity as EntityTrait>::ActiveModelEx,
        db: &DbConn,
    ) -> impl Future<Output = Result<<Self::Entity as EntityTrait>::ModelEx, DbErr>> + Send;

    fn update_active_model_ex(
        am: <Self::Entity as EntityTrait>::ActiveModelEx,
        db: &DbConn,
    ) -> impl Future<Output = Result<<Self::Entity as EntityTrait>::ModelEx, DbErr>> + Send;

    // general use functions

    async fn exists_by_id<U>(&self, id: U) -> Result<bool, DbErr>
    where
        U: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send,
    {
        self.exists_by_id_raw(id, Some(Self::default_filters()), |q| q)
            .await
    }

    async fn get_by_id<U>(
        &self,
        id: U,
    ) -> Result<Option<<Self::Entity as EntityTrait>::Model>, DbErr>
    where
        U: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send,
    {
        self.get_by_id_raw(id, Some(Self::default_filters()), |q| q)
            .await
    }

    async fn get_all(&self) -> Result<Vec<<Self::Entity as EntityTrait>::Model>, DbErr> {
        self.get_all_raw(Some(Self::default_filters()), |q| q).await
    }

    async fn insert<M>(
        &self,
        active_model: M,
    ) -> Result<<Self::Entity as EntityTrait>::ModelEx, DbErr>
    where
        M: Into<<Self::Entity as EntityTrait>::ActiveModelEx> + Send,
        <Self::Entity as EntityTrait>::ActiveModelEx: ActiveAction + Send,
    {
        let am = active_model.into() as <Self::Entity as EntityTrait>::ActiveModelEx;

        Self::insert_active_model_ex(am.creating(), self.get_db()).await
    }

    async fn update<M>(
        &self,
        active_model: M,
    ) -> Result<<Self::Entity as EntityTrait>::ModelEx, DbErr>
    where
        M: Into<<Self::Entity as EntityTrait>::ActiveModelEx> + Send,
        <Self::Entity as EntityTrait>::ActiveModelEx: ActiveAction + Send,
    {
        let am = active_model.into() as <Self::Entity as EntityTrait>::ActiveModelEx;

        Self::update_active_model_ex(am.modifying(), self.get_db()).await
    }

    // 'raw'/extended use functions

    async fn exists_by_id_raw<U, F>(
        &self,
        id: U,
        filter: Option<Condition>,
        with: F,
    ) -> Result<bool, DbErr>
    where
        U: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send,
        F: FnOnce(Select<Self::Entity>) -> Select<Self::Entity> + Send,
    {
        let q = Self::Entity::find_by_id(id);
        let mut q = with(q);

        if let Some(filter) = filter {
            q = q.filter(filter);
        }
        q.exists(self.get_db()).await
    }

    async fn get_by_id_raw<U, F>(
        &self,
        id: U,
        filter: Option<Condition>,
        with: F,
    ) -> Result<Option<<Self::Entity as EntityTrait>::Model>, DbErr>
    where
        U: Into<<<Self::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send,
        F: FnOnce(Select<Self::Entity>) -> Select<Self::Entity> + Send,
    {
        let q = Self::Entity::find_by_id(id);
        let mut q = with(q);

        if let Some(filter) = filter {
            q = q.filter(filter);
        }

        q.one(self.get_db()).await
    }

    async fn get_all_raw<F>(
        &self,
        filter: Option<Condition>,
        with: F,
    ) -> Result<Vec<<Self::Entity as EntityTrait>::Model>, DbErr>
    where
        F: FnOnce(Select<Self::Entity>) -> Select<Self::Entity> + Send,
    {
        let q = Self::Entity::find();
        let mut q = with(q);

        if let Some(filter) = filter {
            q = q.filter(filter);
        }

        q.all(self.get_db()).await
    }
}

#[cfg(test)]
mod test {
    use sea_orm::{
        ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, Database, DatabaseConnection,
        DbErr, sea_query::prelude::Utc,
    };
    use sea_orm::{DbConn, EntityTrait, QueryOrder};
    use std::ops::Not;

    use super::ServiceTrait;
    use entity::tag;

    /// Uses tag as the test entity because it's small
    /// In real services use `&DatabaseConnection` instead of `DatabaseConnection` directly
    struct TestService(DatabaseConnection);

    impl ServiceTrait for TestService {
        type Entity = tag::Entity;

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

        fn get_db(&self) -> &DatabaseConnection {
            &self.0
        }

        fn insert_active_model_ex(
            am: <Self::Entity as EntityTrait>::ActiveModelEx,
            db: &DbConn,
        ) -> impl Future<Output = Result<<Self::Entity as EntityTrait>::ModelEx, DbErr>> {
            am.insert(db)
        }

        fn update_active_model_ex(
            am: <Self::Entity as EntityTrait>::ActiveModelEx,
            db: &DbConn,
        ) -> impl Future<Output = Result<<Self::Entity as EntityTrait>::ModelEx, DbErr>> {
            am.update(db)
        }
    }

    fn mock_data() -> Vec<tag::ActiveModel> {
        let now = Utc::now().naive_local();
        vec![
            tag::ActiveModel {
                id: Set(1),
                created_at: Set(now),
                modified_at: Set(now),
                deleted_at: Set(None),
                name: Set("Test1".to_string()),
            },
            tag::ActiveModel {
                id: Set(2),
                created_at: Set(now),
                modified_at: Set(now),
                deleted_at: Set(None),
                name: Set("Test2".to_string()),
            },
            tag::ActiveModel {
                id: Set(3),
                created_at: Set(now),
                modified_at: Set(now),
                deleted_at: Set(Some(now)),
                name: Set("Test3".to_string()),
            },
        ]
    }

    async fn setup_db() -> Result<DatabaseConnection, DbErr> {
        let db = Database::connect("sqlite::memory:").await?;
        db.get_schema_registry("entity::*").sync(&db).await?;
        Ok(db)
    }

    async fn setup_mock(db: &DatabaseConnection) -> Result<(), DbErr> {
        for i in mock_data() {
            i.insert(db).await?;
        }

        Ok(())
    }

    async fn setup_service() -> Result<TestService, DbErr> {
        let db = setup_db().await?;

        setup_mock(&db).await?;

        Ok(TestService(db))
    }

    // base for service based tests, do take out the lint allow
    #[allow(unused_variables)]
    #[tokio::test]
    async fn service_test_base() -> Result<(), DbErr> {
        let service = setup_service().await?;

        Ok(())
    }

    #[tokio::test]
    async fn exists() -> Result<(), DbErr> {
        let service = setup_service().await?;

        assert!(service.exists_by_id(1).await?);
        assert!(service.exists_by_id(3).await?.not());

        Ok(())
    }

    #[tokio::test]
    async fn exists_raw() -> Result<(), DbErr> {
        let service = setup_service().await?;

        assert!(service.exists_by_id_raw(1, None, |q| q).await?);
        assert!(service.exists_by_id_raw(3, None, |q| q).await?);
        assert!(service.exists_by_id_raw(5, None, |q| q).await?.not());

        Ok(())
    }

    #[tokio::test]
    async fn get_by_id() -> Result<(), DbErr> {
        let service = setup_service().await?;

        let tag1 = service.get_by_id(1).await?;
        assert!(tag1.is_some());
        let tag1 = tag1.expect("already asserted that it exists");

        assert!(tag1.id == 1);
        assert!(tag1.name == "Test1");

        Ok(())
    }

    #[tokio::test]
    async fn get_by_id_raw() -> Result<(), DbErr> {
        let service = setup_service().await?;

        let tag1 = service.get_by_id_raw(3, None, |q| q).await?;
        assert!(tag1.is_some());
        let tag1 = tag1.expect("already asserted that it exists");

        assert!(tag1.id == 3);
        assert!(tag1.deleted_at.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn get_all() -> Result<(), DbErr> {
        let service = setup_service().await?;

        let all = service.get_all().await?;
        assert!(all.len() == 2);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_raw() -> Result<(), DbErr> {
        let service = setup_service().await?;

        let raw_all = service.get_all_raw(None, |q| q).await?;
        assert!(raw_all.len() == 3);

        Ok(())
    }

    #[tokio::test]
    async fn get_with_order() -> Result<(), DbErr> {
        let service = setup_service().await?;

        let raw_all = service
            .get_all_raw(None, |q| q.order_by_desc(tag::Column::Name))
            .await?;
        assert!(raw_all.len() == 3);

        let all_name = raw_all.into_iter().map(|t| t.name).collect::<Vec<_>>();
        assert!(all_name == ["Test3", "Test2", "Test1",]);

        Ok(())
    }
}
