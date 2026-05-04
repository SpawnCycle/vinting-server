use std::env;

use entity::{active_action::ActiveAction, prelude::*, role};
use migrations::{MigratorTraitSelf, constants::ADMIN_ROLE};
use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
};
use sea_orm::{
    DatabaseConnection, DbConn, DbErr, IntoActiveModel, SqlxError,
    sqlx::{
        ConnectOptions, SqlitePool,
        sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    },
};

use crate::constants::ADMIN_EMAIL;

#[derive(Debug)]
pub struct DatabaseFairing;

#[async_trait]
impl Fairing for DatabaseFairing {
    fn info(&self) -> Info {
        Info {
            name: "Database Fairing",
            kind: Kind::Ignite | Kind::Singleton,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let db = match connect_db().await {
            Err(err) => {
                log::error!("Could not make a database connection: {err}");
                return Err(r);
            }
            Ok(conn) => {
                let db = DatabaseConnection::from(conn);
                if let Err(err) = db.get_schema_registry("entity::*").sync(&db).await {
                    log::error!("Error while syncing db: {err}");
                    return Err(r);
                }
                db
            }
        };

        if let Err(err) = register_admin(&db).await {
            log::error!("Error while registering admin: {err}");
            return Err(r);
        }

        match migrations::Migrator.up(&db, None).await {
            Ok(()) => Ok(r.manage(db)),
            Err(err) => {
                log::error!("There was an error while running migrations: {err}");
                Err(r)
            }
        }
    }
}

async fn register_admin(db: &DbConn) -> Result<(), DbErr> {
    let Some(email) = &*ADMIN_EMAIL else {
        return Ok(());
    };
    let Some(user) = User::find_by_email(email).one(db).await? else {
        return Ok(());
    };
    let role = match Role::find_by_name(ADMIN_ROLE).one(db).await? {
        Some(val) => val.into_active_model().into_ex(),
        None => role::ActiveModelEx::new().set_name(ADMIN_ROLE).creating(),
    };

    let am = user.into_active_model().into_ex().add_role(role);

    let _ = am.save(db).await?;

    log::info!("Successfully initialized db");

    Ok(())
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

    let opts = if cfg!(debug_assertions) {
        opts.log_statements(log::LevelFilter::Info)
    } else {
        opts
    };

    let pool_opts = if use_memory {
        SqlitePoolOptions::new().max_connections(1)
    } else {
        SqlitePoolOptions::new()
    };

    pool_opts.connect_with(opts).await
}
