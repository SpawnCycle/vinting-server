mod get;
mod post;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    routes,
};

pub struct ProductFairing;

#[async_trait]
impl Fairing for ProductFairing {
    fn info(&self) -> Info {
        Info {
            name: "Product Fairing",
            kind: Kind::Ignite | Kind::Singleton,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let r = r.mount("/api/products", routes![post::one, get::all, get::one]);

        Ok(r)
    }
}
