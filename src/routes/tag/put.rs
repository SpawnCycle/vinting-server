use dtos::tag::put::TagPutDto;
use rocket::{State, put, response::status::NoContent, serde::json::Json};
use sea_orm::DbConn;
use services::service_trait::ServiceTrait;
use services::tag_service::TagService;

use crate::responder::Responder;

#[put("/<id>", format = "application/json", data = "<data>")]
pub async fn put(
    id: i32,
    data: Json<TagPutDto>,
    db: &State<DbConn>,
) -> Result<NoContent, Responder> {
    let db = db.inner();
    let service = TagService(db);
    let tag = data.into_inner();

    if tag.id != id {
        return Err(Responder::bad_request("Specified id doesn't match with id"));
    }

    let _ = service.update(tag).await?;

    Ok(NoContent)
}
