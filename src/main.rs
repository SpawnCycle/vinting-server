use vinting_server::{
    config, constants::LazyProcFairing, database::DatabaseFairing, dotenv::DotenvFairing,
    file_server::FileServerFairing, responder::CatcherFairing, routes::AllRouteFairing,
};

#[rocket::launch]
fn launch() -> _ {
    config::rocket()
        .attach(DotenvFairing)
        .attach(LazyProcFairing)
        .attach(FileServerFairing)
        .attach(AllRouteFairing)
        .attach(DatabaseFairing)
        .attach(CatcherFairing)
}
