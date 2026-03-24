use rocket::{
    Build, Rocket,
    data::{Limits, ToByteUnit},
};

pub fn rocket() -> Rocket<Build> {
    let figment = rocket::Config::figment().merge(rocket::Config {
        port: 8000,
        limits: Limits::new().limit("file", 5.mebibytes()),
        // After logging is set up, this should be false
        cli_colors: true,
        ..Default::default()
    });

    Rocket::custom(figment)
}
