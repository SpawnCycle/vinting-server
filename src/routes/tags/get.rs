use dtos::tag::get::TagGetDto;
use rocket::{State, get, serde::json::Json};
use sea_orm::DbConn;
use services::{service_trait::ServiceTrait, tag_service::TagService};

use crate::responder::Responder;

#[get("/<id>")]
pub async fn one(id: i32, db: &State<DbConn>) -> Result<Json<TagGetDto>, Responder> {
    let db = db.inner();
    let service = TagService(db);

    let tag = service
        .get_by_id(id)
        .await?
        .ok_or(Responder::not_found("Provided tag id does not exist."))?;

    Ok(Json(tag.into()))
}

#[get("/")]
pub async fn all(db: &State<DbConn>) -> Result<Json<Vec<TagGetDto>>, Responder> {
    let db = db.inner();
    let service = TagService(db);

    let tags = service.get_all_mapping(TagGetDto::from).await?;

    Ok(Json(tags))
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use dtos::tag::get::TagGetDto;
    use rocket::local::asynchronous::Client;

    use super::*;
    use crate::testing;

    #[tokio::test]
    async fn get_all_tags_tracked() -> anyhow::Result<()> {
        let db = testing::db().await?;
        testing::tag::seed_db(&db).await?;
        let r = testing::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let req = client.get("/api/tags/");
        let res = req.dispatch().await;
        let tags = res
            .into_json::<Vec<TagGetDto>>()
            .await
            .ok_or(anyhow!("Couldn't parse into json"))?;

        assert_eq!(tags.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn get_all_tags_func() -> anyhow::Result<()> {
        let db = testing::db().await?;
        let db = State::from(&db);
        testing::tag::seed_db(db).await?;

        let tags = all(db).await?;

        assert_eq!(tags.len(), 3);

        Ok(())
    }
}
