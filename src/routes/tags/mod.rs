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
pub struct TagsFairing;

#[async_trait]
impl Fairing for TagsFairing {
    fn info(&self) -> Info {
        Info {
            name: "Tags route fairing",
            kind: Kind::Ignite | Kind::Singleton,
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

#[cfg(test)]
mod tests {
    use sea_orm::Database;

    use crate::testing;

    #[tokio::test]
    async fn ignites_successfully() -> anyhow::Result<()> {
        let db = Database::connect("sqlite::memory:").await?;
        let r = testing::rocket(db).await?.attach(super::TagsFairing);

        r.ignite().await?;

        Ok(())
    }
}
