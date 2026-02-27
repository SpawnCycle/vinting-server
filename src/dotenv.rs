use rocket::{
    Build, Rocket, async_trait,
    fairing::{self, Fairing, Info, Kind},
};

pub struct DotenvFairing;

#[async_trait]
impl Fairing for DotenvFairing {
    fn info(&self) -> Info {
        Info {
            name: "Fairing that imports the .env files",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        let _ =
            dotenvy::dotenv().inspect_err(|err| log::warn!("Couldn't initialize dotenvy: {err}"));

        Ok(r)
    }
}
