use chrono::Utc;
use entity::{active_action::ActiveAction, tag};
use rocket::{State, delete, response::status::NoContent};
use sea_orm::DbConn;
use services::{service_trait::ServiceTrait, tag_service::TagService};

use crate::responder::Responder;

#[delete("/<id>")]
pub async fn delete(id: i32, db: &State<DbConn>) -> Result<NoContent, Responder> {
    let db = db.inner();
    let service = TagService(db);
    let now = Utc::now().naive_local();

    let _ = service
        .update(
            tag::ActiveModel::builder()
                .modifying()
                .set_id(id)
                .set_deleted_at(now),
        )
        .await?;

    Ok(NoContent)
}
