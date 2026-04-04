use migrations::constants::ADMIN_ROLE;
use rocket::{State, delete, http::CookieJar, response::status::NoContent};
use sea_orm::{DbConn, TransactionTrait};
use services::{order_service::OrderService, service_trait::ServiceTrait};

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
    claims.exists_or_unauthorized(db, jar).await?;
    if !claims.has_role(db, ADMIN_ROLE).await? {
        return Err(Responder::unauhorized(
            "You must be an admin to delete orders",
        ));
    }
    let service = OrderService(db);

    service.exists_by_id(id).await?;

    service.delete_by_id(id).await?;

    Ok(NoContent)
}
