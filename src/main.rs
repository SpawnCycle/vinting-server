use vinting_server::{
    constants::LazyProcFairing, database::DatabaseFairing, dotenv::DotenvFairing,
    file_server::FileServerFairing, routes::AllRouteFairing,
};

#[rocket::launch]
fn launch() -> _ {
    rocket::build()
        .attach(DotenvFairing)
        .attach(LazyProcFairing)
        .attach(FileServerFairing)
        .attach(AllRouteFairing)
        .attach(DatabaseFairing)
}
