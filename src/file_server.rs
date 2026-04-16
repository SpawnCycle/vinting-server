use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
    fs::{FileServer, Options},
    routes,
};
use tokio::fs::{create_dir, try_exists};

use crate::routable_file_server::get_root_regardless;

pub struct FileServerFairing;

#[async_trait]
impl Fairing for FileServerFairing {
    fn info(&self) -> Info {
        Info {
            name: "Fairing for static files",
            kind: Kind::Ignite | Kind::Singleton,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let r = try_mount_web(r).await?;
        let r = try_mount_img(r).await?;

        Ok(r)
    }
}

async fn try_mount_web(r: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    match try_exists("./web/").await.unwrap_or(false) {
        true => Ok(r
            .mount("/", FileServer::from("./web"))
            .mount("/", routes![get_root_regardless])),
        false => {
            log::error!("The 'web' directory is not present, nothing to host");
            Err(r)
        }
    }
}

async fn try_mount_img(r: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    if !try_exists("./img").await.unwrap_or(false)
        && let Err(err) = create_dir("./img").await
    {
        log::error!("There was an error while creating `./img`: {err}");
        return Err(r);
    }
    Ok(r.mount("/img", FileServer::new("./img", Options::None).rank(5)))
}
