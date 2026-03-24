use dtos::user::{get::UserGetDto, whoami::WhoamiDto};
use entity::prelude::*;
use rocket::{State, get, http::CookieJar, serde::json::Json};
use sea_orm::DbConn;
use services::{service_trait::ServiceTrait, user_service::UserService};

use crate::{jwt::JwtClaims, responder::Responder};

#[get("/whoami")]
pub async fn whoami(
    claims: JwtClaims,
    db: &State<DbConn>,
    jar: &CookieJar<'_>,
) -> Result<Json<WhoamiDto>, Responder> {
    let db = db.inner();
    claims.exists_or_remove(db, jar).await?;
    let user = claims
        .load(db, |q| q.with(UserRole).with(Role))
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
    Ok(Json(service.get_all_mutating(UserGetDto::from).await?))
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
