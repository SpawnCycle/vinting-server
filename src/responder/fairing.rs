use rocket::{
    Build, Rocket, async_trait, catchers,
    fairing::{self, Fairing, Info, Kind},
};

#[derive(Debug)]
pub struct CatcherFairing;

#[async_trait]
impl Fairing for CatcherFairing {
    fn info(&self) -> Info {
        Info {
            name: "Json Catcher Fairing",
            kind: Kind::Ignite | Kind::Singleton,
        }
    }

    async fn on_ignite(&self, r: Rocket<Build>) -> fairing::Result {
        Ok(r.register("/", catchers![super::json_catcher::default_responder]))
    }
}
