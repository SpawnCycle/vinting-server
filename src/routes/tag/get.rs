use dtos::category::get::CategoryGetDto;
use dtos::tag::get::TagGetDto;
use rocket::serde::json::Json;
use rocket::{State, get};
use sea_orm::DbConn;
use services::category_service::CategoryService;
use services::service_trait::ServiceTrait;
use services::tag_service::TagService;

use crate::responder::Responder;

#[get("/")]
pub async fn get_all(db: &State<DbConn>) -> Result<Json<Vec<TagGetDto>>, Responder> {
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

#[get("/<id>")]
pub async fn get_single(id: i32, db: &State<DbConn>) -> Result<Json<TagGetDto>, Responder> {
    let db = db.inner();
    let service = TagService(db);

    let tag = service.get_by_id(id).await?.ok_or(Responder::not_found(
        "The tag with the given id was not found",
    ))?;

    Ok(Json(tag.into()))
}
