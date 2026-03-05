use dtos::tag::get::TagGetDto;
use rocket::{State, get, serde::json::Json};
use sea_orm::DbConn;
use services::{service_trait::ServiceTrait, tag_service::TagService};

use crate::responder::Responder;

#[get("/<id>")]
pub async fn one(db: &State<DbConn>, id: i32) -> Result<Json<TagGetDto>, Responder> {
    let db = db.inner();
    let service = TagService(db);

    let tag = service
        .get_by_id(id)
        .await?
        .ok_or(Responder::not_found("Provided tag id does not exist."))?;

    Ok(Json(tag.into()))
}

#[get("/")]
pub async fn all(db: &State<DbConn>) -> Result<Json<Vec<TagGetDto>>, Responder> {
    let db = db.inner();
    let service = TagService(db);

    let tags = service
        .get_all()
        .await?
        .into_iter()
        .map(TagGetDto::from)
        .collect::<Vec<_>>();

    Ok(Json(tags))
}
