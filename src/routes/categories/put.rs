use dtos::CategoryPutDto;
use migrations::constants::ADMIN_ROLE;
use rocket::{State, put, response::status::NoContent, serde::json::Json};
use sea_orm::DbConn;
use services::{category_service::CategoryService, service_trait::ServiceTrait};

use crate::{jwt::JwtClaims, responder::Responder};

#[put("/<id>", format = "application/json", data = "<data>")]
pub async fn one(
    db: &State<DbConn>,
    claims: JwtClaims,
    id: i32,
    data: Json<CategoryPutDto>,
) -> Result<NoContent, Responder> {
    let db = db.inner();

    if !claims.has_role(db, ADMIN_ROLE).await? {
        return Err(Responder::unauhorized(
            "Category Modifications are only allowed for admin users",
        ));
    }

    let service = CategoryService(db);
    let category = data.into_inner();

    if category.id != id {
        return Err(Responder::bad_request(
            "The given id does not match the id in the body",
        ));
    }

    if !service.exists_by_id(id).await? {
        return Err(Responder::conflict(
            "There is no category with the given id",
        ));
    }

    if service.exists_by_name(category.data.name.clone()).await? {
        return Err(Responder::conflict(
            "There is already a category with the given name",
        ));
    }

    let _ = service.update(category).await?;

    Ok(NoContent)
}
