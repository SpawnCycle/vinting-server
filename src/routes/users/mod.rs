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
                post::login,
                post::logout,
                get::whoami,
                get::one,
                get::all,
                get::jwt_test,
                get::auth_test
            ],
        );

        Ok(r)
    }
}
