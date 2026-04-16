use entity::{active_action::ActiveAction, tag};
use rocket::{Build, Rocket};
use sea_orm::{ActiveValue::Set, DbConn, DbErr, TransactionTrait};

use crate::routes::TagsFairing;

pub async fn db() -> Result<DbConn, DbErr> {
    let db = super::db().await?;

    seed_db(&db).await?;

    Ok(db)
}

pub async fn rocket(db: DbConn) -> Result<Rocket<Build>, rocket::Error> {
    let r = super::rocket(db).await?.attach(TagsFairing);

    Ok(r)
}

pub async fn seed_db(db: &DbConn) -> Result<(), DbErr> {
    let trx = db.begin().await?;
    let db = &trx;

    tag::ActiveModel {
        name: Set("Tag 1".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .insert(db)
    .await?;

    tag::ActiveModel {
        name: Set("Tag 2".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .insert(db)
    .await?;

    tag::ActiveModel {
        name: Set("Tag 3".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .insert(db)
    .await?;

    tag::ActiveModel {
        name: Set("Tag deleted 1".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .deleting()
    .insert(db)
    .await?;

    tag::ActiveModel {
        name: Set("Tag deleted 2".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .deleting()
    .insert(db)
    .await?;

    tag::ActiveModel {
        name: Set("Tag deleted 3".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .deleting()
    .insert(db)
    .await?;

    trx.commit().await?;

    Ok(())
}
