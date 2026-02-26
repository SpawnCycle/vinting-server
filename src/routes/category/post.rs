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

use crate::responder::Responder;

#[post("/", format = "application/json", data = "<data>")]
pub async fn post(
    db: &State<DbConn>,
    host: &Host<'_>,
    data: Json<CategoryPostDto>,
) -> Result<Created<Json<CategoryGetDto>>, Responder> {
    let db = db.inner();
    let data = data.into_inner();
    let data_entry = category::ActiveModelEx::from(data);

    let return_var = data_entry.insert(db).await?;

    Ok(
        Created::new(format!("{host}/api/category/{}", return_var.id))
            .body(Json(return_var.into())),
    )
}
