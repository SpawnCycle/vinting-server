//! TODO:
//! DELETE?

mod get;
mod post;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    routes,
};

pub struct UsersFairing;

#[async_trait]
impl Fairing for UsersFairing {
    fn info(&self) -> Info {
        Info {
            name: "Users route fairing",
            kind: Kind::Ignite | Kind::Singleton,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let r = r.mount(
            "/api/users",
            routes![
                post::signup,
                post::login_form,
                post::login_json,
                post::logout,
                get::whoami,
                get::one,
                get::all,
                get::user_products,
            ],
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
        let r = rocket::build().manage(db).attach(super::UsersFairing);

        r.ignite().await?;

        Ok(())
    }
}
