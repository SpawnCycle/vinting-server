use dtos::CategoryGetDto;
use rocket::{State, get, serde::json::Json};
use sea_orm::DbConn;
use services::{category_service::CategoryService, service_trait::ServiceTrait};

use crate::responder::Responder;

#[get("/<id>")]
pub async fn one(id: i32, db: &State<DbConn>) -> Result<Json<CategoryGetDto>, Responder> {
    let db = db.inner();
    let service = CategoryService(db);

    let category = service.get_by_id(id).await?.ok_or(Responder::not_found(
        "The provided category id does not exist",
    ))?;

    Ok(Json(category.into()))
}

#[get("/")]
pub async fn all(db: &State<DbConn>) -> Result<Json<Vec<CategoryGetDto>>, Responder> {
    let db = db.inner();
    let service = CategoryService(db);

    let categories = service.get_all_mapping(CategoryGetDto::from).await?;

    Ok(Json(categories))
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use dtos::CategoryGetDto;
    use rocket::local::asynchronous::Client;

    use super::*;
    use crate::testing::{self, category};

    #[tokio::test]
    async fn categories_get_all_tracked() -> anyhow::Result<()> {
        let db = category::db().await?;
        let r = category::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let req = client.get("/api/categories/");
        let res = req.dispatch().await;
        let categories = res
            .into_json::<Vec<CategoryGetDto>>()
            .await
            .ok_or(anyhow!("Couldn't parse into json"))?;

        // testing::category::seed_db has 3 non-deleted rows
        assert_eq!(categories.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn categories_get_all_func() -> anyhow::Result<()> {
        let db = category::db().await?;
        let db = State::from(&db);

        let categories = all(db).await?;

        // testing::category::seed_db has 3 non-deleted rows
        assert_eq!(categories.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn categories_get_by_id_exists_tracked() -> anyhow::Result<()> {
        let db = category::db().await?;
        let r = category::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let req = client.get("/api/categories/1");
        let res = req.dispatch().await;
        let category = res.into_json::<CategoryGetDto>().await;

        assert!(category.is_some(), "The first category is not deleted");

        Ok(())
    }

    #[tokio::test]
    async fn categories_get_by_id_deleted_tracked() -> anyhow::Result<()> {
        let db = category::db().await?;
        let r = category::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let req = client.get("/api/categories/5");
        let res = req.dispatch().await;

        assert_eq!(res.status().code, 404, "The fifth category is deleted");

        Ok(())
    }

    #[tokio::test]
    async fn categories_get_by_id_not_existant_tracked() -> anyhow::Result<()> {
        let db = category::db().await?;
        let r = category::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let req = client.get("/api/categories/1000");
        let res = req.dispatch().await;

        assert_eq!(res.status().code, 404, "The 1000th category does not exist");

        Ok(())
    }

    #[tokio::test]
    async fn categories_get_by_id_exists_func() -> anyhow::Result<()> {
        let db = category::db().await?;
        let db = State::from(&db);

        let res = one(1, db).await;
        assert!(res.is_ok(), "The first category is not deleted");

        Ok(())
    }

    #[tokio::test]
    async fn categories_get_by_id_deleted_func() -> anyhow::Result<()> {
        let db = testing::db().await?;
        let db = State::from(&db);
        testing::category::seed_db(db).await?;

        let res = one(5, db).await;
        assert!(res.is_err(), "The fifth category is deleted");

        Ok(())
    }

    #[tokio::test]
    async fn categories_get_by_id_not_existant_tracked_func() -> anyhow::Result<()> {
        let db = category::db().await?;
        let db = State::from(&db);

        let res = one(1000, db).await;
        assert!(res.is_err(), "The 1000th category does not exist");

        Ok(())
    }
}
