mod fairing;
mod getters;

pub use fairing::*;
pub use getters::*;

use std::sync::LazyLock;

/// decides wether to use sha hashing,
/// sha provides less accidental conflicts (because it parses the contents of the file),
/// but in turn it takes longer and gives longer links
pub const USE_SHA: bool = true;

pub const JWT_KEY: &str = "JWT";

pub static JWT_SECRET: LazyLock<Option<&str>> =
    LazyLock::new(|| match dotenvy::var("JWT_SECRET") {
        Ok(var) => Some(var.leak()),
        Err(err) => {
            if cfg!(debug_assertions) {
                log::warn!("No secret key set");
                Some("secret")
            } else {
                log::error!("The JWT_SECRET is not present: {err}");
                None
            }
        }
    });

pub static ADMIN_EMAIL: LazyLock<Option<String>> =
    LazyLock::new(|| match dotenvy::var("ADMIN_EMAIL") {
        Ok(var) => {
            if dtos::email_string::EMAIL_RX.is_match(&var) {
                Some(var)
            } else {
                log::warn!("ADMIN_EMAIL doesn't match the email regex");
                None
            }
        }
        Err(_err) => {
            log::warn!("No ADMIN_EMAIL set");
            None
        }
    });

/// The type in which rocket manages the admin email
#[allow(dead_code)]
pub struct AdminEmail(pub Option<String>);
