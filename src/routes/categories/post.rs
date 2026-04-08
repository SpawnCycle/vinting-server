use dtos::{CategoryGetDto, CategoryPostDto};
use entity::{active_action::ActiveAction, category};
use migrations::constants::ADMIN_ROLE;
use rocket::{State, http::uri::Host, post, response::status::Created, serde::json::Json};
use sea_orm::DbConn;
use services::{category_service::CategoryService, service_trait::ServiceTrait};

use crate::{constants::construct_host, jwt::JwtClaims, responder::Responder};

#[post("/", format = "application/json", data = "<data>")]
pub async fn one(
    host: &Host<'_>,
    claims: JwtClaims,
    db: &State<DbConn>,
    data: Json<CategoryPostDto>,
) -> Result<Created<Json<CategoryGetDto>>, Responder> {
    let db = db.inner();

    if !claims.has_role(db, ADMIN_ROLE).await? {
        return Err(Responder::unauhorized(
            "Category Modifications are only allowed for admin users",
        ));
    }

    let service = CategoryService(db);
    let category = data.into_inner();

    if service.exists_by_name(&category.name).await? {
        return Err(Responder::bad_request("The name already exists"));
    }

    let model = match service.get_by_name_all(&category.name).await? {
        Some(existing_category) => {
            let am = category::ActiveModelEx::from(category)
                .set_id(existing_category.id)
                .creating();

            service.insert(am).await?
        }
        None => service.insert(category).await?,
    };

    Ok(Created::new(format!(
        "{}/api/categories/{}",
        construct_host(host),
        model.id
    ))
    .body(Json(model.into())))
}
