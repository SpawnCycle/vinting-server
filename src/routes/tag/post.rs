use dtos::tag::{get::TagGetDto, post::TagPostDto};
use rocket::{State, http::uri::Host, post, response::status::Created, serde::json::Json};
use sea_orm::DbConn;
use services::{service_trait::ServiceTrait, tag_service::TagService};

use crate::responder::Responder;

#[post("/", format = "application/json", data = "<data>")]
pub async fn post(
    db: &State<DbConn>,
    host: &Host<'_>,
    data: Json<TagPostDto>,
) -> Result<Created<Json<TagGetDto>>, Responder> {
    let db = db.inner();
    let service = TagService(db);
    let tag = data.into_inner();

    let model = service.insert(tag).await?;
    Ok(Created::new(format!("{host}/api/tags/{}", model.id)).body(Json(model.into())))
}
