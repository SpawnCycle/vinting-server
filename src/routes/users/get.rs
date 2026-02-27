use rocket::{State, get};
use sea_orm::DbConn;
use services::{service_trait::ServiceTrait, user_service::UserService};

use crate::{responder::Responder, routes::users::jwt::JwtClaims};

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
