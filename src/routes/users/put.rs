use dtos::UserPutDto;
use entity::{active_action::ActiveAction, user};
use rocket::{State, put, response::status::NoContent, serde::json::Json};
use sea_orm::{DbConn, TransactionTrait};
use services::{service_trait::ServiceTrait, user_service::UserService};

use crate::responder::Responder;

#[put("/<id>", format = "application/json", data = "<data>")]
pub async fn one(
    id: i32,
    db: &State<DbConn>,
    data: Json<UserPutDto>,
) -> Result<NoContent, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    let service = UserService(db);
    let user = data.into_inner();
    let email = user.data.email.clone();

    if id != user.id {
        return Err(Responder::bad_request(
            "The id in the url and the body don't match",
        ));
    }

    if service.exists_by_email(email.to_owned()).await? {
        return Err(Responder::conflict("A user with that email already exists"));
    }

    let am = user::ActiveModelEx::from(user);
    match service.get_by_email(email.to_owned()).await? {
        Some(existing_user) => {
            service
                .update(am.set_id(existing_user.id).set_deleted_at(None).creating())
                .await?;
        }
        None => {
            service.insert(am).await?;
        }
    }

    Ok(NoContent)
}
