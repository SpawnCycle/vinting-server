use std::sync::LazyLock;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    http::uri::Host,
};

pub const JWT_STR: &str = "JWT";

pub static JWT_KEY: LazyLock<Option<&str>> = LazyLock::new(|| match dotenvy::var("JWT_KEY") {
    Ok(var) => Some(var.leak()),
    Err(err) => {
        if cfg!(debug_assertions) {
            log::warn!("No secret key set");
            Some("secret")
        } else {
            log::error!("The JWT_KEY is not present: {err}");
            None
        }
    }
});

pub fn get_jwt_key() -> &'static str {
    JWT_KEY.expect("Checked inside `LazyProcFairing`")
}

// TODO: Get the value at runtime
pub fn has_tls() -> bool {
    false
}

/// returns `https` if tls is enabled,
/// otherwise returns `http`
pub fn get_protocol() -> &'static str {
    if has_tls() { "https" } else { "http" }
}

// WARN: There's probably a better way of doing this, but I don't know it
pub fn construct_host(host: &Host<'_>) -> String {
    format!("{}://{host}", get_protocol())
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
        let Some(_) = LazyLock::<_>::force(&JWT_KEY) else {
            return Err(r);
        };

        Ok(r)
    }
}
