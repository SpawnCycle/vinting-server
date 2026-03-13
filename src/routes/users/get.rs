use dtos::user::whoami::WhoamiDto;
use entity::prelude::*;
use rocket::{State, get, serde::json::Json};
use sea_orm::DbConn;
use services::{service_trait::ServiceTrait, user_service::UserService};

use crate::{jwt::JwtClaims, responder::Responder};

#[get("/whoami")]
pub async fn get_self(claims: JwtClaims, db: &State<DbConn>) -> Result<Json<WhoamiDto>, Responder> {
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
        Err(Responder::unauhorized("You are not sigma"))
    }
}
