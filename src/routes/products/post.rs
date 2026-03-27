use dtos::product::{get::ProductGetDto, post::ProductPostDto};
use entity::{
    active_action::ActiveAction,
    image,
    product::{self, ProductCondition, ProductSex},
    tag,
};
use rocket::{
    FromForm, State,
    data::ToByteUnit,
    form::Form,
    fs::TempFile,
    http::{ContentType, CookieJar, uri::Host},
    post,
    response::status::Created,
    serde::json::Json,
};
use sea_orm::{DbConn, IntoActiveModel, TransactionTrait};
use services::{
    category_service::CategoryService, product_service::ProductService,
    service_trait::ServiceTrait, tag_service::TagService,
};

use crate::{
    constants::construct_host,
    jwt::JwtClaims,
    responder::Responder,
    routes::{products::product_post_dto_to_am_with_associations, save_image},
};

fn valid_file_types(files: &Vec<TempFile<'_>>) -> bool {
    files
        .iter()
        .all(|f| rocket::form::validate::ext(f, ContentType::PNG).is_ok())
}

fn valid_file_sizes(files: &Vec<TempFile<'_>>) -> bool {
    files
        .iter()
        .all(|f| rocket::form::validate::len(f, ..5.mebibytes()).is_ok())
}

#[derive(Debug, FromForm)]
pub struct ProductForm<'a> {
    title: String,
    description: String,
    brand: String,
    categories: Vec<String>,
    tags: Vec<String>,
    condition: String,
    gender: String,
    size: String,
    color: String,
    price: u32,
    #[field(validate = with(valid_file_types, "One of the files has an unsupported file type"))]
    #[field(validate = with(valid_file_sizes, "One of the files is too large"))]
    images: Vec<TempFile<'a>>,
}

#[post("/", data = "<data>", rank = 2)]
pub async fn form(
    host: &Host<'_>,
    jar: &CookieJar<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
    data: Form<ProductForm<'_>>,
) -> Result<Created<Json<ProductGetDto>>, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    let host = construct_host(host);
    claims.exists_or_remove(db, jar).await?;
    let mut data = data.into_inner();

    let c_service = CategoryService(db);
    let p_service = ProductService(db);
    let t_service = TagService(db);
    let user = claims
        .fetch(db)
        .await?
        .ok_or(Responder::not_found("Your user was not found"))?;

    let condition = ProductCondition::try_from(data.condition.as_str())
        .map_err(|_| Responder::bad_request("Unsupported condition given"))?;
    let sex = ProductSex::try_from(data.gender.as_str())
        .map_err(|_| Responder::bad_request("Unsupported gender given"))?;

    let mut am = product::ActiveModelEx::new()
        .set_seller_id(user.id)
        .set_has_stock(true)
        .set_name(data.title)
        .set_description(data.description)
        .set_brand(data.brand)
        .set_condition(condition)
        .set_sex(sex)
        .set_size(data.size)
        .set_color(data.color)
        .set_price(data.price);

    for c_name in data.categories {
        let c = c_service
            .get_by_name(&c_name)
            .await?
            .ok_or(Responder::not_found(format!(
                "There is no category named {c_name}"
            )))?;
        am = am.add_category(c.into_active_model());
    }

    for t_name in data.tags {
        let t = t_service
            .get_by_name(&t_name)
            .await?
            .map_or(tag::ActiveModelEx::new().set_name(t_name).creating(), |m| {
                m.into_active_model().into_ex()
            });
        am = am.add_tag(t);
    }

    for img in data.images.iter_mut() {
        let uri = save_image(img).await.map_err(|err| {
            Responder::server_error(if cfg!(debug_assertions) {
                format!("There was an error while saving the file: {err}")
            } else {
                "There was an error while saving the file".to_string()
            })
        })?;

        let img_am = image::ActiveModelEx::new()
            .creating()
            .set_user_id(user.id)
            .set_path(uri);
        am = am.add_image(img_am);
    }

    let model = p_service.insert(am).await?;
    // We need to load it again, because if there are no fields for one of the m-n tables,
    // it will return that HasMany value as unloaded
    let model = p_service
        .load_by_id(model.id)
        .await?
        .expect("We just created it");
    let id = model.id;

    let dto = ProductGetDto::from_model_with_host(model, &host)
        .ok_or(Responder::server_error("Could not create the product"))?;

    trx.commit().await?;

    Ok(Created::new(format!("{}/api/products/{id}", host)).body(dto.into()))
}

#[post("/", format = "application/json", data = "<data>")]
pub async fn one(
    host: &Host<'_>,
    jar: &CookieJar<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
    data: Json<ProductPostDto>,
) -> Result<Created<Json<ProductGetDto>>, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    let service = ProductService(db);
    claims.exists_or_remove(db, jar).await?;

    let model = service
        .insert(product_post_dto_to_am_with_associations(&data, claims.uid, db).await?)
        .await?;
    let model = service
        .load_by_id(model.id)
        .await?
        .expect("The model was just created, it should exist");
    let host = construct_host(host);
    let id = model.id;
    let dto = ProductGetDto::from_model_with_host(model, &host)
        .ok_or(Responder::server_error("Could not construct the dto"))?;

    trx.commit().await?;

    Ok(Created::new(format!("{host}/api/products/{id}")).body(Json(dto)))
}
