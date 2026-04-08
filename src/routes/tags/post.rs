use dtos::{TagGetDto, TagPostDto};
use rocket::{State, http::uri::Host, post, response::status::Created, serde::json::Json};
use sea_orm::{DbConn, TransactionTrait};
use services::{service_trait::ServiceTrait, tag_service::TagService};

use crate::{constants::construct_host, responder::Responder};

#[post("/", format = "application/json", data = "<data>")]
pub async fn one(
    host: &Host<'_>,
    db: &State<DbConn>,
    data: Json<TagPostDto>,
) -> Result<Created<Json<TagGetDto>>, Responder> {
    let trx = db.begin().await?;
    let db = &trx;
    let service = TagService(db);
    let tag = data.into_inner();

    if service.exists_by_name(&tag.name).await? {
        return Err(Responder::conflict("Tag with the same name already exists"));
    }

    let model = service.insert(tag).await?;

    trx.commit().await?;

    Ok(
        Created::new(format!("{}/api/tags/{}", construct_host(host), model.id))
            .body(Json(model.into())),
    )
}

#[cfg(test)]
mod tests {
    use dtos::TagPostDto;
    use rocket::{local::asynchronous::Client, serde::json, uri};

    use super::*;
    use crate::testing;

    #[tokio::test]
    async fn tags_post_tracked_unique() -> anyhow::Result<()> {
        let db = testing::db().await?;
        testing::tag::seed_db(&db).await?;
        let r = testing::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let dto = TagPostDto {
            name: "Tag unique".to_string(),
        };
        let req = testing::json_request(client.post("/api/tags/").body(json::to_string(&dto)?));

        let res = req.dispatch().await;

        assert_eq!(
            res.status().code,
            201,
            "This should create a new tag successfully"
        );

        Ok(())
    }

    #[tokio::test]
    async fn tags_post_tracked_conflict() -> anyhow::Result<()> {
        let db = testing::db().await?;
        testing::tag::seed_db(&db).await?;
        let r = testing::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let dto = TagPostDto {
            name: "Tag 1".to_string(),
        };
        let req = testing::json_request(client.post("/api/tags/").body(json::to_string(&dto)?));
        let res = req.dispatch().await;

        assert_eq!(
            res.status().code,
            409,
            "This should create a conflict, because 'Tag 1' already exists"
        );

        Ok(())
    }

    #[tokio::test]
    async fn tags_post_func_unique() -> anyhow::Result<()> {
        let db = testing::db().await?;
        let db = State::from(&db);
        testing::tag::seed_db(db).await?;
        let host = Host::new(uri!("localhost:8000"));

        let dto = TagPostDto {
            name: "Tag unique".to_string(),
        };
        let res = one(&host, db, Json(dto)).await;

        assert!(res.is_ok(), "This should create a new tag successfully");

        Ok(())
    }

    #[tokio::test]
    async fn tags_post_func_conflict() -> anyhow::Result<()> {
        let db = testing::db().await?;
        let db = State::from(&db);
        testing::tag::seed_db(db).await?;
        let host = Host::new(uri!("localhost:8000"));

        let dto = TagPostDto {
            name: "Tag 1".to_string(),
        };
        let res = one(&host, db, Json(dto)).await;

        assert!(
            res.is_err(),
            "This should create a conflict, because 'Tag 1' already exists"
        );

        Ok(())
    }
}
