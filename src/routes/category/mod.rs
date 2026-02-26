pub mod get;
pub mod post;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    routes,
};

pub struct CategoryFairing;

#[async_trait]
impl Fairing for CategoryFairing {
    fn info(&self) -> Info {
        Info {
            name: "Category route fairing",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let r = r.mount("/api/category", routes![get::get_all, post::post]);

        Ok(r)
    }
}
