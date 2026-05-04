use vinting_server::{
    config, constants::LazyProcFairing, database::DatabaseFairing, dotenv::DotenvFairing,
    file_server::FileServerFairing, responder::CatcherFairing, routes::AllRouteFairing,
};

fn build_rocket() -> rocket::Rocket<rocket::Build> {
    let _ = config::logger().expect("Couldn't properly initialize logging");

    config::rocket()
        .attach(DotenvFairing)
        .attach(LazyProcFairing)
        .attach(FileServerFairing)
        .attach(AllRouteFairing)
        .attach(DatabaseFairing)
        .attach(CatcherFairing)
}

//  when using `rocket::launch`, if anything fails it will panic,
//  which will print out the RUST_BACKTRACE message (not ideal)
#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let _ = build_rocket().launch().await?;

    Ok(())
}
