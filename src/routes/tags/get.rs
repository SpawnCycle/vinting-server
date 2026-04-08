use dtos::TagGetDto;
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
    use dtos::TagGetDto;
    use rocket::local::asynchronous::Client;

    use super::*;
    use crate::testing;

    #[tokio::test]
    async fn tags_get_all_tracked() -> anyhow::Result<()> {
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

        // testing::tag::seed_db has 3 non-deleted rows
        assert_eq!(tags.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn tags_get_all_func() -> anyhow::Result<()> {
        let db = testing::db().await?;
        let db = State::from(&db);
        testing::tag::seed_db(db).await?;

        let tags = all(db).await?;

        // testing::tag::seed_db has 3 non-deleted rows
        assert_eq!(tags.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn tags_get_by_id_tracked_exists() -> anyhow::Result<()> {
        let db = testing::db().await?;
        testing::tag::seed_db(&db).await?;
        let r = testing::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let req = client.get("/api/tags/1");
        let res = req.dispatch().await;
        let tag = res.into_json::<TagGetDto>().await;

        assert!(tag.is_some(), "The first tag is not deleted");

        Ok(())
    }

    #[tokio::test]
    async fn tags_get_by_id_tracked_deleted() -> anyhow::Result<()> {
        let db = testing::db().await?;
        testing::tag::seed_db(&db).await?;
        let r = testing::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let req = client.get("/api/tags/5");
        let res = req.dispatch().await;

        assert_eq!(res.status().code, 404, "The fifth tag is deleted");

        Ok(())
    }

    #[tokio::test]
    async fn tags_get_by_id_tracked_not_existant() -> anyhow::Result<()> {
        let db = testing::db().await?;
        testing::tag::seed_db(&db).await?;
        let r = testing::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let req = client.get("/api/tags/1000");
        let res = req.dispatch().await;

        assert_eq!(res.status().code, 404, "The 1000th tag does not exist");

        Ok(())
    }

    #[tokio::test]
    async fn tags_get_by_id_func() -> anyhow::Result<()> {
        let db = testing::db().await?;
        let db = State::from(&db);
        testing::tag::seed_db(db).await?;

        {
            let res = one(1, db).await;
            assert!(res.is_ok(), "The first tag is not deleted");
        }

        {
            let res = one(2, db).await;
            assert!(res.is_ok(), "The second tag is not deleted");
        }

        {
            let res = one(5, db).await;
            assert!(res.is_err(), "The fifth tag is deleted");
        }

        {
            let res = one(1000, db).await;
            assert!(res.is_err(), "The 1000th tag does not exist");
        }

        Ok(())
    }
}
