use migrations::constants::ADMIN_ROLE;
use rocket::{State, delete, http::CookieJar, response::status::NoContent};
use sea_orm::{DbConn, TransactionTrait};
use services::{image_service::ImageService, service_trait::ServiceTrait};
use tokio::fs;

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
    let service = ImageService(db);
    claims.exists_or_unauthorized(db, jar).await?;

    let image = service
        .get_by_id(id)
        .await?
        .ok_or(Responder::not_found("There is no image with the given id"))?;

    if image.user_id != claims.uid && !claims.has_role(db, ADMIN_ROLE).await? {
        return Err(Responder::unauhorized("You can't delete this image"));
    }

    // cleanup if there are no other references to that image
    if !service.exists_by_path(&image.path).await? {
        fs::remove_file(image.path).await?;
    }

    trx.commit().await?;

    Ok(NoContent)
}
