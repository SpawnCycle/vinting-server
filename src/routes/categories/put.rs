use dtos::category::put::CategoryPutDto;
use rocket::{State, put, response::status::NoContent, serde::json::Json};
use sea_orm::DbConn;
use services::{category_service::CategoryService, service_trait::ServiceTrait};

use crate::responder::Responder;

#[put("/<id>", data = "<data>")]
pub async fn one(
    db: &State<DbConn>,
    id: i32,
    data: Json<CategoryPutDto>,
) -> Result<NoContent, Responder> {
    let db = db.inner();
    let service = CategoryService(db);
    let category = data.into_inner();

    if category.id != id {
        return Err(Responder::bad_request(
            "The given id does not match the id in the body",
        ));
    }

    if !service.exists_by_id(id).await? {
        return Err(Responder::not_found(
            "There is no category with the given id",
        ));
    }

    let _ = service.update(category).await?;

    Ok(NoContent)
}
