use dtos::OrderGetDto;
use migrations::constants::ADMIN_ROLE;
use rocket::{
    State, get,
    http::{CookieJar, uri::Host},
    serde::json::Json,
};
use sea_orm::{DbConn, TransactionTrait};
use services::order_service::OrderService;

use crate::{constants::construct_host, jwt::JwtClaims, responder::Responder};

#[get("/<id>")]
pub async fn one(
    id: i32,
    host: &Host<'_>,
    jar: &CookieJar<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
) -> Result<Json<OrderGetDto>, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    let host = construct_host(host);
    claims.exists_or_unauthorized(db, jar).await?;
    let service = OrderService(db);

    let order = service
        .load_by_id(id)
        .await?
        .ok_or(Responder::not_found("There is no order with that id"))?;

    if order.user_id != claims.uid && !claims.has_role(db, ADMIN_ROLE).await? {
        return Err(Responder::unauhorized(
            "You are not allowed to view this order",
        ));
    }

    Ok(Json(
        OrderGetDto::with_product(order, &host).expect("The model should be properly loaded"),
    ))
}

/// returns the logged in user's orders
#[get("/")]
pub async fn from_user(
    host: &Host<'_>,
    jar: &CookieJar<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
) -> Result<Json<Vec<OrderGetDto>>, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    let host = construct_host(host);
    let service = OrderService(db);
    claims.exists_or_unauthorized(db, jar).await?;

    let orders = service
        .load_from_user(claims.uid)
        .await?
        .into_iter()
        .map(|m| OrderGetDto::with_product(m, &host).expect("The model should be properly loaded"))
        .collect::<Vec<_>>();

    Ok(Json(orders))
}

/// **admin only**
/// returns all of the orders, regardless of whose it is
#[get("/all")]
pub async fn all(
    host: &Host<'_>,
    jar: &CookieJar<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
) -> Result<Json<Vec<OrderGetDto>>, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    let host = construct_host(host);
    let service = OrderService(db);
    claims.exists_or_unauthorized(db, jar).await?;
    if !claims.has_role(db, ADMIN_ROLE).await? {
        return Err(Responder::unauhorized("You can't view all of the orders"));
    }

    let orders = service
        .load_all_mapping(|m| {
            OrderGetDto::with_product(m, &host).expect("The model should be properly loaded")
        })
        .await?;

    Ok(Json(orders))
}
