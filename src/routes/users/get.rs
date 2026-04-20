use dtos::{ProductGetDto, UserGetDto, WhoamiDto};
use entity::{prelude::*, product};
use rocket::{
    State, get,
    http::{CookieJar, uri::Host},
    serde::json::Json,
};
use sea_orm::{ColumnTrait, Condition, DbConn};
use services::{
    product_service::ProductService, service_trait::ServiceTrait, user_service::UserService,
};

use crate::{constants::construct_host, jwt::JwtClaims, responder::Responder};

#[get("/whoami")]
pub async fn whoami(
    claims: JwtClaims,
    db: &State<DbConn>,
    jar: &CookieJar<'_>,
) -> Result<Json<WhoamiDto>, Responder> {
    let db = db.inner();
    claims.exists_or_unauthorized(db, jar).await?;
    let user = claims
        .load(db, |q| q.with(Role))
        .await?
        .ok_or(Responder::not_found(format!(
            "There is no user with id of {}",
            claims.uid
        )))?;

    Ok(Json(
        WhoamiDto::new(user).expect("The necessary fields should be loaded"),
    ))
}

#[get("/")]
pub async fn all(db: &State<DbConn>) -> Result<Json<Vec<UserGetDto>>, Responder> {
    let db = db.inner();
    let service = UserService(db);
    Ok(Json(service.get_all_mapping(UserGetDto::from).await?))
}

#[get("/<id>")]
pub async fn one(id: i32, db: &State<DbConn>) -> Result<Json<UserGetDto>, Responder> {
    let db = db.inner();
    let service = UserService(db);
    Ok(Json(
        service
            .get_by_id_mutating(id, UserGetDto::from)
            .await?
            .ok_or(Responder::not_found("There is no user with the given id"))?,
    ))
}

#[get("/<id>/products")]
pub async fn user_products(
    id: i32,
    host: &Host<'_>,
    db: &State<DbConn>,
) -> Result<Json<Vec<ProductGetDto>>, Responder> {
    let db = db.inner();
    let host = construct_host(host);
    let u_service = UserService(db);
    let p_service = ProductService(db);

    let user = u_service
        .get_by_id(id)
        .await?
        .ok_or(Responder::not_found("There is no user with the given id"))?;
    let filter = Condition::all().add(product::Column::SellerId.eq(user.id));

    let products = p_service
        .load_all_with_mutating(filter, |m| {
            ProductGetDto::from_model_with_host(m, &host)
                .expect("The model should be properly loaded")
        })
        .await?;

    Ok(Json(products))
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use rocket::local::asynchronous::Client;

    use super::*;

    use crate::testing::user;

    #[tokio::test]
    async fn users_get_all() -> anyhow::Result<()> {
        let db = user::db().await?;
        let r = user::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let req = client.get("/api/users/");
        let res = req.dispatch().await;

        assert_eq!(res.status().code, 200);

        let users = res
            .into_json::<Vec<UserGetDto>>()
            .await
            .ok_or(anyhow!("Couldn't parse users into json"))?;

        // there are 2 users that are not deleted
        assert_eq!(users.len(), 2);

        Ok(())
    }

    #[tokio::test]
    async fn users_get_existing() -> anyhow::Result<()> {
        let db = user::db().await?;
        let r = user::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let req = client.get("/api/users/1");
        let res = req.dispatch().await;

        assert_eq!(res.status().code, 200);

        let user = res
            .into_json::<UserGetDto>()
            .await
            .ok_or(anyhow!("Couldn't parse users into json"))?;

        assert_eq!(user.name, "Name 1");

        Ok(())
    }

    #[tokio::test]
    async fn users_get_deleted() -> anyhow::Result<()> {
        let db = user::db().await?;
        let r = user::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let req = client.get("/api/users/3");
        let res = req.dispatch().await;

        assert_eq!(res.status().code, 404);

        Ok(())
    }

    #[tokio::test]
    async fn users_get_non_existant() -> anyhow::Result<()> {
        let db = user::db().await?;
        let r = user::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let req = client.get("/api/users/1000");
        let res = req.dispatch().await;

        assert_eq!(res.status().code, 404);

        Ok(())
    }
}
