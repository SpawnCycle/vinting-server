use dtos::product::{get::ProductGetDto, post::ProductPostDto};
use rocket::{
    State,
    http::{CookieJar, uri::Host},
    post,
    response::status::Created,
    serde::json::Json,
};
use sea_orm::DbConn;
use services::{product_service::ProductService, service_trait::ServiceTrait};

use crate::{
    constants::construct_host, jwt::JwtClaims, responder::Responder,
    routes::products::product_post_dto_to_am_with_associations,
};

#[post("/", format = "application/json", data = "<data>")]
pub async fn one(
    host: &Host<'_>,
    jar: &CookieJar<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
    data: Json<ProductPostDto>,
) -> Result<Created<Json<ProductGetDto>>, Responder> {
    let db = db.inner();
    let service = ProductService(db);

    if !claims.exists(db).await? {
        claims.remove_from(jar);
        return Err(Responder::unauhorized(
            "The user with id given in the cookie does not exists",
        ));
    }

    let model = service
        .insert(product_post_dto_to_am_with_associations(data.into_inner(), claims.uid, db).await?)
        .await?;
    let model = service
        .load_by_id(model.id)
        .await?
        .expect("The model was just created, it should exist");
    let host = construct_host(host);
    let id = model.id;
    let dto = ProductGetDto::from_model_with_host(model, &host)
        .ok_or(Responder::server_error("Could not construct the dto"))?;

    Ok(Created::new(format!("{host}/api/products/{id}")).body(Json(dto)))
}
