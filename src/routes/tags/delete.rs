use rocket::{State, delete, response::status::NoContent};
use sea_orm::DbConn;
use services::{service_trait::ServiceTrait, tag_service::TagService};

use crate::responder::Responder;

#[delete("/<id>")]
pub async fn one(id: i32, db: &State<DbConn>) -> Result<NoContent, Responder> {
    let db = db.inner();
    let service = TagService(db);

    if !service.exists_by_id(id).await? {
        return Err(Responder::not_found("There is no tag with the given id."));
    }

    let _ = service.delete_by_id(id).await?;

    Ok(NoContent)
}

#[cfg(test)]
mod tests {
    use dtos::TagPostDto;
    use rocket::{local::asynchronous::Client, serde::json};

    use super::*;
    use crate::testing::{self, tag};

    #[tokio::test]
    async fn tags_delete_tracked() -> anyhow::Result<()> {
        let db = tag::db().await?;
        let r = tag::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let req = client.delete("/api/tags/1");
        let res = req.dispatch().await;

        assert_eq!(
            res.status().code,
            204,
            "This should delete the tag succesfully"
        );

        let req = client.get("/api/tags/1");
        let res = req.dispatch().await;

        assert_eq!(
            res.status().code,
            404,
            "The tag should be inaccessible after deletion"
        );

        let req = client.delete("/api/tags/1");
        let res = req.dispatch().await;

        assert_eq!(res.status().code, 404, "The tag shouldn't be deletable");

        Ok(())
    }

    #[tokio::test]
    async fn tags_delete_func() -> anyhow::Result<()> {
        let db = tag::db().await?;
        let db = State::from(&db);

        let tags = one(1, db).await;

        assert!(tags.is_ok(), "The tag should be deleted succesfully");

        Ok(())
    }

    #[tokio::test]
    async fn tags_post_deleted() -> anyhow::Result<()> {
        let db = tag::db().await?;
        let r = tag::rocket(db).await?;

        let client = Client::tracked(r).await?;

        let req = client.delete("/api/tags/1");
        let res = req.dispatch().await;

        assert_eq!(
            res.status().code,
            204,
            "This should delete the tag succesfully"
        );

        let req = client.get("/api/tags/1");
        let res = req.dispatch().await;

        assert_eq!(
            res.status().code,
            404,
            "The tag should be inaccessible after deletion"
        );

        let dto = TagPostDto {
            name: "Tag 1".to_string(),
        };

        let req = testing::json_request(client.post("/api/tags/").body(json::to_string(&dto)?));
        let res = req.dispatch().await;

        assert_eq!(res.status().code, 201);

        Ok(())
    }
}
