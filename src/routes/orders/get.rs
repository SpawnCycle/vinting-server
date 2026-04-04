use dtos::order::get::OrderGetDto;
use entity::{order, prelude::*};
use migrations::constants::ADMIN_ROLE;
use rocket::{State, get, http::CookieJar, serde::json::Json};
use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter, TransactionTrait};
use services::{
    order_service::OrderService,
    service_trait::{ServiceFilter, ServiceTrait},
};

use crate::{jwt::JwtClaims, responder::Responder};

#[get("/<id>")]
pub async fn one(
    id: i32,
    jar: &CookieJar<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
) -> Result<Json<OrderGetDto>, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    claims.exists_or_unauthorized(db, jar).await?;
    let service = OrderService(db);

    let order = service
        .get_by_id(id)
        .await?
        .ok_or(Responder::not_found("There is no order with that id"))?;

    if order.user_id != claims.uid && !claims.has_role(db, ADMIN_ROLE).await? {
        return Err(Responder::unauhorized(
            "You are not allowed to view this order",
        ));
    }

    Ok(Json(order.into()))
}

/// returns the logged in user's orders
#[get("/")]
pub async fn from_user(
    jar: &CookieJar<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
) -> Result<Json<Vec<OrderGetDto>>, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    claims.exists_or_unauthorized(db, jar).await?;

    let orders = Order::find()
        .service_filter::<OrderService>()
        .filter(order::Column::UserId.eq(claims.uid))
        .all(db)
        .await?
        .into_iter()
        .map(OrderGetDto::from)
        .collect::<Vec<_>>();

    Ok(Json(orders))
}

/// **admin only**
/// returns all of the orders, regardless of whose it is
#[get("/all")]
pub async fn all(
    jar: &CookieJar<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
) -> Result<Json<Vec<OrderGetDto>>, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    let service = OrderService(db);
    claims.exists_or_unauthorized(db, jar).await?;
    if !claims.has_role(db, ADMIN_ROLE).await? {
        return Err(Responder::unauhorized("You can't view all of the orders"));
    }

    let orders = service.get_all_mapping(OrderGetDto::from).await?;

    Ok(Json(orders))
}
