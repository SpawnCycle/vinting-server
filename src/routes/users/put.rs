use dtos::UserPutDto;
use migrations::constants::ADMIN_ROLE;
use rocket::{State, put, response::status::NoContent, serde::json::Json};
use sea_orm::{DbConn, TransactionTrait};
use services::{service_trait::ServiceTrait, user_service::UserService};

use crate::{jwt::JwtClaims, responder::Responder};

#[put("/<id>", format = "application/json", data = "<data>")]
pub async fn one(
    id: i32,
    claims: JwtClaims,
    db: &State<DbConn>,
    data: Json<UserPutDto>,
) -> Result<NoContent, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    let service = UserService(db);
    let user_dto = data.into_inner();

    if id != user_dto.id {
        return Err(Responder::bad_request(
            "The id in the url and the body don't match",
        ));
    }

    let user = service.get_by_id(id).await?.ok_or(Responder::not_found(
        "The user with the given id doesn't exist",
    ))?;

    if user.id != claims.uid && !claims.has_role(db, ADMIN_ROLE).await? {
        return Err(Responder::unauhorized("You can't change this user"));
    }

    if let Some(email) = user_dto.email.clone()
        && service.exists_by_email_all(email.clone()).await?
        // check if the caller is actually changing the email
        && email.to_string() != user.email
    {
        return Err(Responder::conflict(
            "You can't change your email to the given email",
        ));
    }

    let _ = service.update(user_dto).await?;

    trx.commit().await?;

    Ok(NoContent)
}
