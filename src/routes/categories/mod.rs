mod delete;
mod get;
mod post;
mod put;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    routes,
};

pub struct CategoriesFairing;

#[async_trait]
impl Fairing for CategoriesFairing {
    fn info(&self) -> Info {
        Info {
            name: "Categories route fairing",
            kind: Kind::Ignite | Kind::Singleton,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let r = r.mount(
            "/api/categories",
            routes![get::one, get::all, post::one, put::one, delete::one],
        );

        Ok(r)
    }
}
