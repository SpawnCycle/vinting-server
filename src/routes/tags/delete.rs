use rocket::{State, delete, response::status::NoContent};
use sea_orm::DbConn;
use services::{service_trait::ServiceTrait, tag_service::TagService};

use crate::responder::Responder;

#[delete("/<id>")]
pub async fn one(db: &State<DbConn>, id: i32) -> Result<NoContent, Responder> {
    let db = db.inner();
    let service = TagService(db);

    if !service.exists_by_id(id).await? {
        return Err(Responder::not_found("There is no tag with the given id."));
    }

    let _ = service.delete_by_id(id).await?;

    Ok(NoContent)
}
