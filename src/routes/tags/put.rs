use dtos::TagPutDto;
use rocket::{State, put, response::status::NoContent, serde::json::Json};
use sea_orm::{DbConn, TransactionTrait};
use services::{service_trait::ServiceTrait, tag_service::TagService};

use crate::responder::Responder;

#[put("/<id>", format = "application/json", data = "<data>")]
pub async fn one(
    id: i32,
    db: &State<DbConn>,
    data: Json<TagPutDto>,
) -> Result<NoContent, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    let service = TagService(db);
    let tag = data.into_inner();

    if id != tag.id {
        return Err(Responder::bad_request(
            "The id in the url does not match id in the body",
        ));
    }

    if !service.exists_by_id(id).await? {
        return Err(Responder::not_found("There is no tag with the given id"));
    }

    if service.exists_by_name(tag.data.name.clone()).await? {
        return Err(Responder::conflict(
            "There is already a tag with the given name",
        ));
    }

    let _ = service.update(tag).await?;

    trx.commit().await?;

    Ok(NoContent)
}

#[cfg(test)]
mod tests {
    use anyhow::anyhow;
    use dtos::{TagGetDto, TagPostDto};
    use rocket::{local::asynchronous::Client, serde::json};

    use super::*;
    use crate::testing;

    #[tokio::test]
    async fn tags_put_tracked_successful_modification() -> anyhow::Result<()> {
        let db = testing::db().await?;
        testing::tag::seed_db(&db).await?;
        let r = testing::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let tag_text = "Tag 1 modified";
        let dto = TagPutDto {
            id: 1,
            data: TagPostDto {
                name: tag_text.to_string(),
            },
        };
        let req = testing::json_request(client.put("/api/tags/1").body(json::to_string(&dto)?));

        let res = req.dispatch().await;

        assert_eq!(
            res.status().code,
            204,
            "This should modify the tag successfully"
        );

        let req = client.get("/api/tags/1");
        let res = req.dispatch().await;
        let tag = res
            .into_json::<TagGetDto>()
            .await
            .ok_or(anyhow!("This tag should exist"))?;

        assert_eq!(tag.name, tag_text);

        Ok(())
    }

    #[tokio::test]
    async fn tags_put_tracked_bad_request() -> anyhow::Result<()> {
        let db = testing::db().await?;
        testing::tag::seed_db(&db).await?;
        let r = testing::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let tag_text = "Tag 2";
        let dto = TagPutDto {
            id: 1,
            data: TagPostDto {
                name: tag_text.to_string(),
            },
        };
        let req = testing::json_request(client.put("/api/tags/2").body(json::to_string(&dto)?));

        let res = req.dispatch().await;

        assert_eq!(res.status().code, 403, "This should return a bad request");

        Ok(())
    }

    #[tokio::test]
    async fn tags_put_tracked_doesnt_exist() -> anyhow::Result<()> {
        let db = testing::db().await?;
        testing::tag::seed_db(&db).await?;
        let r = testing::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let tag_text = "Tag 100";
        let dto = TagPutDto {
            id: 100,
            data: TagPostDto {
                name: tag_text.to_string(),
            },
        };
        let req = testing::json_request(client.put("/api/tags/100").body(json::to_string(&dto)?));

        let res = req.dispatch().await;

        assert_eq!(res.status().code, 404, "This tag doesn't exist");

        Ok(())
    }

    #[tokio::test]
    async fn tags_put_func_successful_modification() -> anyhow::Result<()> {
        let db = testing::db().await?;
        let db = State::from(&db);
        testing::tag::seed_db(db).await?;

        let tag_text = "Tag 1 modified";
        let dto = TagPutDto {
            id: 1,
            data: TagPostDto {
                name: tag_text.to_string(),
            },
        };
        let res = one(1, db, Json(dto)).await;

        assert!(res.is_ok(), "This should create a new tag successfully");

        Ok(())
    }

    #[tokio::test]
    async fn tags_put_func_bad_request() -> anyhow::Result<()> {
        let db = testing::db().await?;
        let db = State::from(&db);
        testing::tag::seed_db(db).await?;

        let dto = TagPutDto {
            id: 1,
            data: TagPostDto {
                name: "Tag 2".to_string(),
            },
        };
        let res = one(1, db, Json(dto)).await;

        assert!(res.is_err(), "This should create a bad request");

        Ok(())
    }

    #[tokio::test]
    async fn tags_put_func_doesnt_exist() -> anyhow::Result<()> {
        let db = testing::db().await?;
        let db = State::from(&db);
        testing::tag::seed_db(db).await?;

        let dto = TagPutDto {
            id: 1000,
            data: TagPostDto {
                name: "Tag 2".to_string(),
            },
        };
        let res = one(1000, db, Json(dto)).await;

        assert!(res.is_err(), "This should create a not found");

        Ok(())
    }
}
