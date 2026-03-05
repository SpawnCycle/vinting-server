pub mod delete;
pub mod get;
pub mod post;
pub mod put;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    routes,
};

pub struct TagsFairing;

#[async_trait]
impl Fairing for TagsFairing {
    fn info(&self) -> Info {
        Info {
            name: "Tags route fairing",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let r = r.mount(
            "/api/tags",
            routes![post::one, get::all, get::one, delete::one, put::one],
        );

        Ok(r)
    }
}
