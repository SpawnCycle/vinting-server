use std::sync::LazyLock;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
};

pub const JWT_STR: &str = "JWT";

pub static JWT_KEY: LazyLock<Option<&str>> = LazyLock::new(|| match dotenvy::var("JWT_KEY") {
    Ok(var) => Some(var.leak()),
    #[allow(unused_variables)]
    Err(err) => {
        if cfg!(debug_assertions) {
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
