use std::sync::LazyLock;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
};

use super::{ADMIN_EMAIL, JWT_SECRET};

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
