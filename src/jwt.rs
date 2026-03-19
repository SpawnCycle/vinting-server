use chrono::{Duration, Local};
use entity::user;
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};
use rocket::{
    Request, async_trait,
    http::{CookieJar, Status},
    request::{FromRequest, Outcome},
};
use sea_orm::{DbConn, DbErr, EntityLoaderTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use services::{service_trait::ServiceTrait, user_service::UserService};
use thiserror::Error;

use crate::constants::{JWT_STR, get_jwt_key};

/// This is the struct used inside the JWT
/// It implements FromRequest, so you can check if a user is signed in with the following:
/// ```no_run
/// use rocket::get;
/// use vinting_server::jwt::JwtClaims;
///
///
/// #[get("/")]
/// fn a_route(
///     user_claims: JwtClaims
/// ) -> &'static str {
///     "You are logged in"
/// }
/// ```
/// But since the db is not accessable from within the request,
/// you still need to verify if the user exists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub exp: i64,
    pub iat: i64,
    pub uid: i32,
}

#[derive(Debug, Clone, Error)]
pub enum JwtError {
    #[error("Could not find a jwt")]
    Missing,
    #[error("JWT cookie could not be parsed: {0}")]
    Malformed(String),
    #[error("The jwt has expired")]
    Expired,
}

impl JwtClaims {
    pub fn new(uid: i32) -> Self {
        let now = Local::now();
        let iat = now.timestamp();
        let exp = (now + Duration::days(30)).timestamp();

        JwtClaims { exp, iat, uid }
    }

    pub fn remove_from(&self, jar: &CookieJar<'_>) {
        jar.remove(JWT_STR);
    }

    pub fn encode(self) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(get_jwt_key().as_ref()),
        )
    }

    pub async fn exists_or_remove(&self, db: &DbConn, jar: &CookieJar<'_>) -> Result<bool, DbErr> {
        let exists = self.exists(db).await?;
        if !exists {
            self.remove_from(jar);
        }
        Ok(exists)
    }

    pub async fn exists(&self, db: &DbConn) -> Result<bool, DbErr> {
        let service = UserService(db);

        service.exists_by_id(self.uid).await
    }

    /// fetches the user from the db using the filters defined in `UserService`
    pub async fn fetch(&self, db: &DbConn) -> Result<Option<user::Model>, DbErr> {
        let service = UserService(db);

        service.get_by_id(self.uid).await
    }

    /// fetches the user from the db without any filters
    pub async fn fetch_always(&self, db: &DbConn) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find_by_id(self.uid).one(db).await
    }

    /// fetches the user from the db using the filters defined in `UserService`
    pub async fn load(
        &self,
        db: &DbConn,
        mut with: impl FnMut(user::EntityLoader) -> user::EntityLoader,
    ) -> Result<Option<user::ModelEx>, DbErr> {
        with(user::Entity::load().filter_by_id(self.uid))
            .filter(UserService::default_filters())
            .one(db)
            .await
    }

    /// fetches the user from the db without any filters
    pub async fn load_always(
        &self,
        db: &DbConn,
        mut with: impl FnMut(user::EntityLoader) -> user::EntityLoader,
    ) -> Result<Option<user::ModelEx>, DbErr> {
        with(user::Entity::load().filter_by_id(self.uid))
            .one(db)
            .await
    }
}

#[async_trait]
impl<'a> FromRequest<'a> for JwtClaims {
    type Error = JwtError;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let jar = request.cookies();
        let Some(jwt) = jar.get(JWT_STR) else {
            return Outcome::Error((Status::Unauthorized, JwtError::Missing));
        };
        let jwt = jwt.to_string();

        let Some((_key, jwt)) = jwt.split_once("=") else {
            return Outcome::Error((
                Status::Unauthorized,
                JwtError::Malformed("The JWT cookie has no value".to_string()),
            ));
        };

        let res = decode::<JwtClaims>(
            jwt,
            &DecodingKey::from_secret(get_jwt_key().as_ref()),
            &Validation::default(),
        )
        .map_err(|err| match err.clone().into_kind() {
            ErrorKind::ExpiredSignature => (Status::Unauthorized, JwtError::Expired),
            _ => (Status::Unauthorized, JwtError::Malformed(err.to_string())),
        });

        let jwt = match res {
            Ok(val) => val,
            Err(err) => return Outcome::Error(err),
        };

        Outcome::Success(jwt.claims)
    }
}
