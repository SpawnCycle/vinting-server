#![allow(unused)]

use std::io;

use rocket::Responder;
use sea_orm::DbErr;
use serde::Serialize;

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
        Self::BadRequest(msg.to_string())
    }

    pub fn conflict(msg: impl ToString) -> Self {
        Self::Conflict(msg.to_string())
    }

    pub fn not_found(msg: impl ToString) -> Self {
        Self::NotFound(msg.to_string())
    }

    pub fn server_error(msg: impl ToString) -> Self {
        Self::BadRequest(msg.to_string())
    }

    pub fn unauhorized(msg: impl ToString) -> Self {
        Self::BadRequest(msg.to_string())
    }
}

impl From<DbErr> for Responder {
    fn from(value: DbErr) -> Self {
        Self::ServerError(value.to_string())
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
