use migrations::constants::ADMIN_ROLE;
use rocket::{State, delete, http::CookieJar, response::status::NoContent};
use sea_orm::{DbConn, TransactionTrait};
use services::{product_service::ProductService, service_trait::ServiceTrait};

use crate::{jwt::JwtClaims, responder::Responder};

#[delete("/<id>")]
pub async fn one(
    id: i32,
    jar: &CookieJar<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
) -> Result<NoContent, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    let service = ProductService(db);
    claims.exists_or_unauthorized(db, jar).await?;

    let product = service.get_by_id(id).await?.ok_or(Responder::not_found(
        "A product with the given id doesn't exist",
    ))?;

    if product.seller_id != claims.uid || !claims.has_role(db, ADMIN_ROLE).await? {
        return Err(Responder::unauhorized("You can't delete other's products"));
    }

    let _ = service.delete_by_id(id).await?;

    trx.commit().await?;

    Ok(NoContent)
}
