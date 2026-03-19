use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io,
};

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

use crate::{constants::construct_host, jwt::JwtClaims, responder::Responder};

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
    if !claims.exists_or_remove(db, jar).await? {
        return Err(Responder::unauhorized(
            "Your token is incorrect, it has been removed",
        ));
    }

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

async fn save_image(image: &mut TempFile<'_>) -> Result<String, io::Error> {
    let mut hasher = DefaultHasher::new();

    // the more random stuff to hash, the better
    image.len().hash(&mut hasher);
    image.path().hash(&mut hasher);
    if let Some(b) = image.name() {
        b.hash(&mut hasher)
    }

    let hash = hasher.finish();
    let out = const_hex::display(hash.to_ne_bytes()).to_string();
    let uri = format!("./img/{out}.png");

    image.move_copy_to(uri.clone()).await?;
    Ok(uri.to_string())
}
