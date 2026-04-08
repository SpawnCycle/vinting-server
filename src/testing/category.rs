use entity::{active_action::ActiveAction, category};
use rocket::{Build, Rocket};
use sea_orm::{ActiveValue::Set, DbConn, DbErr, TransactionTrait};

use crate::routes::CategoriesFairing;

pub async fn db() -> Result<DbConn, DbErr> {
    let db = super::db().await?;

    seed_db(&db).await?;

    Ok(db)
}

pub async fn rocket(db: DbConn) -> Result<Rocket<Build>, rocket::Error> {
    let r = super::rocket(db).await?.attach(CategoriesFairing);

    Ok(r)
}

pub async fn seed_db(db: &DbConn) -> Result<(), DbErr> {
    let trx = db.begin().await?;
    let db = &trx;

    category::ActiveModel {
        name: Set("Category 1".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .insert(db)
    .await?;

    category::ActiveModel {
        name: Set("Category 2".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .insert(db)
    .await?;

    category::ActiveModel {
        name: Set("Category 3".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .insert(db)
    .await?;

    category::ActiveModel {
        name: Set("Category deleted 1".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .deleting()
    .insert(db)
    .await?;

    category::ActiveModel {
        name: Set("Category deleted 2".to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
    .deleting()
    .insert(db)
    .await?;

    category::ActiveModel {
        name: Set("Category deleted 3".to_string()),
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
