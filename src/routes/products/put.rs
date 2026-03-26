use dtos::product::put::ProductPutDto;
use entity::{category, image, prelude::*, product_category, product_image, product_tag, tag};
use rocket::{State, http::CookieJar, put, response::status::NoContent, serde::json::Json};
use sea_orm::{ColumnTrait, DbConn, EntityTrait, ModelTrait, QueryFilter, QuerySelect};
use services::{product_service::ProductService, service_trait::ServiceTrait};

use crate::{
    jwt::JwtClaims,
    responder::Responder,
    routes::{id_model::IdModel, products::product_put_dto_to_am_with_associations},
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
    if !claims.exists_or_remove(db, jar).await? {
        return Err(Responder::unauhorized(
            "Your token is not valid, it has been removed",
        ));
    }

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

    // because active model doesn't delete the extra connections
    let categories = product
        .find_related(Category)
        .select_only()
        .column(category::Column::Id)
        .into_model::<IdModel>()
        .all(db)
        .await?
        .into_iter()
        .map(|m| m.id)
        .filter(|t| !data.data.categories.contains(t));

    let tags = product
        .find_related(Tag)
        .select_only()
        .column(tag::Column::Id)
        .into_model::<IdModel>()
        .all(db)
        .await?
        .into_iter()
        .map(|m| m.id)
        .filter(|t| !data.data.tags.contains(t));

    let images = product
        .find_related(Image)
        .select_only()
        .column(image::Column::Id)
        .into_model::<IdModel>()
        .all(db)
        .await?
        .into_iter()
        .map(|m| m.id)
        .filter(|t| !data.data.images.contains(t));

    let product = product_put_dto_to_am_with_associations(&data, claims.uid, db).await?;

    let _ = ProductTag::delete_many()
        .filter(product_tag::Column::ProductId.eq(id))
        .filter(product_tag::Column::TagId.is_in(tags))
        .exec(db)
        .await?;

    let _ = ProductCategory::delete_many()
        .filter(product_category::Column::ProductId.eq(id))
        .filter(product_category::Column::CategoryId.is_in(categories))
        .exec(db)
        .await?;

    let _ = ProductImage::delete_many()
        .filter(product_image::Column::ProductId.eq(id))
        .filter(product_image::Column::ImageId.is_in(images))
        .exec(db)
        .await?;

    let _ = service.update(product).await?;

    Ok(NoContent)
}
