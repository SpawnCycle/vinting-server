use std::sync::LazyLock;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    http::uri::Host,
};

/// decides wether to use sha hashing,
/// sha provides less accidental conflicts (because it parses the contents of the file),
/// but in turn gives longer links
pub const USE_SHA: bool = true;

pub const JWT_KEY: &str = "JWT";

pub static JWT_SECRET: LazyLock<Option<&str>> =
    LazyLock::new(|| match dotenvy::var("JWT_SECRET") {
        Ok(var) => Some(var.leak()),
        Err(err) => {
            if cfg!(debug_assertions) {
                log::warn!("No secret key set");
                Some("secret")
            } else {
                log::error!("The JWT_SECRET is not present: {err}");
                None
            }
        }
    });

pub static ADMIN_EMAIL: LazyLock<Option<&str>> =
    LazyLock::new(|| match dotenvy::var("ADMIN_EMAIL") {
        Ok(var) => {
            if dtos::email_string::EMAIL_RX.is_match(&var) {
                Some(var.leak())
            } else {
                log::warn!("ADMIN_EMAIL doesn't match the email regex");
                None
            }
        }
        Err(_err) => {
            log::warn!("No ADMIN_EMAIL set");
            None
        }
    });

pub fn get_jwt_key() -> &'static str {
    JWT_SECRET.expect("Checked inside `LazyProcFairing`")
}

// Will do for now, the proper way would be to check the given host against a whitelist
pub fn construct_host(_host: &Host<'_>) -> String {
    "http://localhost:8000".to_string()
}

pub struct LazyProcFairing;

#[async_trait]
impl Fairing for LazyProcFairing {
    fn info(&self) -> Info {
        Info {
            name: "Lazy Proc Fairing",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        // wish I could `.ok_or(Err(r))`, but `Rocket<_>` doesn't implement `Clone` or `Copy`
        let Some(_) = LazyLock::<_>::force(&JWT_SECRET) else {
            return Err(r);
        };
        let _ = LazyLock::<_>::force(&ADMIN_EMAIL);

        Ok(r)
    }
}
