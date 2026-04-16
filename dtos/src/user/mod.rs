pub mod get;
pub mod post;
pub mod put;
pub mod whoami;

use argon2::{Argon2, PasswordHasher};

fn hash_password(pwd: &str) -> String {
    let argon = Argon2::from(crate::get_argon_params());

    argon
        .hash_password(pwd.as_bytes())
        .expect("Hashing should not error if configured properly")
        .to_string()
}
