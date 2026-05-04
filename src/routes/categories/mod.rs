mod delete;
mod get;
mod post;
mod put;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    routes,
};

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use sea_orm::Database;

    use crate::testing;

    #[tokio::test]
    async fn ignites_successfully() -> anyhow::Result<()> {
        let db = Database::connect("sqlite::memory:").await?;
        let r = testing::rocket(db).await?.attach(super::CategoriesFairing);

        r.ignite().await?;

        Ok(())
    }
}
