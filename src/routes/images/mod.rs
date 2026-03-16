pub mod post;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    routes,
};

pub struct ImagesFairing;

#[async_trait]
impl Fairing for ImagesFairing {
    fn info(&self) -> Info {
        Info {
            name: "Images Fairing",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let r = r.mount("/api/images/", routes![post::upload]);
        Ok(r)
    }
}
