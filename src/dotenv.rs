use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
};

/// Environment variables that can be used with .env:
///     - JWT_KEY: The secret key for jwt
///     - INMEMORY(only debug): if set launches the db in memory mode
pub struct DotenvFairing;

#[async_trait]
impl Fairing for DotenvFairing {
    fn info(&self) -> Info {
        Info {
            name: "Fairing that imports the .env files",
            kind: Kind::Ignite | Kind::Singleton,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let mut success = false;
        if !cfg!(debug_assertions) {
            success = dotenvy::from_filename(".env.production").is_ok() || success;
        }
        success = dotenvy::dotenv()
            .inspect_err(|err| log::warn!("Couldn't initialize dotenvy: {err}"))
            .is_ok()
            || success;

        if !success && !cfg!(debug_assertions) {
            log::error!("Couldn't read an env file");
            return Err(r);
        }

        Ok(r)
    }
}
