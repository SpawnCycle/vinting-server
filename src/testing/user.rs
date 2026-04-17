use argon2::{Argon2, PasswordHasher};
use dtos::get_argon_params;
use entity::{active_action::ActiveAction, user};
use rocket::{Build, Rocket};
use sea_orm::{ActiveValue::Set, DbConn, DbErr, TransactionTrait};

use crate::routes::UsersFairing;

pub async fn db() -> Result<DbConn, DbErr> {
    let db = super::db().await?;

    seed_db(&db).await?;

    Ok(db)
}

pub async fn rocket(db: DbConn) -> Result<Rocket<Build>, rocket::Error> {
    let r = super::rocket(db).await?.attach(UsersFairing);

    Ok(r)
}

pub async fn seed_db(db: &DbConn) -> Result<(), DbErr> {
    let trx = db.begin().await?;
    let db = &trx;

    make_user("Name 1", "email@email.com", "password")
        .insert(db)
        .await?;

    make_user("Name 2", "second@email.com", "password2")
        .insert(db)
        .await?;

    make_user("Name Deleted", "deleted@email.com", "password_password")
        .deleting()
        .insert(db)
        .await?;

    trx.commit().await?;

    Ok(())
}

fn make_user(
    name: impl ToString,
    email: impl ToString,
    password: impl ToString,
) -> user::ActiveModelEx {
    let params = get_argon_params();
    let argon = Argon2::from(params);

    user::ActiveModel {
        name: Set(name.to_string()),
        email: Set(email.to_string()),
        password_hash: Set(argon
            .hash_password(password.to_string().as_bytes())
            .expect("The configuration is correct")
            .to_string()),
        ..Default::default()
    }
    .into_ex()
    .creating()
}
