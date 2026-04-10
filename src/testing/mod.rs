pub mod category;
pub mod tag;

use anyhow::anyhow;
use dtos::{UserPostDto, email_string::EmailString};
use entity::{active_action::ActiveAction, role};
use migrations::MigratorTraitSelf;
use rocket::{
    Build, Phase, Rocket,
    http::{Header, uri::Host},
    local::asynchronous::{Client, LocalRequest},
    uri,
};
use sea_orm::{Database, DbConn, DbErr, IntoActiveModel};

use crate::{config, constants::AdminEmail};

// admin is id 1
// user is id 2
pub async fn db() -> Result<DbConn, DbErr> {
    let db = Database::connect("sqlite::memory:").await?;
    db.get_schema_registry("entity::*").sync(&db).await?;

    Ok(db)
}

pub async fn setup_users(db: &DbConn) -> Result<(), DbErr> {
    migrations::testing::TestMigrator.up(db, None).await?;

    let admin = UserPostDto {
        name: "admin".to_string(),
        email: EmailString::try_from("admin@admin.com".to_string())
            .expect("This is, in fact, an email"),
        password: "password".to_string(),
    };

    let user = UserPostDto {
        name: "user".to_string(),
        email: EmailString::try_from("user@user.com".to_string())
            .expect("This is, in fact, an email"),
        password: "password".to_string(),
    };

    let admin_role = role::Entity::find_by_name("Admin")
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Couldn't find Admin role".to_string()))?
        .into_active_model();
    let user_role = role::Entity::find_by_name("User")
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Couldn't find User role".to_string()))?
        .into_active_model();

    entity::user::ActiveModelEx::from(admin)
        .add_role(admin_role.clone())
        .add_role(user_role.clone())
        .creating()
        .insert(db)
        .await?;
    entity::user::ActiveModelEx::from(user)
        .add_role(user_role.clone())
        .creating()
        .insert(db)
        .await?;

    Ok(())
}

pub async fn rocket(db: DbConn) -> Result<Rocket<Build>, rocket::Error> {
    let r = config::rocket()
        .manage(db)
        .manage(AdminEmail(Some("admin@admin.com".to_string())));

    Ok(r)
}

pub async fn admin_client<P>(r: Rocket<P>) -> anyhow::Result<Client>
where
    P: Phase,
{
    let client = Client::tracked(r).await?;

    {
        let req = json_request(client.post("/api/users/login").body(
            r#"{
            "email": "admin@admin.com",
            "password": "password"
        }"#,
        ));
        let res = req.dispatch().await;
        if res.status().code != 204 {
            return Err(anyhow!(
                "Couldn't login as admin, consider adding the `/api/users/signup` path"
            ));
        }
    }

    Ok(client)
}

pub async fn user_client<P>(r: Rocket<P>) -> anyhow::Result<Client>
where
    P: Phase,
{
    let client = Client::tracked(r).await?;

    {
        let req = json_request(client.post("/api/users/login").body(
            r#"{
            "email": "admin@admin.com",
            "password": "password"
        }"#,
        ));
        let res = req.dispatch().await;
        if res.status().code != 201 {
            return Err(anyhow!(
                "Couldn't login as user, consider adding the `/api/users/signup` path"
            ));
        }
    }

    Ok(client)
}

pub fn json_request(client: LocalRequest) -> LocalRequest {
    let content_type = Header::new("Content-Type", "application/json");
    let mut req = client.header(content_type);
    req.set_host(Host::new(uri!("localhost:8000")));
    req
}
