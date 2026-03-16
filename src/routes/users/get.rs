use dtos::user::{get::UserGetDto, whoami::WhoamiDto};
use entity::prelude::*;
use rocket::{State, get, serde::json::Json};
use sea_orm::DbConn;
use services::{service_trait::ServiceTrait, user_service::UserService};

use crate::{jwt::JwtClaims, responder::Responder};

#[get("/whoami")]
pub async fn whoami(claims: JwtClaims, db: &State<DbConn>) -> Result<Json<WhoamiDto>, Responder> {
    let db = db.inner();
    let user = claims
        .load(db, |q| q.with(Role))
        .await?
        .ok_or(Responder::not_found(format!(
            "There is no user with id of {}",
            claims.uid
        )))?;

    Ok(Json(
        WhoamiDto::new(user).expect("The necessary should be loaded"),
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

#[get("/echo")]
pub async fn jwt_test(_claims: JwtClaims) -> &'static str {
    "You have a jwt"
}

#[get("/echo_auth")]
pub async fn auth_test(claims: JwtClaims, db: &State<DbConn>) -> Result<&'static str, Responder> {
    let db = db.inner();
    if UserService(db).exists_by_id(claims.uid).await? {
        Ok("You are a real user")
    } else {
        Err(Responder::unauhorized("You are not a sigma"))
    }
}
