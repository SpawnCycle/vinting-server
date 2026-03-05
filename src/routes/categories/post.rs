use dtos::category::{get::CategoryGetDto, post::CategoryPostDto};
use rocket::{State, http::uri::Host, post, response::status::Created, serde::json::Json};
use sea_orm::DbConn;
use serde::ser::Error;
use services::{category_service::CategoryService, service_trait::ServiceTrait};

use crate::responder::Responder;

#[post("/", data = "<data>")]
pub async fn one(
    db: &State<DbConn>,
    data: Json<CategoryPostDto>,
    host: &Host<'_>,
) -> Result<Created<Json<CategoryGetDto>>, Responder> {
    let db = db.inner();
    let service = CategoryService(db);
    let category = data.into_inner();

    if service.exists_by_name_all(&category.name).await? {
        return Err(Responder::bad_request("The name already exists"));
    }

    let model = service.insert(category).await?;

    Ok(Created::new(format!("{host}/api/categories/{}", model.id)).body(Json(model.into())))
}
