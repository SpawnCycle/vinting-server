use dtos::OrderPutDto;
use migrations::constants::ADMIN_ROLE;
use rocket::{State, http::CookieJar, put, response::status::NoContent, serde::json::Json};
use sea_orm::{DbConn, TransactionTrait};
use services::{order_service::OrderService, service_trait::ServiceTrait};

use crate::{jwt::JwtClaims, responder::Responder};

#[put("/<id>", format = "application/json", data = "<data>")]
pub async fn one(
    id: i32,
    jar: &CookieJar<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
    data: Json<OrderPutDto>,
) -> Result<NoContent, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    let service = OrderService(db);
    let order = data.into_inner();
    claims.exists_or_unauthorized(db, jar).await?;

    if !claims.has_role(db, ADMIN_ROLE).await? {
        return Err(Responder::unauhorized("Only admins can change orders"));
    }

    if id != order.id {
        return Err(Responder::bad_request(
            "The id in the url and body do not match",
        ));
    }

    let _ = service.update(order).await?;

    trx.commit().await?;

    Ok(NoContent)
}
