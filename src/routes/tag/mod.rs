pub mod delete;
pub mod get;
pub mod post;
pub mod put;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    routes,
};

pub struct TagFairing;

#[async_trait]
impl Fairing for TagFairing {
    fn info(&self) -> Info {
        Info {
            name: "Tag route fairing",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let r = r.mount(
            "/api/tags",
            routes![
                get::get_all,
                get::get_single,
                post::post,
                put::put,
                delete::delete
            ],
        );

        Ok(r)
    }
}
