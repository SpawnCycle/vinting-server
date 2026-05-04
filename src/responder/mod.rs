mod err_macro;
mod fairing;
mod json_catcher;

pub use fairing::*;
pub use json_catcher::*;

use std::{error::Error, fmt::Display, io};

use chrono::{DateTime, Utc};
use rocket::Responder;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

use crate::http_err;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMessage {
    code: u16,
    timestamp: DateTime<Utc>,
    message: String,
}

impl ErrorMessage {
    pub fn new(code: u16, message: &(impl ToString + ?Sized)) -> Self {
        let now = Utc::now();
        Self {
            code,
            message: message.to_string(),
            timestamp: now,
        }
    }
}

/// This struct, in reality has a serialized `ErrorMessage`
#[derive(Debug, Clone, Serialize, Responder)]
pub enum Responder {
    #[response(status = 500)]
    ServerError(String),
    #[response(status = 409)]
    Conflict(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 401)]
    Unauhorized(String),
    #[response(status = 400)]
    BadRequest(String),
}

impl Display for Responder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Responder::ServerError(msg)
            | Responder::Conflict(msg)
            | Responder::NotFound(msg)
            | Responder::BadRequest(msg)
            | Responder::Unauhorized(msg) => write!(f, "{msg}"),
        }
    }
}

impl Error for Responder {}

// passing by value is more convenient here
#[allow(clippy::needless_pass_by_value)]
impl Responder {
    pub fn bad_request(msg: impl ToString) -> Self {
        http_err!(BadRequest, 403, msg)
    }

    pub fn conflict(msg: impl ToString) -> Self {
        http_err!(Conflict, 409, msg)
    }

    pub fn not_found(msg: impl ToString) -> Self {
        http_err!(NotFound, 404, msg)
    }

    pub fn server_error(msg: impl ToString) -> Self {
        let msg = msg.to_string();
        log::error!("Internal error: {msg}");
        http_err!(ServerError, 500, msg)
    }

    pub fn unauhorized(msg: impl ToString) -> Self {
        http_err!(Unauhorized, 401, msg)
    }
}

impl From<DbErr> for Responder {
    fn from(value: DbErr) -> Self {
        Self::server_error(value)
    }
}

impl From<io::Error> for Responder {
    fn from(value: io::Error) -> Self {
        if cfg!(debug_assertions) {
            Self::server_error(format!("There was an io error: {value}"))
        } else {
            Self::server_error("There was an error while saving the file")
        }
    }
}

impl From<jsonwebtoken::errors::Error> for Responder {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        if cfg!(debug_assertions) {
            Self::server_error(e.to_string())
        } else {
            Self::server_error("There was an error with the jwt")
        }
    }
}
