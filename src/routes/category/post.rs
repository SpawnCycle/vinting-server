use dtos::category::{get::CategoryGetDto, post::CategoryPostDto};
use entity::category;
use rocket::{
    State,
    http::uri::Host,
    post,
    response::status::{Created, NoContent},
    serde::json::Json,
};
use sea_orm::DbConn;
use services::{category_service::CategoryService, service_trait::ServiceTrait};

use crate::responder::Responder;

#[post("/", format = "application/json", data = "<data>")]
pub async fn post(
    db: &State<DbConn>,
    host: &Host<'_>,
    data: Json<CategoryPostDto>,
) -> Result<Created<Json<CategoryGetDto>>, Responder> {
    let db = db.inner();
    let category = data.into_inner();
    let service = CategoryService(db);

    let model = service.insert(category).await?;

    Ok(Created::new(format!("{host}/api/categories/{}", model.id)).body(Json(model.into())))
}
