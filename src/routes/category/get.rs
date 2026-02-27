use dtos::category::get::CategoryGetDto;
use entity::prelude::Category;
use rocket::serde::json::Json;
use rocket::{State, get};
use sea_orm::{DbConn, EntityTrait};
use services::category_service::CategoryService;
use services::service_trait::{ServiceFilter, ServiceTrait};

use crate::responder::Responder;

#[get("/")]
pub async fn get_all(db: &State<DbConn>) -> Result<Json<Vec<CategoryGetDto>>, Responder> {
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
#[get("/<id>")]
pub async fn get_single(id: i32, db: &State<DbConn>) -> Result<Json<CategoryGetDto>, Responder> {
    let db = db.inner();

    let service = CategoryService(db);

    let category = service
        .get_by_id(id)
        .await?
        .ok_or(Responder::not_found("User with the given id not found"))?;

    Ok(Json(category.into()))
}
