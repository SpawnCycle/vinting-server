#![allow(unused)]

use std::io;

use chrono::{DateTime, Utc};
use rocket::{
    Responder,
    serde::json::{self, Json},
};
use sea_orm::DbErr;
use serde::Serialize;

macro_rules! http_err {
    ($name:ident, $code:expr, $msg:ident) => {{
        let err = ErrorMessage::new($code, $msg.to_string());
        let msg = rocket::serde::json::to_string(&err).expect("The serialization shouldn't fail");
        Self::$name(msg)
    }};
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorMessage {
    code: u16,
    timestamp: DateTime<Utc>,
    message: String,
}

impl ErrorMessage {
    pub fn new(code: u16, message: String) -> Self {
        let now = Utc::now();
        Self {
            code,
            message,
            timestamp: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Responder)]
pub enum Responder {
    #[response(status = 500)]
    ServerError(String),
    #[response(status = 409)]
    Conflict(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 403)]
    BadRequest(String),
    #[response(status = 401)]
    Unauhorized(String),
}

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
            Self::server_error(format!("There was an io error: {}", value))
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
