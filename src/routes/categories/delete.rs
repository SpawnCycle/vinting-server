use migrations::constants::ADMIN_ROLE;
use rocket::{State, delete, response::status::NoContent};
use sea_orm::DbConn;
use services::{category_service::CategoryService, service_trait::ServiceTrait};

use crate::{jwt::JwtClaims, responder::Responder};

#[delete("/<id>")]
pub async fn one(id: i32, claims: JwtClaims, db: &State<DbConn>) -> Result<NoContent, Responder> {
    let db = db.inner();

    if !claims.has_role(db, ADMIN_ROLE).await? {
        return Err(Responder::unauhorized(
            "Category Modifications are only allowed for admin users",
        ));
    }

    let service = CategoryService(db);

    if !service.exists_by_id(id).await? {
        return Err(Responder::not_found(
            "There is no category with the given id",
        ));
    }

    let _ = service.delete_by_id(id).await?;

    Ok(NoContent)
}
