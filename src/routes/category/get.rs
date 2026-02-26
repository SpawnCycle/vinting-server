use dtos::category::get::CategoryGetDto;
use entity::prelude::Category;
use rocket::serde::json::Json;
use rocket::{State, get};
use sea_orm::{DbConn, EntityTrait};
use services::category_service::CategoryService;
use services::service_trait::ServiceFilter;

use crate::responder::Responder;

#[get("/")]
pub async fn get_all(db: &State<DbConn>) -> Result<Json<Vec<CategoryGetDto>>, Responder> {
    let db = db.inner();
    let dtovec: Vec<CategoryGetDto> = Category::find()
        .service_filter::<CategoryService>()
        .all(db)
        .await?
        .iter()
        .map(|val| val.clone().into())
        .collect();

    Ok(Json(dtovec))
}
#[get("/<id>")]
pub async fn get_single(id: i32, db: &State<DbConn>) -> Result<Json<CategoryGetDto>, Responder> {
    let db = db.inner();
    match Category::find_by_id(id)
        .service_filter::<CategoryService>()
        .one(db)
        .await?
        .iter()
        .map(|val| val.clone().into())
        .collect::<Vec<CategoryGetDto>>()
        .first()
    {
        Some(val) => Ok(Json(val.clone())),
        None => Err(Responder::BadRequest(String::from("id does not exist"))),
    }
}
