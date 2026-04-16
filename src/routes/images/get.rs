use dtos::ImageGetDto;
use rocket::{State, get, http::uri::Host, serde::json::Json};
use sea_orm::DbConn;
use services::{image_service::ImageService, service_trait::ServiceTrait};

use crate::{constants::construct_host, responder::Responder};

#[get("/")]
pub async fn all(host: &Host<'_>, db: &State<DbConn>) -> Result<Json<Vec<ImageGetDto>>, Responder> {
    let db = db.inner();
    let host = construct_host(host);
    let service = ImageService(db);

    let images = service
        .get_all_mapping(|m| ImageGetDto::from_model_with_host(m, &host))
        .await?;

    Ok(Json(images))
}

#[get("/<id>")]
pub async fn one(
    id: i32,
    host: &Host<'_>,
    db: &State<DbConn>,
) -> Result<Json<ImageGetDto>, Responder> {
    let db = db.inner();
    let host = construct_host(host);
    let service = ImageService(db);

    let images = service
        .get_by_id_mutating(id, |m| ImageGetDto::from_model_with_host(m, &host))
        .await?
        .ok_or(Responder::not_found("There is no image with the given id"))?;

    Ok(Json(images))
}
