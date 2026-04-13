use migrations::constants::ADMIN_ROLE;
use rocket::{State, delete, http::CookieJar, response::status::NoContent};
use sea_orm::{DbConn, IntoActiveModel, TransactionTrait};
use services::{
    order_service::OrderService, product_service::ProductService, service_trait::ServiceTrait,
};

use crate::{jwt::JwtClaims, responder::Responder};

/// **admin only**
#[delete("/<id>")]
pub async fn one(
    id: i32,
    jar: &CookieJar<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
) -> Result<NoContent, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    let o_service = OrderService(db);
    let p_service = ProductService(db);

    claims.exists_or_unauthorized(db, jar).await?;
    if !claims.has_role(db, ADMIN_ROLE).await? {
        return Err(Responder::unauhorized(
            "You must be an admin to delete orders",
        ));
    }

    let order = o_service
        .get_by_id(id)
        .await?
        .ok_or(Responder::not_found("There is no order with the given id"))?;

    o_service.delete_by_id(id).await?;

    // reduce the sold count if the product_id points to a valid product
    if let Some(product) = p_service.get_by_id(order.product_id).await? {
        let sold = product.sold_stock;
        let am = product
            .into_active_model()
            .into_ex()
            .set_sold_stock(sold.saturating_sub(1));
        let _ = p_service.update(am).await?;
    }

    trx.commit().await?;

    Ok(NoContent)
}
