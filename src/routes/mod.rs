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

mod categories;
mod images;
mod products;
mod tags;
mod users;

use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io,
};

use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    fs::TempFile,
};

use crate::routes::{
    categories::CategoriesFairing, images::ImagesFairing, products::ProductFairing,
    tags::TagsFairing, users::UsersFairing,
};

pub struct AllRouteFairing;

#[async_trait]
impl Fairing for AllRouteFairing {
    fn info(&self) -> Info {
        Info {
            name: "All route fairing",
            kind: Kind::Ignite | Kind::Singleton,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let r = r
            .attach(UsersFairing)
            .attach(CategoriesFairing)
            .attach(TagsFairing)
            .attach(ImagesFairing)
            .attach(ProductFairing);

        Ok(r)
    }
}

pub async fn save_image(image: &mut TempFile<'_>) -> Result<String, io::Error> {
    let mut hasher = DefaultHasher::new();

    // the more random stuff to hash, the better
    image.len().hash(&mut hasher);
    image.path().hash(&mut hasher);
    if let Some(b) = image.name() {
        b.hash(&mut hasher)
    }

    let hash = hasher.finish();
    let out = const_hex::display(hash.to_ne_bytes()).to_string();
    let uri = format!("./img/{out}.png");

    image.move_copy_to(uri.clone()).await?;
    Ok(uri.to_string())
}
