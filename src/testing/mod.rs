pub mod tag;

use crate::{config, routes::AllRouteFairing};
use rocket::{
    Ignite, Rocket,
    http::{Header, uri::Host},
    local::asynchronous::LocalRequest,
    uri,
};
use sea_orm::{Database, DbConn, DbErr};

pub async fn db() -> Result<DbConn, DbErr> {
    let db = Database::connect("sqlite::memory:").await?;
    db.get_schema_registry("entity::*").sync(&db).await?;
    Ok(db)
}

pub async fn rocket(db: DbConn) -> Result<Rocket<Ignite>, rocket::Error> {
    let r = config::rocket().manage(db).attach(AllRouteFairing);
    let r = r.ignite().await?;

    Ok(r)
}

pub fn json_request(client: LocalRequest) -> LocalRequest {
    let content_type = Header::new("Content-Type", "application/json");
    let mut req = client.header(content_type);
    req.set_host(Host::new(uri!("localhost:8000")));
    req
}
