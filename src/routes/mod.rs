//! The routes are mounted via rocket fairing,
//! each route subfolder has its own fairing,
//! which mounts all of the necessary routes
//!
//! Routes file structure:
//! <table>/
//!     mod.rs  (location of the fairing)
//!     get.rs  (all of the routes which accept a GET http request)
//!     post.rs (all of the routes which accept a POST http request)
//!     put.rs  (all of the routes which accept a PUT http request)

pub mod categories;
pub mod tags;
pub mod users;

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
};

use crate::routes::users::UsersFairing;

pub struct AllRouteFairing;

#[async_trait]
impl Fairing for AllRouteFairing {
    fn info(&self) -> Info {
        Info {
            name: "All route fairing",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let r = r.attach(UsersFairing);

        Ok(r)
    }
}
