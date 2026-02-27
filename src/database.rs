use std::env;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
};
use sea_orm::{
    DatabaseConnection, SqlxError,
    sqlx::{
        SqlitePool,
        sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    },
};

pub struct DatabaseFairing;

#[async_trait]
impl Fairing for DatabaseFairing {
    fn info(&self) -> Info {
        Info {
            name: "Database Fairing",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        match connect_db().await {
            Err(err) => {
                log::error!("Could not make a database connection: {err}");
                Err(r)
            }
            Ok(conn) => {
                let db = DatabaseConnection::from(conn);
                if let Err(err) = db.get_schema_registry("entity::*").sync(&db).await {
                    log::error!("Error while syncing db: {err}");
                    return Err(r);
                }
                Ok(r.manage(db))
            }
        }
    }
}

async fn connect_db() -> Result<SqlitePool, SqlxError> {
    let use_memory = cfg!(debug_assertions) && env::var("INMEMORY").is_ok();
    let opts = if use_memory {
        SqliteConnectOptions::new()
            .in_memory(true)
            .shared_cache(true)
            .create_if_missing(true)
    } else {
        SqliteConnectOptions::new()
            .filename("./vinting.db") // TODO: maybe change in the future?
            .create_if_missing(true)
    };

    let pool_opts = if use_memory {
        SqlitePoolOptions::new().max_connections(1)
    } else {
        SqlitePoolOptions::new()
    };

    pool_opts.connect_with(opts).await
}
