use rocket::{
    Build, Rocket,
    data::{Limits, ToByteUnit},
};

pub fn rocket() -> Rocket<Build> {
    let figment = rocket::Config::figment().merge(rocket::Config {
        port: 8000,
        limits: Limits::new().limit("file", 5.mebibytes()),
        // when rocket outputs logs, it will output color codes regardless if cli_colors are turned
        // on, which is not very good if you log into a file
        cli_colors: false,
        ..Default::default()
    });

    Rocket::custom(figment)
}
