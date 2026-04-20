mod delete;
mod get;
mod post;

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
            kind: Kind::Ignite | Kind::Singleton,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let r = r.mount(
            "/api/images/",
            routes![post::upload, get::all, get::one, delete::one],
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
        let r = testing::rocket(db).await?.attach(super::ImagesFairing);

        r.ignite().await?;

        Ok(())
    }
}
