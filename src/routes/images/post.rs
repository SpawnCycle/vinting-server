use dtos::image::get::ImageGetDto;
use entity::{active_action::ActiveAction, image};
use rocket::{
    FromForm, State,
    data::ToByteUnit,
    form::Form,
    fs::TempFile,
    http::{ContentType, CookieJar, uri::Host},
    post,
    serde::json::Json,
};
use sea_orm::DbConn;
use services::{image_service::ImageService, service_trait::ServiceTrait};

use crate::{constants::construct_host, jwt::JwtClaims, responder::Responder, routes::save_image};

#[derive(Debug, FromForm)]
pub struct ImageForm<'a> {
    #[field(validate = ext(ContentType::PNG))]
    #[field(validate = len(..5.mebibytes()))]
    image: TempFile<'a>,
}

#[post("/", data = "<form>")]
pub async fn upload(
    host: &Host<'_>,
    jar: &CookieJar<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
    mut form: Form<ImageForm<'_>>,
) -> Result<Json<ImageGetDto>, Responder> {
    let db = db.inner();
    claims.exists_or_unauthorized(db, jar).await?;

    let uri = save_image(&mut form.image).await?;
    let service = ImageService(db);
    let m = service
        .insert(
            image::ActiveModelEx::new()
                .creating()
                .set_path(uri)
                .set_user_id(claims.uid),
        )
        .await?;

    Ok(Json(ImageGetDto::from_model_with_host(
        m,
        &construct_host(host),
    )))
}
