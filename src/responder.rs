#![allow(unused)]

use rocket::Responder;
use sea_orm::DbErr;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Responder)]
pub enum Responder {
    #[response(status = 500)]
    ServerError(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 403)]
    BadRequest(String),
    #[response(status = 401)]
    Unauhorized(String),
}

impl Responder {
    pub fn bad_request<S: ToString + ?Sized>(msg: &S) -> Self {
        Self::BadRequest(msg.to_string())
    }

    pub fn not_found<S: ToString + ?Sized>(msg: &S) -> Self {
        Self::NotFound(msg.to_string())
    }

    pub fn server_error<S: ToString + ?Sized>(msg: &S) -> Self {
        Self::BadRequest(msg.to_string())
    }

    pub fn unauhorized<S: ToString + ?Sized>(msg: &S) -> Self {
        Self::BadRequest(msg.to_string())
    }
}

impl From<DbErr> for Responder {
    fn from(value: DbErr) -> Self {
        Self::ServerError(value.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for Responder {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        if cfg!(debug_assertions) {
            Self::server_error(&e.to_string())
        } else {
            Self::server_error("There was an error with the jwt")
        }
    }
}
