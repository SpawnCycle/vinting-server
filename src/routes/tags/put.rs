use dtos::tag::put::TagPutDto;
use rocket::{State, put, response::status::NoContent, serde::json::Json};
use sea_orm::DbConn;
use services::{service_trait::ServiceTrait, tag_service::TagService};

use crate::responder::Responder;

#[put("/<id>", data = "<data>")]
pub async fn one(
    db: &State<DbConn>,
    id: i32,
    data: Json<TagPutDto>,
) -> Result<NoContent, Responder> {
    let db = db.inner();
    let service = TagService(db);
    let tag = data.into_inner();

    if id != tag.id {
        return Err(Responder::bad_request(
            "The id in the url does not match id in the body",
        ));
    }

    if !service.exists_by_id(id).await? {
        return Err(Responder::not_found("There is no tag with the given id"));
    }

    let _ = service.update(tag).await?;

    Ok(NoContent)
}
