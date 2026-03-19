use dtos::product::put::ProductPutDto;
use rocket::{State, http::CookieJar, put, response::status::NoContent, serde::json::Json};
use sea_orm::DbConn;
use services::{product_service::ProductService, service_trait::ServiceTrait};

use crate::{
    jwt::JwtClaims, responder::Responder, routes::products::product_put_dto_to_am_with_associations,
};

#[put("/<id>", format = "application/json", data = "<data>")]
pub async fn one(
    id: i32,
    jar: &CookieJar<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
    data: Json<ProductPutDto>,
) -> Result<NoContent, Responder> {
    let db = db.inner();
    let service = ProductService(db);
    if !claims.verify_or_remove(db, jar).await? {
        return Err(Responder::unauhorized(
            "Your token is not valid, it has been removed",
        ));
    }

    if id != data.id {
        return Err(Responder::bad_request(
            "The id in the url and in the body of the request does not match",
        ));
    }

    let _ = service
        .update(product_put_dto_to_am_with_associations(data.into_inner(), claims.uid, db).await?)
        .await?;

    Ok(NoContent)
}
