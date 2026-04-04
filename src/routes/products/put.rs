use dtos::product::put::ProductPutDto;
use rocket::{State, http::CookieJar, put, response::status::NoContent, serde::json::Json};
use sea_orm::{DbConn, TransactionTrait};
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
    let trx = db.begin().await?;
    let db = &trx;
    let service = ProductService(db);
    let data = data.into_inner();
    claims.exists_or_unauthorized(db, jar).await?;

    if id != data.id {
        return Err(Responder::bad_request(
            "The id in the url and in the body of the request does not match",
        ));
    }

    let product = service.get_by_id(id).await?.ok_or(Responder::not_found(
        "Couldn't find a product with the given id",
    ))?;

    if product.seller_id != claims.uid {
        return Err(Responder::unauhorized("You can't modify others' products"));
    }

    let product = product_put_dto_to_am_with_associations(&data, claims.uid, db).await?;

    let _ = service.update(product).await?;

    trx.commit().await?;

    Ok(NoContent)
}
