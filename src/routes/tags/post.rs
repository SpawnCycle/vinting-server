use dtos::tag::{get::TagGetDto, post::TagPostDto};
use entity::tag::{self};
use rocket::{State, http::uri::Host, post, response::status::Created, serde::json::Json};
use sea_orm::DbConn;
use services::{service_trait::ServiceTrait, tag_service::TagService};

use crate::responder::Responder;

#[post("/", data = "<data>")]
pub async fn one(
    host: &Host<'_>,
    db: &State<DbConn>,
    data: Json<TagPostDto>,
) -> Result<Created<Json<TagGetDto>>, Responder> {
    let db = db.inner();
    let service = TagService(db);
    let tag = data.into_inner();

    if service.exists_by_name_all(&tag.name).await? {
        return Err(Responder::bad_request(
            "Tag with the same name already exists",
        ));
    }

    let model = service.insert(tag::ActiveModelEx::from(tag)).await?;

    Ok(Created::new(format!("{host}/api/tags/{}", model.id)).body(Json(model.into())))
}
