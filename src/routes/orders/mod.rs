mod delete;
mod get;
mod put;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    routes,
};

pub struct OrderFairing;

#[async_trait]
impl Fairing for OrderFairing {
    fn info(&self) -> Info {
        Info {
            name: "Order Fairing",
            kind: Kind::Ignite | Kind::Singleton,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let r = r.mount(
            "/api/orders",
            routes![get::one, get::from_user, get::all, delete::one, put::one],
        );

        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::Database;

    #[tokio::test]
    async fn ignites_successfully() -> anyhow::Result<()> {
        let db = Database::connect("sqlite::memory:").await?;
        let r = rocket::build().manage(db).attach(super::OrderFairing);

        r.ignite().await?;

        Ok(())
    }
}
