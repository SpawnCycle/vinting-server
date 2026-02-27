use std::sync::LazyLock;

pub const JWT_STR: &str = "JWT";

pub static JWT_KEY: LazyLock<&str> = LazyLock::new(|| match dotenvy::var("JWT_KEY") {
    Ok(var) => var.leak(),
    #[allow(unused_variables)]
    Err(err) => {
        if cfg!(debug_assertions) {
            "secret"
        } else {
            panic!("The JWT_KEY is not present: {err}");
        }
    }
});
