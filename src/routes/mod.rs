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
mod id_model;
mod images;
mod orders;
mod products;
mod tags;
mod users;

mod fairings;

pub use fairings::*;

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

use crate::constants::USE_SHA;

#[derive(Debug)]
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
            .attach(ProductFairing)
            .attach(OrderFairing);

        log::info!("Successfully added routes");

        Ok(r)
    }
}

/// Returns the uri of the image and wether or not there was a conflict
///
/// # Errors
///
/// Errors if there are any problems while saving the image
pub async fn save_image(image: &mut TempFile<'_>) -> Result<String, io::Error> {
    let hash = if USE_SHA && let Ok(hash) = compute_sha(image).await {
        hash
    } else {
        // the more random stuff to hash, the better
        let mut hasher = DefaultHasher::new();
        image.len().hash(&mut hasher);
        image.path().hash(&mut hasher);
        if let Some(b) = image.name() {
            b.hash(&mut hasher);
        }

        hasher.finish().to_ne_bytes().to_vec()
    };

    let out = const_hex::display(hash).to_string();
    let uri = format!("./img/{out}.png");

    let conflict = try_exists(&uri).await?;
    if conflict {
        log::warn!("There was a conflict during file upload");
    } else {
        image.move_copy_to(uri.clone()).await?;
    }

    Ok(uri.clone())
}

async fn compute_sha(image: &mut TempFile<'_>) -> Result<Vec<u8>, io::Error> {
    const BUF_SIZE: usize = 10 * 1024;

    let mut stream = image.open().await?;
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
    use crate::testing;

    #[tokio::test]
    async fn ignites_successfully() -> anyhow::Result<()> {
        let db = testing::db().await?;
        let r = testing::rocket(db).await?.attach(super::AllRouteFairing);

        r.ignite().await?;

        Ok(())
    }
}
