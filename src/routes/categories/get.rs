use dtos::category::get::CategoryGetDto;
use rocket::{State, get, serde::json::Json};
use sea_orm::DbConn;
use services::{category_service::CategoryService, service_trait::ServiceTrait};

use crate::responder::Responder;

#[get("/<id>")]
pub async fn one(db: &State<DbConn>, id: i32) -> Result<Json<CategoryGetDto>, Responder> {
    let db = db.inner();
    let service = CategoryService(db);

    let tag = service.get_by_id(id).await?.ok_or(Responder::not_found(
        "The provided category id does not exist",
    ))?;

    Ok(Json(tag.into()))
}

#[get("/")]
pub async fn all(db: &State<DbConn>) -> Result<Json<Vec<CategoryGetDto>>, Responder> {
    let db = db.inner();
    let service = CategoryService(db);

    let categories = service
        .get_all()
        .await?
        .into_iter()
        .map(CategoryGetDto::from)
        .collect::<Vec<_>>();

    Ok(Json(categories))
}
