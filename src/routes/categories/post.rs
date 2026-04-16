use dtos::{CategoryGetDto, CategoryPostDto};
use migrations::constants::ADMIN_ROLE;
use rocket::{State, http::uri::Host, post, response::status::Created, serde::json::Json};
use sea_orm::DbConn;
use services::{category_service::CategoryService, service_trait::ServiceTrait};

use crate::{constants::construct_host, jwt::JwtClaims, responder::Responder};

#[post("/", format = "application/json", data = "<data>")]
pub async fn one(
    host: &Host<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
    data: Json<CategoryPostDto>,
) -> Result<Created<Json<CategoryGetDto>>, Responder> {
    let db = db.inner();

    if !claims.has_role(db, ADMIN_ROLE).await? {
        return Err(Responder::unauhorized(
            "Category Modifications are only allowed for admin users",
        ));
    }

    let service = CategoryService(db);
    let category = data.into_inner();

    if service.exists_by_name(&category.name).await? {
        return Err(Responder::conflict(
            "A category with the given name already exists",
        ));
    }

    let model = service.insert(category).await?;

    Ok(Created::new(format!(
        "{}/api/categories/{}",
        construct_host(host),
        model.id
    ))
    .body(Json(model.into())))
}

#[cfg(test)]
mod tests {
    use dtos::CategoryPostDto;
    use rocket::{serde::json, uri};

    use super::*;
    use crate::testing::{self, category};

    #[tokio::test]
    async fn categories_post_unique_tracked() -> anyhow::Result<()> {
        let db = category::db().await?;
        let r = category::rocket(db).await?;

        let client = testing::admin_client(r).await?;

        let dto = CategoryPostDto {
            name: "Category unique".to_string(),
        };
        let req =
            testing::json_request(client.post("/api/categories/").body(json::to_string(&dto)?));

        let res = req.dispatch().await;

        assert_eq!(
            res.status().code,
            201,
            "This should create a new category successfully"
        );

        Ok(())
    }

    #[tokio::test]
    async fn categories_post_conflict_tracked() -> anyhow::Result<()> {
        let db = category::db().await?;
        let r = category::rocket(db).await?;

        let client = testing::admin_client(r).await?;

        let dto = CategoryPostDto {
            name: "Category 1".to_string(),
        };
        let req =
            testing::json_request(client.post("/api/categories/").body(json::to_string(&dto)?));
        let res = req.dispatch().await;

        assert_eq!(
            res.status().code,
            409,
            "This should create a conflict, because 'Category 1' already exists"
        );

        Ok(())
    }

    #[tokio::test]
    async fn categories_post_unique_func() -> anyhow::Result<()> {
        let db = category::db().await?;
        let db = State::from(&db);
        let host = Host::new(uri!("localhost:8000"));

        let dto = CategoryPostDto {
            name: "Category unique".to_string(),
        };

        let claims = JwtClaims::new(1);
        let res = one(&host, claims, db, Json(dto)).await;

        assert!(
            res.is_ok(),
            "This should create a new category successfully"
        );

        Ok(())
    }

    #[tokio::test]
    async fn categories_post_conflict_func() -> anyhow::Result<()> {
        let db = category::db().await?;
        let db = State::from(&db);
        let host = Host::new(uri!("localhost:8000"));

        let dto = CategoryPostDto {
            name: "Category 1".to_string(),
        };

        let claims = JwtClaims::new(1);
        let res = one(&host, claims, db, Json(dto)).await;

        assert!(
            res.is_err(),
            "This should create a conflict, because 'Category 1' already exists"
        );

        Ok(())
    }
}
