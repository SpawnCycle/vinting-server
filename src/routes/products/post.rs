use dtos::product::{get::ProductGetDto, post::ProductPostDto};
use entity::{prelude::*, product};
use rocket::{
    State,
    http::{CookieJar, uri::Host},
    post,
    response::status::Created,
    serde::json::Json,
};
use sea_orm::{DbConn, EntityTrait, IntoActiveModel};
use services::{
    category_service::CategoryService, image_service::ImageService,
    product_service::ProductService, service_trait::ServiceTrait, tag_service::TagService,
};

use crate::{constants::construct_host, jwt::JwtClaims, responder::Responder};

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
        .insert(dto_to_am_with_associations(claims.uid, data.into_inner(), db).await?)
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

// Setting the active model id to the id and trying to insert with it doesn't work,
// so this is the way to go, very sad
pub async fn dto_to_am_with_associations(
    uid: i32,
    dto: ProductPostDto,
    db: &DbConn,
) -> Result<product::ActiveModelEx, Responder> {
    let mut am = dto.clone().into_active_model(uid);
    let c_service = CategoryService(db);
    let t_service = TagService(db);
    let i_service = ImageService(db);

    for c_id in dto.categories {
        let c = c_service
            .get_by_id(c_id)
            .await?
            .ok_or(Responder::not_found(format!(
                "There is no category with the id of {c_id}"
            )))?;
        am = am.add_category(c.into_active_model());
    }

    for t_id in dto.tags {
        let t = t_service
            .get_by_id(t_id)
            .await?
            .ok_or(Responder::not_found(format!(
                "There is no tag with the id of {t_id}"
            )))?;
        am = am.add_tag(t.into_active_model());
    }

    for i_id in dto.images {
        let i = i_service
            .get_by_id(i_id)
            .await?
            .ok_or(Responder::not_found(format!(
                "There is no image with the id of {i_id}"
            )))?;
        am = am.add_image(i.into_active_model());
    }

    Ok(am)
}
