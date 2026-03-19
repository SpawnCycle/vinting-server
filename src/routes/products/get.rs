use dtos::product::get::ProductGetDto;
use rocket::{State, get, http::uri::Host, serde::json::Json};
use sea_orm::DbConn;
use services::product_service::ProductService;

use crate::{constants::construct_host, responder::Responder};

#[get("/<id>")]
pub async fn one(
    id: i32,
    host: &Host<'_>,
    db: &State<DbConn>,
) -> Result<Json<ProductGetDto>, Responder> {
    let db = db.inner();
    let service = ProductService(db);
    let host = construct_host(host);

    let product = service
        .load_by_id(id)
        .await?
        .map(|p| {
            ProductGetDto::from_model_with_host(p, &host)
                .ok_or(Responder::server_error("Could not properly load the model"))
        })
        .ok_or(Responder::not_found(
            "Couldn't find a product with the given id",
        ))??;

    Ok(Json(product))
}

#[get("/")]
pub async fn all(
    host: &Host<'_>,
    db: &State<DbConn>,
) -> Result<Json<Vec<ProductGetDto>>, Responder> {
    let db = db.inner();
    let service = ProductService(db);
    let host = construct_host(host);

    let products = service
        .load_all_mutating(|p| {
            ProductGetDto::from_model_with_host(p, &host)
                .expect("The model from the service should be properly loaded")
        })
        .await?;

    Ok(Json(products))
}
