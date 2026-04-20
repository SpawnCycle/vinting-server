use entity::{active_action::ActiveAction, category};
use rocket::{Build, Rocket};
use sea_orm::{ActiveValue::Set, DbConn, DbErr, TransactionTrait};

use crate::routes::{CategoriesFairing, UsersFairing};

pub async fn db() -> Result<DbConn, DbErr> {
    let db = super::db().await?;

    seed_db(&db).await?;
    super::setup_users(&db).await?;

    Ok(db)
}

pub async fn rocket(db: DbConn) -> Result<Rocket<Build>, rocket::Error> {
    let r = super::rocket(db)
        .await?
        .attach(CategoriesFairing)
        .attach(UsersFairing);

    Ok(r)
}

pub async fn seed_db(db: &DbConn) -> Result<(), DbErr> {
    let trx = db.begin().await?;
    let db = &trx;

    make_category("Category 1").insert(db).await?;

    make_category("Category 2").insert(db).await?;

    make_category("Category 3").insert(db).await?;

    make_category("Category deleted 1")
        .deleting()
        .insert(db)
        .await?;

    make_category("Category deleted 2")
        .deleting()
        .insert(db)
        .await?;

    make_category("Category deleted 3")
        .deleting()
        .insert(db)
        .await?;

    trx.commit().await?;

    Ok(())
}

fn make_category(name: impl ToString) -> category::ActiveModelEx {
    category::ActiveModel {
        name: Set(name.to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
}
