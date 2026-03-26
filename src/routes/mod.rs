//! The routes are mounted via rocket fairing,
//! each route subfolder has its own fairing,
//! which mounts all of the necessary routes
//!
//! Routes file structure:
//! \<table>/
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
use sha2::Digest;
use tokio::{fs::try_exists, io::AsyncReadExt};

use crate::{
    constants::USE_SHA,
    routes::{
        categories::CategoriesFairing, images::ImagesFairing, products::ProductFairing,
        tags::TagsFairing, users::UsersFairing,
    },
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

/// Returns the uri of the image and wether or not there was a conflict
pub async fn save_image(image: &mut TempFile<'_>) -> Result<String, io::Error> {
    let hash = if USE_SHA && let Ok(hash) = compute_sha(image).await {
        hash
    } else {
        // the more random stuff to hash, the better
        let mut hasher = DefaultHasher::new();
        image.len().hash(&mut hasher);
        image.path().hash(&mut hasher);
        if let Some(b) = image.name() {
            b.hash(&mut hasher)
        }

        hasher.finish().to_ne_bytes().to_vec()
    };

    let out = const_hex::display(hash).to_string();
    let uri = format!("./img/{out}.png");

    let conflict = try_exists(&uri).await?;
    if !conflict {
        image.move_copy_to(uri.clone()).await?;
    } else {
        log::warn!("There was a conflict during file upload");
    }

    Ok(uri.to_string())
}

async fn compute_sha(image: &mut TempFile<'_>) -> Result<Vec<u8>, io::Error> {
    let mut stream = image.open().await?;
    const BUF_SIZE: usize = 10 * 1024;
    let mut buf = [0u8; BUF_SIZE];
    let mut sha = sha2::Sha256::new();
    while stream.read(&mut buf).await? != 0 {
        sha.update(buf);
        // reset buffer
        buf.fill(0);
    }
    let hash = sha.finalize();

    Ok(hash.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use sea_orm::Database;

    #[tokio::test]
    async fn ignites_successfully() -> anyhow::Result<()> {
        let db = Database::connect("sqlite::memory:").await?;
        let r = rocket::build().manage(db).attach(super::AllRouteFairing);

        r.ignite().await?;

        Ok(())
    }
}
